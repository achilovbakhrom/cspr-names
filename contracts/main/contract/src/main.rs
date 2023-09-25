#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod config_db;
mod name_contract_hash_db;
mod names_validator;
mod utils;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use common_lib::{
    constants::common_keys::{
        ARG_DATABASE_DOMAIN_NAME, ARG_DATABASE_RESOLVER, ARG_MAIN_AUTHORITY,
        ARG_MAIN_CUSTOMER_PURSE, ARG_MAIN_DOMAIN, ARG_MAIN_DOMAIN_PAGE, ARG_MAIN_DURATION,
        ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH, ARG_MAIN_REGISTER_AMOUNT, ARG_MAIN_RESOLVER_ADDRESS,
        ARG_MAIN_SUBDOMAIN, ARG_NFT_METADATA, ARG_NFT_TOKEN_OWNER, ARG_PO_PRICE_TYPE_CHARS_COUNT,
        ARG_REGISTRY_CONTRACT_HASH, ARG_REGISTRY_OPERATOR_TYPE, ENDPOINT_DATABASE_GET_DOMAIN,
        ENDPOINT_DATABASE_SAVE_DOMAIN_NAME, ENDPOINT_PO_GET_PRICE, ENTRYPOINT_MAIN_ADD_AUTHORITY,
        ENTRYPOINT_MAIN_EXTEND, ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN,
        ENTRYPOINT_MAIN_REGISTER_DOMAIN, ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN,
        ENTRYPOINT_MAIN_REMOVE_AUTHORITY, ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN,
        ENTRYPOINT_MAIN_RESOLVE_DOMAIN, ENTRYPOINT_MAIN_SET_AUTHORITIES_CONTRACT_HASH,
        ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN,
        ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN, KEY_DATABASE_DICTIONARY_DOMAIN,
        KEY_DATABASE_DICTIONARY_DOMAIN_LIST, KEY_DATABASE_DICTIONARY_SUBDOMAIN,
        KEY_MAIN_AUTHORITIES, KEY_MAIN_AUTHORITIES_CONTRACT_HASH, KEY_MAIN_CONTRACT_ACCESS_UREF,
        KEY_MAIN_CONTRACT_HASH, KEY_MAIN_CONTRACT_PACKAGE_NAME, KEY_MAIN_CONTRACT_VERSION,
        KEY_MAIN_DICTIONARY_DOMAIN_METADATA, KEY_MAIN_MAINTAINER, KEY_MAIN_MAINTAINER_PURSE,
        KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH, MAX_PAGE_SIZE, MAX_SUBDOMAIN_COUNT,
    },
    enums::{
        caller_verification_type::CallerVerificationType, contracts_enum::ContractKind,
        domain_name_actual_state::DomainNameActualState, main_contract::Error,
    },
    errors::{CommonError, MainContractErrors},
    models::{DomainName, LocalMetadata, SubdomainName},
    utils::{
        domain_name::{
            calculate_domain_name_end_date, get_end_time_actual_state, is_domain_name_valid,
            is_extension_duration_correct, is_sub_domain_name_valid, year_to_millis,
        },
        registry::{has_authority, is_maintainer},
        storage::{
            get_dictionary_value_from_key, get_stored_value_from_key, store_value_for_key,
            upsert_dictionary_value_from_key,
        },
    },
};

use casper_contract::contract_api::runtime::call_contract;
use casper_contract::contract_api::system::{
    create_purse, get_purse_balance, transfer_from_purse_to_purse,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    account::AccountHash, contracts::NamedKeys, runtime_args, CLType, CLTyped, CLValue,
    ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
    RuntimeArgs, URef, U512,
};
use common_lib::constants::common_keys::{
    ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER, ENDPOINT_NFT_MINT,
};

use crate::config_db::ConfigDb;
use crate::name_contract_hash_db::NameContractHashDb;
use crate::names_validator::NamesValidator;
use common_lib::utils::registry::get_contract_hash_from_authority_contract;
use common_lib::utils::response::{response_error, response_success};

/**
 * Registers a new domain name
 * Steps:
 *
 * 1. Check validity of the domain name
 * 2. Check whether the name is already exists (check name_contract_hash_db mapping and after database contract)
 * 3. Get Price from PriceOracle and check it with payment amount
 * 4. Check registering duration
 * 5. Payment process
 * 6. Mint new NFT for the domain name
 * 7. Store domain in database
 * 8. Map name with database contract hash in name_contract_hash_db
 * 9. Retrieve domain name object
 *
 */
#[no_mangle]
pub extern "C" fn register_domain() {
    let domain: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);
    let duration: u8 = runtime::get_named_arg(ARG_MAIN_DURATION);
    let resolver_address: AccountHash = runtime::get_named_arg(ARG_MAIN_RESOLVER_ADDRESS);
    let amount: U512 = runtime::get_named_arg(ARG_MAIN_REGISTER_AMOUNT);
    let customer_purse: URef = runtime::get_named_arg(ARG_MAIN_CUSTOMER_PURSE);

    let extensions = ConfigDb::instance().get_allowed_extensions();
    if extensions.is_none() {
        return response_error(MainContractErrors::AllowedExtensionsNotConfigured);
    }

    let maintainer: AccountHash = match get_stored_value_from_key(KEY_MAIN_MAINTAINER) {
        Some(res) => res,
        None => {
            return response_error(MainContractErrors::MaintainerNotConfigured);
        }
    };

    let config_db_instance = ConfigDb::instance();

    let registry_hash = config_db_instance
        .get_registry_contract_hash()
        .unwrap_or_revert_with(MainContractErrors::RegistryContractHashNotConfigured);

    let operators_key: Vec<Key> = runtime::call_contract(
        registry_hash,
        "get_operators_for_contract_hash",
        runtime_args! {
            ARG_REGISTRY_CONTRACT_HASH => registry_hash,
            ARG_REGISTRY_OPERATOR_TYPE => CallerVerificationType::OnlyAccountHash,
        },
    );

    let mut operators = operators_key
        .iter()
        .map(|item| {
            item.into_account()
                .unwrap_or_revert_with(CommonError::FailedToConvertToAccountHash)
        })
        .collect::<Vec<AccountHash>>();
    operators.push(maintainer);

    // Validation
    let validator = NamesValidator::instance(extensions.unwrap(), true);

    let model = match validator.validate_name(domain.to_string()) {
        Ok(res) => res,
        Err(e) => {
            return response_error(e);
        }
    };

    if duration > 3 {
        runtime::revert(MainContractErrors::InvalidDuration);
    }

    // Checking for existence
    let db_contract_hash =
        NameContractHashDb::instance().get_contract_hash_for_domain_name(&domain);

    if let Some(hash) = db_contract_hash {
        let store_domain_optional: Option<DomainName> = call_contract(
            hash,
            ENDPOINT_DATABASE_GET_DOMAIN,
            runtime_args! {
                ARG_DATABASE_DOMAIN_NAME => domain.to_string()
            },
        );

        if let Some(store_domain) = store_domain_optional {
            let actual_state = get_end_time_actual_state(Some(store_domain.end_time));
            match actual_state {
                DomainNameActualState::Busy => {
                    runtime::revert(MainContractErrors::DomainNameIsBusy);
                }
                DomainNameActualState::GracePeriod => {
                    runtime::revert(MainContractErrors::DomainNameIsInGracePeriod);
                }
                DomainNameActualState::Available => {}
            }
        }
    }

    // Checking with price oracle
    let authorities_hash: Option<ContractHash> =
        get_stored_value_from_key(KEY_MAIN_AUTHORITIES_CONTRACT_HASH);
    if authorities_hash.is_none() {
        return response_error(MainContractErrors::AuthoritiesContractHashNotConfigured);
    }

    let chars_count = model.get_name_len();

    let price_oracle_contract_hash: ContractHash = match get_contract_hash_from_authority_contract(
        authorities_hash.unwrap(),
        ContractKind::PriceOracle,
        None,
    ) {
        Ok(res) => match res {
            Some(res) => res,
            None => {
                return response_error(CommonError::NoContractHashWasFoundInAuthoritiesContract);
            }
        },
        Err(e) => {
            return response_error(e);
        }
    };

    let price: U512 = runtime::call_contract(
        price_oracle_contract_hash,
        ENDPOINT_PO_GET_PRICE,
        runtime_args! {
            ARG_PO_PRICE_TYPE_CHARS_COUNT => chars_count as u64
        },
    );

    // Checking for duration
    if U512::from(duration) * price != amount {
        runtime::revert(MainContractErrors::PriceDiscrepancy);
    }

    // Payment process
    let purse: URef = get_stored_value_from_key(KEY_MAIN_MAINTAINER_PURSE)
        .unwrap_or_revert_with(MainContractErrors::MaintainerPurseNotConfigured);

    let balance = get_purse_balance(customer_purse).unwrap_or_revert();

    if balance < amount {
        return response_error(MainContractErrors::InsufficientCustomerBalance);
    }
    transfer_from_purse_to_purse(customer_purse, purse, amount, None).unwrap_or_revert();

    // Mint NFT
    let nft_contract_hash = match get_contract_hash_from_authority_contract(
        authorities_hash.unwrap(),
        ContractKind::NFT,
        None,
    ) {
        Ok(res) => match res {
            Some(res) => res,
            None => {
                return response_error(CommonError::NoContractHashWasFoundInAuthoritiesContract);
            }
        },
        Err(e) => {
            return response_error(e);
        }
    };

    let token_id = base16::encode_lower(&runtime::blake2b(&domain));

    runtime::call_contract::<()>(
        nft_contract_hash,
        ENDPOINT_NFT_MINT,
        runtime_args! {
            ARG_NFT_TOKEN_OWNER => runtime::get_caller(),
            ARG_NFT_METADATA => token_id,
        },
    );

    // Save to database
    let end_time = calculate_domain_name_end_date(duration);
    let caller = runtime::get_caller();

    let db_contract_hash = match get_contract_hash_from_authority_contract(
        authorities_hash.unwrap(),
        ContractKind::Database,
        Some(model.extension),
    ) {
        Ok(hash) => match hash {
            Some(res) => res,
            None => {
                return response_error(MainContractErrors::DatabaseFulfilledOrNotConfigured);
            }
        },
        Err(e) => {
            return response_error(e);
        }
    };

    let saving_domain_name = DomainName {
        end_time,
        name: domain.clone(),
        token_id: token_id.to_string(),
        owner: caller,
        resolver: resolver_address,
    };

    runtime::call_contract::<()>(
        db_contract_hash,
        ENDPOINT_DATABASE_SAVE_DOMAIN_NAME,
        runtime_args! {
            ARG_DATABASE_DOMAIN_NAME => saving_domain_name.clone()
        },
    );
    // Save to name_contract_hash_map
    NameContractHashDb::instance().set_contract_hash_for_domain_name(&domain, db_contract_hash);

    response_success(
        saving_domain_name,
        "Error while converting value to CL_Value",
    );
}

/**
 * Searches by domain name, if domain name is found, returns resolver address of the entity
 * otherwise returns MainContractErrors::DomainNotExists error
 *
 * Steps:
 *
 * 1. Validate domain
 * 2. Find domain
 * 3. If domain name is expired, or it is not found, return corresponding error
 * 4. If domain name is found return its resolver address
 */
#[no_mangle]
pub extern "C" fn resolve_domain() {
    let domain_name: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);

    if !is_domain_name_valid(&domain_name) {
        runtime::revert(Error::InvalidDomainName);
    }

    let db_contract_hash =
        match NameContractHashDb::instance().get_contract_hash_for_domain_name(&domain_name) {
            Some(res) => res,
            None => {
                return response_error(MainContractErrors::DomainNotExists);
            }
        };

    let db_domain_name = call_contract::<Option<DomainName>>(
        db_contract_hash,
        ENDPOINT_DATABASE_GET_DOMAIN,
        runtime_args! {
            ARG_DATABASE_DOMAIN_NAME => domain_name
        },
    );

    if let Some(domain) = db_domain_name {
        match get_end_time_actual_state(Some(domain.end_time)) {
            DomainNameActualState::Available => {
                runtime::revert(MainContractErrors::DomainNotExists)
            }
            DomainNameActualState::GracePeriod => {
                runtime::revert(MainContractErrors::DomainNameIsInGracePeriod)
            }
            DomainNameActualState::Busy => {
                response_success(
                    domain.resolver,
                    "Error while create cl_value from resolver address",
                );
            }
        }
    } else {
        runtime::revert(MainContractErrors::DomainNotExists)
    }
}

/**
 * Changes resolver address of the domain name
 *
 * Steps:
 *
 * 1. Validate and Find entity by domain name
 * 2. Update Resolver address of the entity
 * 3. Save domain name entity with new resolver address
 */
#[no_mangle]
pub extern "C" fn set_resolver_address_for_domain() {
    let domain_name: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);
    let resolver_address: AccountHash = runtime::get_named_arg(ARG_MAIN_RESOLVER_ADDRESS);
    if !is_domain_name_valid(&domain_name) {
        runtime::revert(Error::InvalidDomainName);
    }

    let db_contract_hash =
        match NameContractHashDb::instance().get_contract_hash_for_domain_name(&domain_name) {
            Some(res) => res,
            None => {
                return response_error(MainContractErrors::DomainNotExists);
            }
        };

    let db_domain_name = call_contract::<Option<DomainName>>(
        db_contract_hash.clone(),
        ENDPOINT_DATABASE_GET_DOMAIN,
        runtime_args! {
            ARG_DATABASE_DOMAIN_NAME => domain_name
        },
    );

    if let Some(domain) = db_domain_name {
        match get_end_time_actual_state(Some(domain.end_time)) {
            DomainNameActualState::Available => {
                runtime::revert(MainContractErrors::DomainNotExists)
            }
            DomainNameActualState::GracePeriod => {
                runtime::revert(MainContractErrors::DomainNameIsInGracePeriod)
            }
            DomainNameActualState::Busy => {
                if domain.owner == runtime::get_caller() {
                    call_contract::<()>(
                        db_contract_hash,
                        ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER,
                        runtime_args! {
                            ARG_DATABASE_DOMAIN_NAME => domain_name,
                            ARG_DATABASE_RESOLVER => resolver_address
                        },
                    )
                }
            }
        }
    } else {
        runtime::revert(MainContractErrors::DomainNotExists)
    }
}

/**
 * Registers a new subdomain for a domain name
 *
 * Steps:
 * 1. Check validity of the sub domain name
 * 2. Get domain of the subdomain
 * 3. Check wether domain exists or not, if not return corresponding error
 * 4. Get metadata for subdomains
 * 5. Store subdomain
 * 6. Update metadata and save it
 */
#[no_mangle]
pub extern "C" fn register_sub_domain() {
    let subdomain_name: String = runtime::get_named_arg(ARG_MAIN_SUBDOMAIN);
    let resolver_address: AccountHash = runtime::get_named_arg(ARG_MAIN_RESOLVER_ADDRESS);
    let caller = runtime::get_caller();

    let (is_valid, domain_name) = is_sub_domain_name_valid(&subdomain_name);

    if !is_valid {
        runtime::revert(MainContractErrors::InvalidSubdomain);
    }

    if let Some(domain) = domain_name {
        let found_domain: Option<DomainName> =
            get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);
        if let Some(dm) = found_domain {
            if dm.owner == caller {
                let subdomains: Option<Vec<SubdomainName>> =
                    get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain);
                if let Some(mut sd) = subdomains {
                    if sd.len() < MAX_SUBDOMAIN_COUNT.into() {
                        sd.push(SubdomainName {
                            name: subdomain_name,
                            resolver: resolver_address,
                        });
                        upsert_dictionary_value_from_key(
                            KEY_DATABASE_DICTIONARY_SUBDOMAIN,
                            &domain,
                            sd,
                        );
                    } else {
                        runtime::revert(MainContractErrors::SubdomainMaxCountExceeded);
                    }
                } else {
                    upsert_dictionary_value_from_key(
                        KEY_DATABASE_DICTIONARY_SUBDOMAIN,
                        &domain,
                        vec![SubdomainName {
                            name: subdomain_name,
                            resolver: resolver_address,
                        }],
                    );
                }
            } else {
                runtime::revert(MainContractErrors::InvalidOwner);
            }
        } else {
            runtime::revert(MainContractErrors::DomainNotExists);
        }
    } else {
        runtime::revert(MainContractErrors::SubdomainParseError);
    }
}

#[no_mangle]
pub extern "C" fn remove_subdomain() {
    let subdomain_name: String = runtime::get_named_arg(ARG_MAIN_SUBDOMAIN);
    let caller = runtime::get_caller();

    let (is_valid, domain_name) = is_sub_domain_name_valid(&subdomain_name);

    if !is_valid {
        runtime::revert(MainContractErrors::InvalidSubdomain);
    }
    if let Some(domain) = domain_name {
        let found_domain: Option<DomainName> =
            get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);

        if let Some(dm) = found_domain {
            if dm.owner == caller {
                let subdomains: Option<Vec<SubdomainName>> =
                    get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain);
                if let Some(mut sd) = subdomains {
                    let pos = sd
                        .iter()
                        .position(|x| x.name == subdomain_name)
                        .unwrap_or_revert_with(MainContractErrors::SubdomainNotExists);
                    sd.remove(pos);
                    upsert_dictionary_value_from_key(
                        KEY_DATABASE_DICTIONARY_SUBDOMAIN,
                        &domain,
                        sd,
                    );
                }
            } else {
                runtime::revert(MainContractErrors::InvalidOwner);
            }
        } else {
            runtime::revert(MainContractErrors::DomainNotExists);
        }
    }
}

/**
 *
 * 1. Check validity of the sub domain name
 * 2. Get Price from PriceOracle and check payment amount
 * 3. Store data
 *
 */
#[no_mangle]
pub extern "C" fn set_resolver_address_for_subdomain() {
    let subdomain_name: String = runtime::get_named_arg(ARG_MAIN_SUBDOMAIN);
    let resolver_address: AccountHash = runtime::get_named_arg(ARG_MAIN_RESOLVER_ADDRESS);
    let caller = runtime::get_caller();

    let (is_valid, domain_name) = is_sub_domain_name_valid(&subdomain_name);

    if !is_valid {
        runtime::revert(MainContractErrors::InvalidSubdomain);
    }
    if let Some(domain) = domain_name {
        let found_domain: Option<DomainName> =
            get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);

        if let Some(dm) = found_domain {
            if dm.owner == caller {
                let subdomains: Option<Vec<SubdomainName>> =
                    get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain);
                if let Some(sd) = subdomains {
                    let mapped = sd
                        .iter()
                        .map(|x| {
                            if x.name == subdomain_name {
                                return SubdomainName {
                                    name: x.name.to_string(),
                                    resolver: resolver_address,
                                };
                            }
                            (*x).clone()
                        })
                        .collect::<Vec<SubdomainName>>();
                    upsert_dictionary_value_from_key(
                        KEY_DATABASE_DICTIONARY_SUBDOMAIN,
                        &domain,
                        mapped,
                    );
                }
            } else {
                runtime::revert(MainContractErrors::InvalidOwner);
            }
        } else {
            runtime::revert(MainContractErrors::DomainNotExists);
        }
    }
}

#[no_mangle]
pub extern "C" fn get_sudomains_for_domain() {
    let domain_name: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);
    if is_domain_name_valid(&domain_name) {
        let subdomains: Option<Vec<SubdomainName>> =
            get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain_name);
        let result =
            CLValue::from_t(subdomains).expect("Error while converting subdomains to cl_value");
        runtime::ret(result);
    } else {
        runtime::revert(MainContractErrors::InvalidName);
    }
}

#[no_mangle]
pub extern "C" fn get_domain_list() {
    let page: u8 = runtime::get_named_arg(ARG_MAIN_DOMAIN_PAGE);
    let domains: Option<Vec<String>> = get_dictionary_value_from_key(
        KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
        &format!("{}:{}", page, MAX_PAGE_SIZE),
    );
    if let Some(list) = domains {
        let result = CLValue::from_t(list).expect("error while converting domain list to cl_value");
        runtime::ret(result);
    } else {
        let result = CLValue::from_t::<Vec<String>>(vec![])
            .expect("error while converting domain list to cl_value");
        runtime::ret(result);
    }
}

#[no_mangle]
pub extern "C" fn add_authority() {
    let caller = runtime::get_caller();
    let maintainer = get_stored_value_from_key::<AccountHash>(KEY_MAIN_MAINTAINER)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    if caller != maintainer {
        runtime::revert(MainContractErrors::OnlyMaintainerHasAccess);
    }

    let authority: AccountHash = runtime::get_named_arg(ARG_MAIN_AUTHORITY);

    if authority == maintainer {
        runtime::revert(MainContractErrors::CannotAddMaintainer);
    }
    let has_access = is_maintainer_or_has_authority(&authority);
    if has_access {
        runtime::revert(MainContractErrors::AuthorityHasAlreadyTaken);
    }
    let mut authorities = get_stored_value_from_key::<Vec<AccountHash>>(KEY_MAIN_AUTHORITIES)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    authorities.push(authority);
    store_value_for_key(KEY_MAIN_AUTHORITIES, authorities);
}

#[no_mangle]
pub extern "C" fn remove_authority() {
    let caller = runtime::get_caller();
    let maintainer = get_stored_value_from_key::<AccountHash>(KEY_MAIN_MAINTAINER)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    if caller != maintainer {
        runtime::revert(MainContractErrors::OnlyMaintainerHasAccess);
    }

    let authority: AccountHash = runtime::get_named_arg(ARG_MAIN_AUTHORITY);

    if authority == maintainer {
        runtime::revert(MainContractErrors::CannotRemoveMaintainer);
    }
    let has_access = is_maintainer_or_has_authority(&authority);
    if !has_access {
        runtime::revert(MainContractErrors::UserHasNoAccess);
    }
    let mut authorities = get_stored_value_from_key::<Vec<AccountHash>>(KEY_MAIN_AUTHORITIES)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    let index = authorities
        .iter()
        .position(|item| item == &authority)
        .unwrap();
    authorities.remove(index);
    store_value_for_key(KEY_MAIN_AUTHORITIES, authorities);
}

#[no_mangle]
pub extern "C" fn extend() {
    let domain: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);
    let duration: u8 = runtime::get_named_arg(ARG_MAIN_DURATION);

    if !is_domain_name_valid(&domain) {
        runtime::revert(MainContractErrors::InvalidName);
    }

    let store_domain_optional: Option<DomainName> =
        get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);
    if let Some(mut domain) = store_domain_optional {
        let result = is_extension_duration_correct(domain.end_time, duration.into());

        if result {
            let binding = year_to_millis(duration);
            domain.end_time += binding;
            upsert_dictionary_value_from_key(
                KEY_DATABASE_DICTIONARY_DOMAIN,
                &domain.name,
                domain.clone(),
            );
        } else {
            runtime::revert(MainContractErrors::InvalidDuration)
        }
    } else {
        runtime::revert(MainContractErrors::DomainNotExists)
    }
}

/**
 * Not in MVP
 */
#[no_mangle]
pub extern "C" fn set_data() {}

/**
 * Not in MVP
 */
#[no_mangle]
pub extern "C" fn get_data() {}

#[no_mangle]
pub extern "C" fn set_authorities_contract_hash() {
    let authorities_contract_hash: ContractHash =
        runtime::get_named_arg(ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH);
    store_value_for_key(
        KEY_MAIN_AUTHORITIES_CONTRACT_HASH,
        authorities_contract_hash,
    );
}

#[no_mangle]
pub extern "C" fn init() {
    storage::new_dictionary(KEY_DATABASE_DICTIONARY_DOMAIN).unwrap_or_revert();
    storage::new_dictionary(KEY_DATABASE_DICTIONARY_SUBDOMAIN).unwrap_or_revert();
    storage::new_dictionary(KEY_MAIN_DICTIONARY_DOMAIN_METADATA).unwrap_or_revert();
    storage::new_dictionary(KEY_DATABASE_DICTIONARY_DOMAIN_LIST).unwrap_or_revert();
    upsert_dictionary_value_from_key::<Vec<String>>(
        KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
        "0",
        vec![],
    );
    let metadata_uref = storage::new_uref(LocalMetadata {
        total_count: 0,
        page: 0,
    });
    runtime::put_key(
        KEY_MAIN_DICTIONARY_DOMAIN_METADATA,
        casper_types::Key::URef(metadata_uref),
    );
}

/**
 * Endpoints:
 * 1. register_domain - bakhrom.cspr
 * 2. resolve_domain - bakhrom.cspr -> 0x123123
 * 3. set_resolver_address_for_domain bakhrom.cspr -> 0x1234234
 * 4. register_sub_domain_name -> sub.bakhrom.cspr -> address1
 * 5. remove_subdomain ->
 * 6. set_resolver_address_for_subdomain -> sub.bakhrom.cspr -> address1
 * 7. get_sudomains_for_domain
 * 8. get_domain_list
 * 9. add authority
 * 10. remove authority
 * 11. set_data
 * 12. get_data
 */
#[no_mangle]
pub extern "C" fn call() {
    let mut entrypoints = EntryPoints::new();

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_SET_AUTHORITIES_CONTRACT_HASH,
        vec![Parameter::new(
            ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH,
            ContractHash::cl_type(),
        )],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        "init",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_REGISTER_DOMAIN,
        vec![
            Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
            Parameter::new(ARG_MAIN_DURATION, u8::cl_type()),
            Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_RESOLVE_DOMAIN,
        vec![Parameter::new(ARG_MAIN_DOMAIN, String::cl_type())],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN,
        vec![
            Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
            Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_RESOLVE_DOMAIN,
        vec![
            Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
            Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN,
        vec![
            Parameter::new(ARG_MAIN_SUBDOMAIN, String::cl_type()),
            Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN,
        vec![Parameter::new(ARG_MAIN_SUBDOMAIN, String::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN,
        vec![
            Parameter::new(ARG_MAIN_SUBDOMAIN, String::cl_type()),
            Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN,
        vec![Parameter::new(ARG_MAIN_DOMAIN, String::cl_type())],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_ADD_AUTHORITY,
        vec![Parameter::new(ARG_MAIN_AUTHORITY, AccountHash::cl_type())],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_REMOVE_AUTHORITY,
        vec![Parameter::new(ARG_MAIN_AUTHORITY, AccountHash::cl_type())],
        CLType::Any,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_ADD_AUTHORITY,
        vec![Parameter::new(ARG_MAIN_DOMAIN_PAGE, u8::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENTRYPOINT_MAIN_EXTEND,
        vec![
            Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
            Parameter::new(ARG_MAIN_DURATION, u8::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut main_named_keys = NamedKeys::new();

    let maintainer_uref = storage::new_uref(runtime::get_caller());
    main_named_keys.insert(KEY_MAIN_MAINTAINER.to_string(), maintainer_uref.into());

    let authorities_uref = storage::new_uref(Vec::<AccountHash>::new());
    main_named_keys.insert(KEY_MAIN_AUTHORITIES.to_string(), authorities_uref.into());

    let price_oracle_contract_hash: ContractHash =
        runtime::get_named_arg(ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH);
    let price_oracle_contract_hash_uref = storage::new_uref(price_oracle_contract_hash);
    main_named_keys.insert(
        KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH.to_string(),
        price_oracle_contract_hash_uref.into(),
    );

    let purse = create_purse();
    main_named_keys.insert(KEY_MAIN_MAINTAINER_PURSE.to_string(), purse.into());

    let (contract_hash, version) = storage::new_contract(
        entrypoints,
        Some(main_named_keys),
        Some(KEY_MAIN_CONTRACT_PACKAGE_NAME.to_string()),
        Some(KEY_MAIN_CONTRACT_ACCESS_UREF.to_string()),
    );

    let contract_hash_uref = storage::new_uref(contract_hash);
    runtime::put_key(KEY_MAIN_CONTRACT_HASH, contract_hash_uref.into());

    let contract_version_uref = storage::new_uref(version);
    runtime::put_key(KEY_MAIN_CONTRACT_VERSION, contract_version_uref.into());
}

fn is_maintainer_or_has_authority(account: &AccountHash) -> bool {
    let caller_has_authority = has_authority(KEY_MAIN_AUTHORITIES, account);
    let is_maintainer = is_maintainer(KEY_MAIN_MAINTAINER, account);

    is_maintainer || caller_has_authority
}

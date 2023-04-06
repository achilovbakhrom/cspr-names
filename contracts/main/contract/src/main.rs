#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::hash::Hash;

use alloc::{
    vec,
    vec::Vec,
    string::{String, ToString}, format,
};

use common_lib::{
    utils::{
        domain_name:: {
            is_domain_name_valid,
            namehash_label,
            calculate_domain_name_end_date, 
            is_sub_domain_name_valid,
            get_end_time_actual_state, get_domain_name_chars_count, is_extension_duration_correct, year_to_millis,
        },
        storage::{
            get_dictionary_value_from_key,
            upsert_dictionary_value_from_key,
            get_stored_value_from_key,
            store_value_for_key
        }, authority::{has_authority, is_maintainer},
    },
    models::{
        DomainName,
        SubdomainName, LocalMetadata,
    },
    enums::{main_contract::Error, domain_name_actual_state::DomainNameActualState},
    constants::{
        MAX_PAGE_SIZE,
        MAX_SUBDOMAIN_COUNT,
        ENDPOINT_PO_GET_PRICE,
        KEY_MAIN_MAINTAINER,
        KEY_MAIN_AUTHORITIES,
        KEY_MAIN_CONTRACT_PACKAGE_NAME,
        KEY_MAIN_CONTRACT_ACCESS_UREF,
        KEY_MAIN_CONTRACT_HASH,
        KEY_MAIN_CONTRACT_VERSION,
        ARG_MAIN_REGISTER_AMOUNT,
        KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH,
        ARG_MAIN_AUTHORITY,
        ENTRYPOINT_MAIN_ADD_AUTHORITY,
        ENTRYPOINT_MAIN_REMOVE_AUTHORITY,
        ENTRYPOINT_MAIN_EXTEND,
        CSPR_HASH,
    },
    errors::{
        MainContractErrors, CommonError
    },
};

use casper_contract::{
    contract_api::{
        account,
        runtime,
        storage,
        system,                
    },
    unwrap_or_revert::UnwrapOrRevert,
    ext_ffi,
    
};
use casper_types::{
    URef,
    ApiError,
    bytesrepr::{
        FromBytes,
        ToBytes
    }, 
    CLValue,
    CLType,
    EntryPoint,
    EntryPoints,
    EntryPointType,
    EntryPointAccess,
    Parameter,
    CLTyped,
    ContractHash,
    Contract,
    runtime_args,
    U512,
    RuntimeArgs, account::AccountHash, Key, contracts::NamedKeys
};
use common_lib::constants::{
    ARG_MAIN_DOMAIN,
    ARG_MAIN_DOMAIN_PAGE,
    ARG_MAIN_DURATION,
    ARG_MAIN_RESOLVER_ADDRESS,
    ARG_MAIN_SUBDOMAIN,
    ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH,
    KEY_DATABASE_DICTIONARY_DOMAIN,
    KEY_MAIN_DICTIONARY_DOMAIN_METADATA,
    KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
    KEY_DATABASE_DICTIONARY_SUBDOMAIN,
    KEY_PO_CONTRACT_HASH,

    ENTRYPOINT_MAIN_GET_DOMAIN_LIST,
    ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN,
    ENTRYPOINT_MAIN_REGISTER_DOMAIN,
    ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN,
    ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN,
    ENTRYPOINT_MAIN_RESOLVE_DOMAIN,
    ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN,
    ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN,
    ENTRYPOINT_MAIN_SET_PRICE_ORACLE_CONTRACT_HASH,
};


/**
 * Registers a new domain name
 * Steps:
 * 
 * 1. Check validity of the domain name
 * 2. Get Price from PriceOracle and check payment amount
 * 3. Create NFT for the domain name
 * 4. Store domain in store
 */
#[no_mangle]
pub extern "C" fn register_domain() {

    let domain: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);
    let duration: u8 = runtime::get_named_arg(ARG_MAIN_DURATION);
    let resolver_address: AccountHash = runtime::get_named_arg(ARG_MAIN_RESOLVER_ADDRESS);
    let amount: U512 = runtime::get_named_arg(ARG_MAIN_REGISTER_AMOUNT);

    let caller = runtime::get_caller();
    
    if !is_domain_name_valid(&domain) {
        runtime::revert(MainContractErrors::InvalidDomain);
    }
    
    let chars_count = get_domain_name_chars_count(&domain);
    
    if chars_count <= 3 && !is_maintainer_or_has_authority(&caller) {
        runtime::revert(MainContractErrors::UserHasNoAccessToRegister);
    }

    let store_domain_optional: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);
    
    if let Some(store_domain) = store_domain_optional {
        let actual_state = get_end_time_actual_state(Some(store_domain.end_time));
        match actual_state {
            DomainNameActualState::Busy => {
                runtime::revert(MainContractErrors::DomainNameIsBusy)
            },
            DomainNameActualState::GracePeriod => {
                runtime::revert(MainContractErrors::DomainNameIsInGracePeriod)
            },
            DomainNameActualState::Available => {}
        }
    }

    if duration > 3 || duration <= 0 {
        runtime::revert(MainContractErrors::InvalidDuration);
    }

    let price_oracle_contract_hash = get_stored_value_from_key::<ContractHash>(KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH)    
        .unwrap_or_revert_with(MainContractErrors::PriceOracleContractHashNotFound);

    let price: U512 = runtime::call_contract(
        price_oracle_contract_hash,
        ENDPOINT_PO_GET_PRICE,
        runtime_args! {
            "arg_price_type_chars_count" => chars_count as u64
        }
    );

    if U512::from(duration) * price != amount {
        runtime::revert(MainContractErrors::PriceDiscrepancy);
    }
    
    let end_time = calculate_domain_name_end_date(duration);
    let binding = domain.clone();
    let splitted = binding.split('.').collect::<Vec<&str>>();
    let label = splitted.first().expect("Error while getting first element of split!").to_string();

    let token_id = namehash_label(CSPR_HASH, &label);

    let caller = runtime::get_caller();

    let metadata: Option<LocalMetadata> = get_stored_value_from_key(KEY_MAIN_DICTIONARY_DOMAIN_METADATA);
    
    if let Some(mut meta) = metadata {

        // TODO: create NFT for the domain name here
        // TODO: payment system

        let current_pagination_key = &format!("{}", &meta.page);
        let domain_list: Option<Vec<String>> = get_dictionary_value_from_key(
            KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
            current_pagination_key
        );
        if let Some(mut dl) = domain_list {
            
            dl.push(domain.clone());
            upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN_LIST, current_pagination_key, dl.clone());
            
            
            if dl.len() == MAX_PAGE_SIZE as usize {
                meta.page += 1;
                upsert_dictionary_value_from_key(
                    KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
                    &format!("{}", &meta.page),
                    Vec::<String>::new()
                );
            }
            meta.total_count += 1;
            store_value_for_key(KEY_MAIN_DICTIONARY_DOMAIN_METADATA, meta);
            
            upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain, DomainName {
                end_time,
                name: domain.clone(),        
                token_id,
                owner: caller,
                resolver: resolver_address,
            });
        } else {
            runtime::revert(MainContractErrors::NoDictionaryDomainList);
        }
        
    } else {
        runtime::revert(MainContractErrors::NoDictionaryDomainMetadata);
    }
}


/**
 * Searchs by domain name, if domain name is found, returns resolver address of the entity
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
    let store_domain: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain_name);
    if let Some(domain) = store_domain {
        // TODO: check expiration of the domain
        let result = CLValue::from_t(domain.resolver).expect("Error while create cl_value from resolver address");
        runtime::ret(result);
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
    let found_domain_name_item: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain_name);

    if let Some(mut domain) = found_domain_name_item {

        if domain.owner == runtime::get_caller() {
            domain.resolver = resolver_address;
            let binding = domain.clone();
            upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain_name, binding);
        } else {
            runtime::revert(MainContractErrors::InvalidOwner);
        }        
    } else {
        runtime::revert(MainContractErrors::DomainNotExists);
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
    
    if let Some (domain) = domain_name {
        let found_domain: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);
        if let Some(dm) = found_domain {
            if &dm.owner == &caller {
                let subdomains: Option<Vec<SubdomainName>> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain);
                if let Some (mut sd) = subdomains {
                    if sd.len() < MAX_SUBDOMAIN_COUNT.into() {
                        
                        sd.push(SubdomainName {
                            name: subdomain_name,
                            resolver: resolver_address
                        });
                        upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain, sd);
                    } else {
                        runtime::revert(MainContractErrors::SubdomainMaxCountExceeded);
                    }
                } else {
                    upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain, vec![
                        SubdomainName {
                            name: subdomain_name,
                            resolver: resolver_address
                        }
                    ]);
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
        let found_domain: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);

        if let Some(dm) = found_domain {
            if &dm.owner == &caller {
                let subdomains: Option<Vec<SubdomainName>> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain);
                if let Some (mut sd) = subdomains {
                    let pos = sd.iter().position(|x| &x.name == &subdomain_name).unwrap();
                    if pos >= 0 {
                        sd.remove(pos);
                        upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain, sd);
                    } else {
                        runtime::revert(MainContractErrors::SubdomainNotExists);
                    }
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
 * Not in MVP
 * 
 * 1. Check validity of the sub domain name
 * 2. Get Price from PriceOracle and check payment amount
 * 3. Store data
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
    if let Some (domain) = domain_name {
        let found_domain: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);

        if let Some(dm) = found_domain {
            if &dm.owner == &caller {
                let subdomains: Option<Vec<SubdomainName>> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain);
                if let Some (sd) = subdomains {
                    let mapped = sd.iter().map(|x| {
                        if &x.name == &subdomain_name {
                            
                            return SubdomainName {
                                name: x.name.to_string(),
                                resolver: resolver_address
                            };                            
                        }
                        return (*x).clone();
                    }).collect::<Vec<SubdomainName>>();
                    upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain, mapped);
                    
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
        let subdomains: Option<Vec<SubdomainName>> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_SUBDOMAIN, &domain_name);
        let result = CLValue::from_t(subdomains).expect("Error while converting subdomains to cl_value");
        runtime::ret(result);
    } else {
        runtime::revert(MainContractErrors::InvalidDomain);
    }
}

#[no_mangle]
pub extern "C" fn get_domain_list() {
    let page: u8 = runtime::get_named_arg(ARG_MAIN_DOMAIN_PAGE);
    let domains: Option<Vec<String>> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN_LIST, &format!("{}:{}", page, MAX_PAGE_SIZE));
    if let Some(list) = domains {
        let result = CLValue::from_t(list).expect("error while converting domain list to cl_value");
        runtime::ret(result);
    } else {
        let result = CLValue::from_t::<Vec<String>>(vec![]).expect("error while converting domain list to cl_value");
        runtime::ret(result);
    }
}

#[no_mangle]
pub extern "C" fn add_authority() {
    let caller = runtime::get_caller();
    let maintainer = get_stored_value_from_key::<AccountHash>(KEY_MAIN_MAINTAINER)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    if &caller != &maintainer {
        runtime::revert(MainContractErrors::OnlyMaintainerHasAccess);
    }

    let authority: AccountHash = runtime::get_named_arg(ARG_MAIN_AUTHORITY);
    
    if &authority == &maintainer {
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
    if &caller != &maintainer {
        runtime::revert(MainContractErrors::OnlyMaintainerHasAccess);
    }

    let authority: AccountHash = runtime::get_named_arg(ARG_MAIN_AUTHORITY);
    
    if &authority == &maintainer {
        runtime::revert(MainContractErrors::CannotRemoveMaintainer);
    }
    let has_access = is_maintainer_or_has_authority(&authority);
    if !has_access {
        runtime::revert(MainContractErrors::UserHasNoAccess);
    }
    let mut authorities = get_stored_value_from_key::<Vec<AccountHash>>(KEY_MAIN_AUTHORITIES)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    let index = authorities.iter().position(|item| { item == &authority }).unwrap();
    authorities.remove(index);
    store_value_for_key(KEY_MAIN_AUTHORITIES, authorities);
}

#[no_mangle]
pub extern "C" fn extend() {
    let domain: String = runtime::get_named_arg(ARG_MAIN_DOMAIN);
    let duration: u8 = runtime::get_named_arg(ARG_MAIN_DURATION);
    
    if !is_domain_name_valid(&domain) {
        runtime::revert(MainContractErrors::InvalidDomain);
    }

    let store_domain_optional: Option<DomainName> = get_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain);
    if let Some(mut domain) = store_domain_optional {
        let result = is_extension_duration_correct(domain.clone().end_time, duration.into());

        if result {
            let binding = year_to_millis(duration);
            domain.end_time = domain.clone().end_time + binding;
            upsert_dictionary_value_from_key(KEY_DATABASE_DICTIONARY_DOMAIN, &domain.name, domain.clone());
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
pub extern "C" fn set_data() {

}

/**
 * Not in MVP
 */
#[no_mangle]
pub extern "C" fn get_data() {

}

#[no_mangle]
pub extern "C" fn set_price_oracle_contract_hash() {
    let price_oracle_contract_hash: ContractHash = runtime::get_named_arg(ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH);
    store_value_for_key(KEY_PO_CONTRACT_HASH, price_oracle_contract_hash);    
}

#[no_mangle]
pub extern "C" fn init() {
    storage::new_dictionary(KEY_DATABASE_DICTIONARY_DOMAIN).unwrap_or_revert();
    storage::new_dictionary(KEY_DATABASE_DICTIONARY_SUBDOMAIN).unwrap_or_revert();
    storage::new_dictionary(KEY_MAIN_DICTIONARY_DOMAIN_METADATA).unwrap_or_revert();
    storage::new_dictionary(KEY_DATABASE_DICTIONARY_DOMAIN_LIST).unwrap_or_revert();
    upsert_dictionary_value_from_key::<Vec<String>>(KEY_DATABASE_DICTIONARY_DOMAIN_LIST, &format!("{}", 0), vec![]);
    let metadata_uref = storage::new_uref(LocalMetadata {
        total_count: 0,
        page: 0,
    });
    runtime::put_key(KEY_MAIN_DICTIONARY_DOMAIN_METADATA, casper_types::Key::URef(metadata_uref));
}

/**
 * Endpoints:
 * 1. register_domain
 * 2. resolve_domain
 * 3. set_resolver_address_for_domain
 * 4. register_sub_domain_name
 * 5. remove_subdomain
 * 6. set_resolver_address_for_subdomain
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

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_SET_PRICE_ORACLE_CONTRACT_HASH,
            vec![
                Parameter::new(ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH, ContractHash::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public, 
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            "init",
            vec![],
            CLType::Unit,
            EntryPointAccess::Public, 
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_REGISTER_DOMAIN,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
                Parameter::new(ARG_MAIN_DURATION, u8::cl_type()),
                Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_RESOLVE_DOMAIN,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN, String::cl_type())
            ],
            CLType::Any,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
                Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_RESOLVE_DOMAIN,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
                Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, String::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN,
            vec![
                Parameter::new(ARG_MAIN_SUBDOMAIN, String::cl_type()),
                Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN,
            vec![
                Parameter::new(ARG_MAIN_SUBDOMAIN, String::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN,
            vec![
                Parameter::new(ARG_MAIN_SUBDOMAIN, String::cl_type()),
                Parameter::new(ARG_MAIN_RESOLVER_ADDRESS, AccountHash::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN, String::cl_type())
            ],
            CLType::Any,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_ADD_AUTHORITY,
            vec![
                Parameter::new(ARG_MAIN_AUTHORITY, AccountHash::cl_type())
            ],
            CLType::Any,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_REMOVE_AUTHORITY,
            vec![
                Parameter::new(ARG_MAIN_AUTHORITY, AccountHash::cl_type())
            ],
            CLType::Any,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_ADD_AUTHORITY,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN_PAGE, u8::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENTRYPOINT_MAIN_EXTEND,
            vec![
                Parameter::new(ARG_MAIN_DOMAIN, String::cl_type()),
                Parameter::new(ARG_MAIN_DURATION, u8::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );
    
    let mut main_named_keys = NamedKeys::new();
    
    let maintainer_uref = storage::new_uref(runtime::get_caller());
    main_named_keys.insert(KEY_MAIN_MAINTAINER.to_string(), maintainer_uref.into());
    
    let authorities_uref = storage::new_uref(Vec::<AccountHash>::new());
    main_named_keys.insert(KEY_MAIN_AUTHORITIES.to_string(), authorities_uref.into());

    let price_oracle_contract_hash: ContractHash = runtime::get_named_arg(ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH);
    let price_oracle_contract_hash_uref = storage::new_uref(price_oracle_contract_hash);
    main_named_keys.insert(KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH.to_string(), price_oracle_contract_hash_uref.into());    

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
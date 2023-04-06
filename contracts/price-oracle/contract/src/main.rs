#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod price_oracle_db;
mod price_fetcher;

extern crate alloc;

use alloc::{
    string::{
        ToString
    },
    vec,
    vec::Vec
};

use casper_contract::{
    contract_api::{
        runtime::{ self, revert },
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,    
};
use casper_types::{    
    CLTyped,
    CLValue,
    CLType,
    EntryPoint,
    EntryPoints,
    EntryPointType,
    EntryPointAccess,
    Parameter,
    U512,
    contracts::NamedKeys,
    account::AccountHash,
};

use common_lib::{
    enums::price_oracle_contract::PriceType,
    constants::{
        ARG_PO_PRICE_TYPE,
        ARG_PO_PRICE,
        ARG_PO_PRICE_MID,
        ARG_PO_PRICE_MORE,
        ARG_PO_PRICE_TYPE_CHARS_COUNT,
        ARG_PO_CHARS_COUNT_MID,
        ARG_PO_AUTHORITY,
        KEY_PO_MAINTAINER,
        KEY_PO_AUTHORITIES,
        KEY_PO_PRICE_TYPE,
        KEY_PO_PRICE,
        KEY_PO_PRICE_MID,
        KEY_PO_PRICE_MORE,
        KEY_PO_CHARS_COUNT_MID,
        KEY_PO_CONTRACT_HASH,
        KEY_PO_CONTRACT_VERSION,
        KEY_PO_CONTRACT_PACKAGE_NAME,
        KEY_MAIN_CONTRACT_ACCESS_UREF,
        
        ENDPOINT_PO_GET_PRICE,
        ENDPOINT_PO_SET_PRICE,
        ENDPOINT_PO_ADD_AUTHORITY,
        ENDPOINT_PO_REMOVE_AUTHORITY,        
    },
    utils::{
        storage::{
            store_value_for_key,
            get_stored_value_from_key,
        },
        authority::{
            has_authority,
            is_maintainer
        },
    },
    errors::{
        PriceOracleContractErrors,
        CommonError
    },
    defaults::{
        price_oracle::{
            DEFAULT_DOMAIN_NAME_PRICE
        },
    }
};
use common_lib::constants::{ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS, ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS, KEY_PO_SIMPLE_OPERATIONS};
use common_lib::utils::response::{response_error, response_success};
use crate::price_fetcher::PriceFetcher;
use crate::price_oracle_db::PriceOracleDb;

#[no_mangle]
pub extern "C" fn set_price() {
    // let caller = runtime::get_caller();
    // let has_access = is_maintainer_or_has_authority(&caller);
    // if !has_access {
    //     revert(CommonError::NoAuthority);
    // }
    let mut db_instance = PriceOracleDb::instance();
    let price_type: PriceType = runtime::get_named_arg(ARG_PO_PRICE_TYPE);

    match price_type {
        PriceType::Fixed => {
            let price: U512 = runtime::get_named_arg(ARG_PO_PRICE);
            db_instance.set_fixed_price(price);
        },
        PriceType::Dynamic => {
            let price: U512 = runtime::get_named_arg(ARG_PO_PRICE);
            let price_mid: Vec<U512> = runtime::get_named_arg(ARG_PO_PRICE_MID);
            let chars_count_mid: Vec<u64> = runtime::get_named_arg(ARG_PO_CHARS_COUNT_MID);
            if price_mid.len() != chars_count_mid.len() {
                revert(PriceOracleContractErrors::PriceMidLengthAndMidCharsCountMismatch)
            }
            let price_more: U512 = runtime::get_named_arg(ARG_PO_PRICE_MORE);

            db_instance.set_dynamic_price(price, price_mid, chars_count_mid, price_more);
        },
    }
}

#[no_mangle]
pub extern "C" fn get_price_simple_operations() {
    let price_fetcher = PriceFetcher::instance();
    let price = price_fetcher.get_price_simple_operations();
    if price.is_none() {
        response_error(PriceOracleContractErrors::PriceSimpleOperationsIsNotSet);
    }
    response_success(price.unwrap(), "Error while getting value type");
}

#[no_mangle]
pub extern "C" fn set_price_simple_operations() {
    let price: U512 = runtime::get_named_arg(ARG_PO_PRICE);
    let db_instance = PriceOracleDb::instance();
    db_instance.set_simple_operations_price(price);
}

#[no_mangle]
pub extern "C" fn get_price() {

    let price_type = get_stored_value_from_key::<PriceType>(KEY_PO_PRICE_TYPE);
    if price_type.is_none() {
        response_error(PriceOracleContractErrors::PriceTypeIsNotFound);
    }

    let mut price_fetcher = PriceFetcher::instance();

    match price_type.unwrap() {
        PriceType::Fixed => {
            let price = price_fetcher.get_fixed_price();
            if price.is_none() {
                response_error(PriceOracleContractErrors::PriceIsNotSet);
            }
            response_success(price.unwrap(), "Error while getting value type");
        },
        PriceType::Dynamic => {
            let chars_count: u64 = runtime::get_named_arg(ARG_PO_PRICE_TYPE_CHARS_COUNT);
            let price = price_fetcher.get_price_dynamic(chars_count);
            if price.is_none() {
                response_error(PriceOracleContractErrors::PriceForCharsCountNotFound);
            }
            response_success(price.unwrap(), "Error while getting value type");
        }
    }    
}

#[no_mangle]
pub extern "C" fn add_authority() {
    let caller = runtime::get_caller();
    let maintainer = get_stored_value_from_key::<AccountHash>(KEY_PO_MAINTAINER)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    if &caller != &maintainer {
        revert(PriceOracleContractErrors::PriceOnlyMaintainerHasAccess);
    }

    let authority: AccountHash = runtime::get_named_arg(ARG_PO_AUTHORITY);
    
    if &authority == &maintainer {
        revert(PriceOracleContractErrors::PriceCannotAddMaintainer);
    }
    let has_access = is_maintainer_or_has_authority(&authority);
    if has_access {
        revert(PriceOracleContractErrors::PriceAuthorityHasAlreadyTaken);
    }
    let mut authorities = get_stored_value_from_key::<Vec<AccountHash>>(KEY_PO_AUTHORITIES)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    authorities.push(authority);
    store_value_for_key(KEY_PO_AUTHORITIES, authorities);
}

#[no_mangle]
pub extern "C" fn remove_authority() {
    let caller = runtime::get_caller();
    let maintainer = get_stored_value_from_key::<AccountHash>(KEY_PO_MAINTAINER)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    if &caller != &maintainer {
        revert(PriceOracleContractErrors::PriceOnlyMaintainerHasAccess);
    }

    let authority: AccountHash = runtime::get_named_arg(ARG_PO_AUTHORITY);
    
    if &authority == &maintainer {
        revert(PriceOracleContractErrors::PriceCannotRemoveMaintainer);
    }
    let has_access = is_maintainer_or_has_authority(&authority);
    if !has_access {
        revert(PriceOracleContractErrors::PriceUserHasNoAccess);
    }
    let mut authorities = get_stored_value_from_key::<Vec<AccountHash>>(KEY_PO_AUTHORITIES)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    let index = authorities.iter().position(|item| { item == &authority }).unwrap();
    authorities.remove(index);
    store_value_for_key(KEY_PO_AUTHORITIES, authorities);
}

#[no_mangle]
pub extern "C" fn call() {
    
    let mut entrypoints = EntryPoints::new();

    entrypoints.add_entry_point(EntryPoint::new(
        ENDPOINT_PO_SET_PRICE,
        vec![
            Parameter::new(ARG_PO_PRICE_TYPE, PriceType::cl_type()),
            Parameter::new(ARG_PO_PRICE, U512::cl_type()),
            Parameter::new(ARG_PO_PRICE_MID, Vec::<U512>::cl_type()),
            Parameter::new(ARG_PO_CHARS_COUNT_MID, Vec::<u64>::cl_type()),
            Parameter::new(ARG_PO_PRICE_MORE, U512::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public, 
        EntryPointType::Contract
    ));
    entrypoints.add_entry_point(EntryPoint::new(
        ENDPOINT_PO_GET_PRICE,
        vec![
            Parameter::new(ARG_PO_PRICE_TYPE_CHARS_COUNT, u8::cl_type())            
        ],
        CLType::Unit,
        EntryPointAccess::Public, 
        EntryPointType::Contract
    ));
    entrypoints.add_entry_point(EntryPoint::new(
        ENDPOINT_PO_ADD_AUTHORITY, 
        vec![
            Parameter::new(ARG_PO_AUTHORITY, AccountHash::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public, 
        EntryPointType::Contract
    ));
    entrypoints.add_entry_point(EntryPoint::new(
        ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS,
        vec![],
        CLType::U512,
        EntryPointAccess::Public,
        EntryPointType::Contract
    ));

    entrypoints.add_entry_point(EntryPoint::new(
        ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS,
        vec![
            Parameter::new(ARG_PO_PRICE, U512::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    ));
    entrypoints.add_entry_point(EntryPoint::new(
        ENDPOINT_PO_REMOVE_AUTHORITY, 
        vec![
            Parameter::new(ARG_PO_AUTHORITY, AccountHash::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public, 
        EntryPointType::Contract
    ));

    let mut price_oralce_named_keys = NamedKeys::new();
    let price_type_uref = storage::new_uref(PriceType::Fixed);
    price_oralce_named_keys.insert(KEY_PO_PRICE_TYPE.to_string(), price_type_uref.into());
    
    let price_uref = storage::new_uref(U512::from(DEFAULT_DOMAIN_NAME_PRICE));
    price_oralce_named_keys.insert(KEY_PO_PRICE.to_string(), price_uref.into());
    
    let price_mid_uref = storage::new_uref(Vec::<U512>::new());
    price_oralce_named_keys.insert(KEY_PO_PRICE_MID.to_string(), price_mid_uref.into());
    
    let chars_count_mid_uref = storage::new_uref(Vec::<u64>::new());
    price_oralce_named_keys.insert(KEY_PO_CHARS_COUNT_MID.to_string(), chars_count_mid_uref.into());
    
    let price_more_uref = storage::new_uref(U512::from(0u64));
    price_oralce_named_keys.insert(KEY_PO_PRICE_MORE.to_string(), price_more_uref.into());
    
    let price_oracle_maintainer = storage::new_uref(runtime::get_caller());
    price_oralce_named_keys.insert(KEY_PO_MAINTAINER.to_string(), price_oracle_maintainer.into());
    
    let price_oracle_authorities = storage::new_uref(Vec::<AccountHash>::new());
    price_oralce_named_keys.insert(KEY_PO_AUTHORITIES.to_string(), price_oracle_authorities.into());

    let price_oracle_price_simple_operations = storage::new_uref(U512::from(0u64));
    price_oralce_named_keys.insert(KEY_PO_SIMPLE_OPERATIONS.to_string(), price_oracle_price_simple_operations.into());

    let (contract_hash, contract_version) = storage::new_contract(
        entrypoints,
        Some(price_oralce_named_keys),
        Some(KEY_PO_CONTRACT_PACKAGE_NAME.to_string()),
        Some(KEY_MAIN_CONTRACT_ACCESS_UREF.to_string()),
    );

    let version_uref = storage::new_uref(contract_version);
    let contract_hash_uref = storage::new_uref(contract_hash);
    runtime::put_key(KEY_PO_CONTRACT_VERSION, version_uref.into());
    runtime::put_key(KEY_PO_CONTRACT_HASH, contract_hash_uref.into());

}

// helpers
fn is_maintainer_or_has_authority(account: &AccountHash) -> bool {
    let caller_has_authority = has_authority(KEY_PO_AUTHORITIES, &account);
    let is_maintainer = is_maintainer(KEY_PO_MAINTAINER, &account);

    is_maintainer || caller_has_authority 
}
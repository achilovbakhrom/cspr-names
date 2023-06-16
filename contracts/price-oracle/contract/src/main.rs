#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod price_fetcher;
mod price_oracle_db;

extern crate alloc;

use alloc::string::String;
use alloc::{ string::ToString, vec, vec::Vec };

use casper_contract::{
    contract_api::{ runtime::{ self, revert }, storage },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash,
    contracts::NamedKeys,
    CLType,
    CLTyped,
    CLValue,
    EntryPoint,
    EntryPointAccess,
    EntryPointType,
    EntryPoints,
    Parameter,
    U512,
};

use crate::price_fetcher::PriceFetcher;
use crate::price_oracle_db::PriceOracleDb;
use common_lib::constants::{
    ARG_PO_EXTENSION,
    ENDPOINT_PO_INIT,
    ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS,
    ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS,
    KEY_PO_SIMPLE_OPERATIONS,
};
use common_lib::utils::response::{ response_error, response_success };
use common_lib::{
    constants::{
        ARG_PO_AUTHORITY,
        ARG_PO_CHARS_COUNT_MID,
        ARG_PO_PRICE,
        ARG_PO_PRICE_MID,
        ARG_PO_PRICE_MORE,
        ARG_PO_PRICE_TYPE,
        ARG_PO_PRICE_TYPE_CHARS_COUNT,
        ENDPOINT_PO_ADD_AUTHORITY,
        ENDPOINT_PO_GET_PRICE,
        ENDPOINT_PO_REMOVE_AUTHORITY,
        ENDPOINT_PO_SET_PRICE,
        KEY_MAIN_CONTRACT_ACCESS_UREF,
        KEY_PO_AUTHORITIES,
        KEY_PO_CHARS_COUNT_MID,
        KEY_PO_CONTRACT_HASH,
        KEY_PO_CONTRACT_PACKAGE_NAME,
        KEY_PO_CONTRACT_VERSION,
        KEY_PO_MAINTAINER,
        KEY_PO_PRICE,
        KEY_PO_PRICE_MID,
        KEY_PO_PRICE_MORE,
        KEY_PO_PRICE_TYPE,
    },
    defaults::price_oracle::DEFAULT_DOMAIN_NAME_PRICE,
    enums::price_oracle_contract::PriceType,
    errors::{ CommonError, PriceOracleContractErrors },
    utils::{
        authority::{ has_authority, is_maintainer },
        storage::{ get_stored_value_from_key, store_value_for_key },
    },
};

#[no_mangle]
pub extern "C" fn set_price() {
    let mut db_instance = PriceOracleDb::instance();
    let extension: String = runtime::get_named_arg(ARG_PO_EXTENSION);
    let price_type: PriceType = runtime::get_named_arg(ARG_PO_PRICE_TYPE);

    match price_type {
        PriceType::Fixed => {
            let price: U512 = runtime::get_named_arg(ARG_PO_PRICE);
            db_instance.set_fixed_price(&extension, price);
        }
        PriceType::Dynamic => {
            let price: U512 = runtime::get_named_arg(ARG_PO_PRICE);
            let price_mid: Vec<U512> = runtime::get_named_arg(ARG_PO_PRICE_MID);
            let chars_count_mid: Vec<u64> = runtime::get_named_arg(ARG_PO_CHARS_COUNT_MID);
            if price_mid.len() != chars_count_mid.len() {
                revert(PriceOracleContractErrors::PriceMidLengthAndMidCharsCountMismatch);
            }
            let price_more: U512 = runtime::get_named_arg(ARG_PO_PRICE_MORE);

            db_instance.set_dynamic_price(
                &extension,
                price,
                price_mid,
                chars_count_mid,
                price_more
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn get_price() {
    let chars_count: u8 = runtime::get_named_arg(ARG_PO_PRICE_TYPE_CHARS_COUNT);
    let extension: String = runtime::get_named_arg(ARG_PO_EXTENSION);

    let price_fetcher = PriceFetcher::instance();

    let price = price_fetcher.get_price_for(&extension, chars_count);

    if let Some(p) = price {
        response_success(p, "Error while converting CL_Value");
    }

    response_error(PriceOracleContractErrors::PricePriceIsNotSetForExtension);
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
    db_instance.set_price_for_simple_operations(price);
}

#[no_mangle]
pub extern "C" fn init() {
    PriceOracleDb::initialize()
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entrypoints = EntryPoints::new();

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_PO_INIT,
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_PO_SET_PRICE,
            vec![
                Parameter::new(ARG_PO_EXTENSION, String::cl_type()),
                Parameter::new(ARG_PO_PRICE_TYPE, PriceType::cl_type()),
                Parameter::new(ARG_PO_PRICE, U512::cl_type()),
                Parameter::new(ARG_PO_PRICE_MID, Vec::<U512>::cl_type()),
                Parameter::new(ARG_PO_CHARS_COUNT_MID, Vec::<u64>::cl_type()),
                Parameter::new(ARG_PO_PRICE_MORE, U512::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );
    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_PO_GET_PRICE,
            vec![
                Parameter::new(ARG_PO_EXTENSION, String::cl_type()),
                Parameter::new(ARG_PO_PRICE_TYPE_CHARS_COUNT, u8::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS,
            vec![Parameter::new(ARG_PO_PRICE, U512::cl_type())],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );
    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_PO_REMOVE_AUTHORITY,
            vec![Parameter::new(ARG_PO_AUTHORITY, AccountHash::cl_type())],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    let mut price_oracle_named_keys = NamedKeys::new();

    let price_oracle_maintainer = storage::new_uref(runtime::get_caller());
    price_oracle_named_keys.insert(KEY_PO_MAINTAINER.to_string(), price_oracle_maintainer.into());

    let price_oracle_price_simple_operations = storage::new_uref(U512::from(0u64));
    price_oracle_named_keys.insert(
        KEY_PO_SIMPLE_OPERATIONS.to_string(),
        price_oracle_price_simple_operations.into()
    );

    let (contract_hash, contract_version) = storage::new_contract(
        entrypoints,
        Some(price_oracle_named_keys),
        Some(KEY_PO_CONTRACT_PACKAGE_NAME.to_string()),
        Some(KEY_MAIN_CONTRACT_ACCESS_UREF.to_string())
    );

    let version_uref = storage::new_uref(contract_version);
    let contract_hash_uref = storage::new_uref(contract_hash);
    runtime::put_key(KEY_PO_CONTRACT_VERSION, version_uref.into());
    runtime::put_key(KEY_PO_CONTRACT_HASH, contract_hash_uref.into());
}

#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod authority_db;
mod contract_hashes_db;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, CLType, CLTyped, ContractHash, EntryPoint, EntryPointAccess, EntryPoints, EntryPointType, Key, Parameter};
use casper_types::account::AccountHash;
use casper_types::CLType::String;
use casper_types::contracts::NamedKeys;
use common_lib::constants::{ARG_AUTHORITY_AUTHORITY_LIST, ARG_AUTHORITY_CONTRACT_HASH, ARG_AUTHORITY_CONTRACT_TYPE, ARG_AUTHORITY_EXTENSION, ARG_AUTHORITY_MUTATION_TYPE, ENDPOINT_AUTHORITY_GET_AUTHORITY, ENDPOINT_AUTHORITY_GET_CONTRACT, ENDPOINT_AUTHORITY_REMOVE_AUTHORITY, ENDPOINT_AUTHORITY_REMOVE_CONTRACT, ENDPOINT_AUTHORITY_SET_AUTHORITY, ENDPOINT_AUTHORITY_SET_CONTRACT, KEY_AUTHORITY_CONTRACT_ACCESS_UREF, KEY_AUTHORITY_CONTRACT_HASH, KEY_AUTHORITY_CONTRACT_PACKAGE_NAME, KEY_AUTHORITY_CONTRACT_VERSION, KEY_AUTHORITY_MAINTAINER};
use common_lib::enums::contracts_enum::ContractKind;
use common_lib::enums::mutation_type::MutationType;
use common_lib::errors::AuthorityErrors;
use common_lib::utils::response::{response_error, response_success};
use common_lib::utils::storage::get_stored_value_from_key;
use crate::authority_db::AuthorityDb;
use crate::contract_hashes_db::ContractHashDb;

fn check_whether_maintainer_or_not() {
    let maintainer = get_stored_value_from_key::<AccountHash>(KEY_AUTHORITY_MAINTAINER)
        .unwrap_or_revert_with(AuthorityErrors::AuthorityMaintainerIsNotSet);
    let caller = runtime::get_caller();
    if maintainer != caller {
        response_error(AuthorityErrors::AuthorityInvalidCaller);
    }
}

pub extern "C" fn set_authority() {
    check_whether_maintainer_or_not();
    let operation_type = runtime::get_named_arg::<MutationType>(ARG_AUTHORITY_MUTATION_TYPE);
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_AUTHORITY_CONTRACT_HASH);
    let authority_list = runtime::get_named_arg::<Vec<Key>>(ARG_AUTHORITY_AUTHORITY_LIST);

    let instance = AuthorityDb::instance();

    match operation_type {
        MutationType::Replace => {
            instance.set_authority_list(contract_hash, authority_list)
        },
        MutationType::Append => {
            instance.add_authority_list(contract_hash, authority_list)
        }
    }
}

pub extern "C" fn get_authority() {
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_AUTHORITY_CONTRACT_HASH);

    let list = AuthorityDb::instance().get_authority_list(contract_hash).unwrap_or(vec![]);
    response_success(list, "Cannot convert Value To CL_Value")
}

pub extern "C" fn remove_authority() {
    check_whether_maintainer_or_not();
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_AUTHORITY_CONTRACT_HASH);
    let authority_list = runtime::get_named_arg::<Vec<Key>>(ARG_AUTHORITY_AUTHORITY_LIST);
    authority_list.iter().for_each(|item| {
        AuthorityDb::instance().remove_authority(contract_hash, item.into())
    })
}

pub extern "C" fn set_contract() {
    check_whether_maintainer_or_not();
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_AUTHORITY_CONTRACT_HASH);
    let contract_type = runtime::get_named_arg::<ContractKind>(ARG_AUTHORITY_CONTRACT_TYPE);
    let extension = runtime::get_named_arg::<Option<String>>(ARG_AUTHORITY_EXTENSION);

    ContractHashDb::instance().set_contract_hash(contract_type, contract_hash, extension)
}

pub extern "C" fn get_contract() {
    let contract_type = runtime::get_named_arg::<ContractKind>(ARG_AUTHORITY_CONTRACT_TYPE);
    let extension = runtime::get_named_arg::<Option<String>>(ARG_AUTHORITY_EXTENSION);
    let result = ContractHashDb::instance().get_contract_hash(contract_type, extension);

    response_success(result, "Error while converting value to CL_VALUE")
}

pub extern "C" fn remove_contract() {
    check_whether_maintainer_or_not();
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_AUTHORITY_CONTRACT_HASH);
    let contract_type = runtime::get_named_arg::<ContractKind>(ARG_AUTHORITY_CONTRACT_TYPE);
    let extension = runtime::get_named_arg::<Option<String>>(ARG_AUTHORITY_EXTENSION);
    ContractHashDb::instance().remove_contract_hash(contract_type, contract_hash, extension)
}

pub extern "C" fn init() {
    AuthorityDb::initialize();
    ContractHashDb::initialize();
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entrypoints = EntryPoints::new();

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_AUTHORITY_SET_AUTHORITY,
            vec![
                Parameter::new(ARG_AUTHORITY_CONTRACT_TYPE, ContractKind::cl_type()),
                Parameter::new(ARG_AUTHORITY_CONTRACT_HASH, ContractHash::cl_type()),
                Parameter::new(ARG_AUTHORITY_AUTHORITY_LIST, Vec::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_AUTHORITY_GET_AUTHORITY,
            vec![
                Parameter::new(ARG_AUTHORITY_CONTRACT_HASH, ContractHash::cl_type()),
            ],
            CLType::Any,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_AUTHORITY_REMOVE_AUTHORITY,
            vec![
                Parameter::new(ARG_AUTHORITY_CONTRACT_HASH, ContractHash::cl_type()),
                Parameter::new(ARG_AUTHORITY_AUTHORITY_LIST, Vec::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_AUTHORITY_SET_CONTRACT,
            vec![
                Parameter::new(ARG_AUTHORITY_CONTRACT_HASH, ContractHash::cl_type()),
                Parameter::new(ARG_AUTHORITY_CONTRACT_TYPE, Vec::cl_type()),
                Parameter::new(ARG_AUTHORITY_EXTENSION, Option::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_AUTHORITY_GET_CONTRACT,
            vec![
                Parameter::new(ARG_AUTHORITY_CONTRACT_TYPE, Vec::cl_type()),
                Parameter::new(ARG_AUTHORITY_EXTENSION, Option::cl_type()),
            ],
            CLType::Any,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_AUTHORITY_REMOVE_CONTRACT,
            vec![
                Parameter::new(ARG_AUTHORITY_CONTRACT_HASH, ContractHash::cl_type()),
                Parameter::new(ARG_AUTHORITY_CONTRACT_TYPE, Vec::cl_type()),
                Parameter::new(ARG_AUTHORITY_EXTENSION, Option::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    let mut main_named_keys = NamedKeys::new();
    let maintainer_uref = storage::new_uref(runtime::get_caller());
    main_named_keys.insert(KEY_AUTHORITY_MAINTAINER.to_string(), maintainer_uref.into());

    let (contract_hash, version) = storage::new_contract(
        entrypoints,
        Some(main_named_keys),
        Some(KEY_AUTHORITY_CONTRACT_PACKAGE_NAME.to_string()),
        Some(KEY_AUTHORITY_CONTRACT_ACCESS_UREF.to_string()),
    );

    let contract_hash_uref = storage::new_uref(contract_hash);
    runtime::put_key(KEY_AUTHORITY_CONTRACT_HASH, contract_hash_uref.into());

    let contract_version_uref = storage::new_uref(version);
    runtime::put_key(KEY_AUTHORITY_CONTRACT_VERSION, contract_version_uref.into());
}

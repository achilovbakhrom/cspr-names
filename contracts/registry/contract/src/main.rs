#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod domain_contract_hash_map;
mod operators_db;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{
        self,
        runtime::{self},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{account::AccountHash, ApiError, ContractHash, Key};
use common_lib::{
    constants::{
        ARG_REGISTRY_DATABASE_CONTRACT_HASH, ARG_REGISTRY_DOMAIN_NAME,
        ARG_REGISTRY_NFT_CONTRACT_HASH, KEY_REGISTRY_MAINTAINER,
    },
    errors::{CommonError, RegistryErrors},
    models::registry_contract_hash_pair::RegistryContractHashPair,
    utils::{
        response::{response_error, response_success},
        storage::get_stored_value_from_key,
    },
};

use crate::{domain_contract_hash_map::DomainContractHashMap, operators_db::OperatorsDb};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

pub extern "C" fn map_domain_name_to_contract_hash() {
    let domain_name: String = runtime::get_named_arg(ARG_REGISTRY_DOMAIN_NAME);
    let database_contract_hash: ContractHash =
        runtime::get_named_arg(ARG_REGISTRY_DATABASE_CONTRACT_HASH);
    let nft_contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_NFT_CONTRACT_HASH);

    const maintainer: AccountHash = match get_stored_value_from_key(KEY_REGISTRY_MAINTAINER) {
        Some(res) => res,
        None => return response_error(RegistryErrors::MaintainerIsNotSet),
    };

    if runtime::get_caller() != maintainer {
        return response_error(CommonError::InvalidMaintainer);
    }

    let registry_object = RegistryContractHashPair {
        db_contract_hash: database_contract_hash,
        nft_contract_hash,
    };

    DomainContractHashMap::instance()
        .map_domain_name_to_contract_hash(domain_name.clone(), registry_object);
}

pub extern "C" fn get_contract_hash_for_domain_name() {
    let domain_name: String = runtime::get_named_arg(ARG_REGISTRY_DOMAIN_NAME);

    match DomainContractHashMap::instance().get_contract_hash_for_domain_name(domain_name) {
        Some(res) => response_success(res, "Error while converting CL_Typed value"),
        None => response_error(RegistryErrors::RegistryObjectNotFound),
    }
}

pub extern "C" fn set_contract_hash_list() {}

pub extern "C" fn get_contract() {}

pub extern "C" fn increment_count_of_contract() {}

pub extern "C" fn add_operator() {}

pub extern "C" fn remove_operator() {}

#[no_mangle]
pub extern "C" fn call() {
    // The key shouldn't already exist in the named keys.
    let missing_key = runtime::get_key(KEY_NAME);
    if missing_key.is_some() {
        runtime::revert(Error::KeyAlreadyExists);
    }

    // This contract expects a single runtime argument to be provided.  The arg is named "message"
    // and will be of type `String`.
    let value: String = runtime::get_named_arg(RUNTIME_ARG_NAME);

    // Store this value under a new unforgeable reference a.k.a `URef`.
    let value_ref = storage::new_uref(value);

    // Store the new `URef` as a named key with a name of `KEY_NAME`.
    let key = Key::URef(value_ref);
    runtime::put_key(KEY_NAME, key);

    // The key should now be able to be retrieved.  Note that if `get_key()` returns `None`, then
    // `unwrap_or_revert()` will exit the process, returning `ApiError::None`.
    let retrieved_key = runtime::get_key(KEY_NAME).unwrap_or_revert();
    if retrieved_key != key {
        runtime::revert(Error::KeyMismatch);
    }
}

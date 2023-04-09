#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod storage_db;
mod listing_db;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::{format, vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    ApiError,
    CLType,
    CLTyped,
    ContractHash,
    EntryPoint,
    EntryPointAccess,
    EntryPoints,
    EntryPointType,
    Key,
    Parameter,
    runtime_args
};
use casper_types::account::AccountHash;

use casper_types::contracts::NamedKeys;
use common_lib::constants::{ARG_NFT_CONTRACT_HASH, ARG_NFT_METADATA, ARG_NFT_TOKEN_OWNER, ENDPOINT_NFT_BURN, ENDPOINT_NFT_BUY, ENDPOINT_NFT_LIST, ENDPOINT_NFT_MINT, ENDPOINT_NFT_SET_NFT_CONTRACT_HASH, ENDPOINT_NFT_TRANSFER, ENDPOINT_NFT_UN_LIST, KEY_NFT_CONTRACT_ACCESS_UREF, KEY_NFT_CONTRACT_HASH, KEY_NFT_CONTRACT_PACKAGE_NAME, KEY_NFT_CONTRACT_VERSION};
use common_lib::errors::{DatabaseErrors, NFTErrors};
use common_lib::utils::response::response_error;
use crate::listing_db::ListingDb;
use crate::storage_db::StorageDb;
use casper_types::RuntimeArgs;
use common_lib::models::nft::Metadata;
use common_lib::utils::helpers::get_metadata_schema;
use serde_json::to_string;
use common_lib::errors::DatabaseErrors::DatabaseUnexpected;

const ARG_TOKEN_OWNER: &str = "token_owner";
const ARG_TOKEN_META_DATA: &str = "token_meta_data";
const ARG_TARGET_KEY: &str = "target_key";
const ARG_SOURCE_KEY: &str = "source_key";
const ARG_TOKEN_ID: &str = "token_id";
const ARG_REVERSE_LOOKUP: &str = "reverse_lookup";


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

#[no_mangle]
pub extern "C" fn mint() {
    let token_owner = runtime::get_named_arg::<AccountHash>(ARG_NFT_TOKEN_OWNER);
    let metadata = runtime::get_named_arg::<String>(ARG_NFT_METADATA);
    let nft_core_contract_hash: ContractHash = StorageDb::instance()
        .get_nft_core_contract_hash()
        .unwrap_or_revert_with(NFTErrors::NFTCoreHashIsNotSet);
    runtime::print(&format!("test: {}, owner: {}", &metadata, &token_owner));
    // let key = Key::Account()
    runtime::call_contract::<(String, Key, String)>(
        nft_core_contract_hash,
        ENDPOINT_NFT_MINT,
        runtime_args! {
            ARG_TOKEN_OWNER => Key::Account(token_owner),
            ARG_TOKEN_META_DATA => metadata,
        },
    );

}

#[no_mangle]
pub extern "C" fn transfer() {
    let nft_core_contract_hash: ContractHash = StorageDb::instance()
        .get_nft_core_contract_hash()
        .unwrap_or_revert_with(NFTErrors::NFTCoreHashIsNotSet);

    let token_id = runtime::get_named_arg::<u64>(ARG_TOKEN_ID);
    let from_token_owner = runtime::get_named_arg::<Key>(ARG_SOURCE_KEY);
    let target_token_owner = runtime::get_named_arg::<Key>(ARG_TARGET_KEY);

    let (collection_name, owned_tokens_dictionary_key) = runtime::call_contract::<(String, Key)>(
        nft_core_contract_hash,
        ENDPOINT_NFT_TRANSFER,
        runtime_args! {
            ARG_TOKEN_ID => token_id,
            ARG_SOURCE_KEY => from_token_owner,
            ARG_TARGET_KEY => target_token_owner
        },
    );

    runtime::put_key(&collection_name, owned_tokens_dictionary_key)
}

#[no_mangle]
pub extern "C" fn burn() {
    let nft_core_contract_hash: ContractHash = StorageDb::instance()
        .get_nft_core_contract_hash()
        .unwrap_or_revert_with(NFTErrors::NFTCoreHashIsNotSet);

    let token_id = runtime::get_named_arg::<u64>(ARG_TOKEN_ID);

    runtime::call_contract::<()>(
        nft_core_contract_hash,
        ENDPOINT_NFT_BURN,
        runtime_args! {
            ARG_TOKEN_ID => token_id
        },
    )
}

#[no_mangle]
pub extern "C" fn list() {
    let token_id = runtime::get_named_arg::<u64>(ARG_TOKEN_ID);
    let mut instance = ListingDb::instance();
    if !instance.is_listed(token_id) {
        instance.list(token_id)
    }
}

#[no_mangle]
pub extern "C" fn un_list() {
    let token_id = runtime::get_named_arg::<u64>(ARG_TOKEN_ID);
    let mut instance = ListingDb::instance();
    if instance.is_listed(token_id) {
        instance.un_list(token_id)
    }
}

#[no_mangle]
pub extern "C" fn buy() {
    let token_id = runtime::get_named_arg::<u64>(ARG_TOKEN_ID);
    let mut instance = ListingDb::instance();
    if !instance.is_listed(token_id) {
        response_error(NFTErrors::NFTIsNotListed)
    }
    // TODO: payment (2.5% - commission, 97.5% - to the owner)
    // TODO: transfer NFT
}

#[no_mangle]
pub extern "C" fn set_nft_contract_hash() {
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_NFT_CONTRACT_HASH);
    StorageDb::instance().set_nft_core_contract_hash(contract_hash);
}

/**
* 1. mint
* 2. transfer
* 3. burn
* 4. list
* 5. un_list
* 6. buy
*/
#[no_mangle]
pub extern "C" fn call() {

    let mut entrypoints = EntryPoints::new();
    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_MINT,
            vec![
                Parameter::new(ARG_NFT_TOKEN_OWNER, Key::cl_type()),
                Parameter::new(ARG_NFT_METADATA, String::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_BURN,
            vec![
                Parameter::new(ARG_TOKEN_ID, u64::cl_type())
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_TRANSFER,
            vec![
                Parameter::new(ARG_TOKEN_ID, u64::cl_type()),
                Parameter::new(ARG_SOURCE_KEY, Key::cl_type()),
                Parameter::new(ARG_TARGET_KEY, Key::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_LIST,
            vec![
                Parameter::new(ARG_TOKEN_ID, u64::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_UN_LIST,
            vec![
                Parameter::new(ARG_TOKEN_ID, u64::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_BUY,
            vec![
                Parameter::new(ARG_TOKEN_ID, u64::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    entrypoints.add_entry_point(
        EntryPoint::new(
            ENDPOINT_NFT_SET_NFT_CONTRACT_HASH,
            vec![
                Parameter::new(ARG_NFT_CONTRACT_HASH, ContractHash::cl_type()),
            ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract
        )
    );

    let mut nft_named_keys = NamedKeys::new();

    let (contract_hash, version) = storage::new_contract(
        entrypoints,
        Some(nft_named_keys),
        Some(KEY_NFT_CONTRACT_PACKAGE_NAME.to_string()),
        Some(KEY_NFT_CONTRACT_ACCESS_UREF.to_string()),
    );

    let contract_hash_uref = storage::new_uref(contract_hash);
    runtime::put_key(KEY_NFT_CONTRACT_HASH, contract_hash_uref.into());

    let contract_version_uref = storage::new_uref(version);
    runtime::put_key(KEY_NFT_CONTRACT_VERSION, contract_version_uref.into());

}

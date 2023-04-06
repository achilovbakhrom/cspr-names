#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod storage_db;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractHash, Key, runtime_args};
use common_lib::constants::{ARG_NFT_TOKEN_ID, ARG_NFT_TOKEN_OWNER, ENDPOINT_NFT_BURN, ENDPOINT_NFT_MINT, ENDPOINT_NFT_TRANSFER};
use common_lib::errors::NFTErrors;
use crate::storage_db::StorageDb;

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
    // check caller, whether sc or not
    // mint
    let token_owner = runtime::get_named_arg::<Key>(ARG_NFT_TOKEN_OWNER);
    let token_id: String = runtime::get_named_arg(ARG_NFT_TOKEN_ID);
    let nft_core_contract_hash: ContractHash = StorageDb::instance()
        .get_nft_core_contract_hash()
        .unwrap_or_revert_with(NFTErrors::NFTCoreHashIsNotSet);

    runtime::call_contract::<(String, Key, String)>(
        nft_core_contract_hash,
        ENDPOINT_NFT_MINT,
        runtime_args! {
            ARG_TOKEN_OWNER => token_owner,
            ARG_TOKEN_META_DATA => token_id,
        },
    );

}

#[no_mangle]
pub extern "C" fn transfer() {
    // check whether owner or not
    // extract fee and transfer
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
    // check whether owner or not
    // burn
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
    // check whether owner or not
    // list
}

#[no_mangle]
pub extern "C" fn un_list() {
    // check whether owner or not
    // unlist
}

#[no_mangle]
pub extern "C" fn buy() {
    // check whether owner or not
    // list
}

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

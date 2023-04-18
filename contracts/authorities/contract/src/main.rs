#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod authority_db;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractHash, Key};
use common_lib::constants::{ARG_AUTHORITY_AUTHORITY_LIST, ARG_AUTHORITY_CONTRACT_HASH, ARG_AUTHORITY_MUTATION_TYPE};
use common_lib::enums::mutation_type::MutationType;

pub extern "C" fn set_authority() {
    let operation_type = runtime::get_named_arg::<MutationType>(ARG_AUTHORITY_MUTATION_TYPE);
    let contract_hash = runtime::get_named_arg::<ContractHash>(ARG_AUTHORITY_CONTRACT_HASH);
    let authority_list = runtime::get_named_arg::<Vec<Key>>(ARG_AUTHORITY_AUTHORITY_LIST);




}


pub extern "C" fn get_authority() {

}


pub extern "C" fn get_contract() {

}

pub extern "C" fn set_contract() {

}

pub extern "C" fn init() {

}

#[no_mangle]
pub extern "C" fn call() {

}

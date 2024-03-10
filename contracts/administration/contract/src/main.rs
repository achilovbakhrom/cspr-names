#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
	"target arch should be wasm32: compile with '--target wasm32-unknown-unknown'"
);

extern crate alloc;

mod db;
mod types;
mod service;
mod utils;

use alloc::{
	string::{ ToString, String },
	vec::Vec,
	vec,
	borrow::ToOwned,
	boxed::Box,
};

use casper_contract::contract_api::storage;
use casper_types::{
	CLType,
	EntryPointAccess,
	EntryPointType,
	Parameter,
	CLTyped,
	ContractHash,
	EntryPoints,
	contracts::NamedKeys,
};
use common_lib::{
	constants::common_keys::{
		AdministractionStoreKeys,
		AdministrationArgs,
		AdministrationEndpoints,
		CommonArgs,
		CommonEndpoints,
	},
	enums::{ contracts_enum::ContractKind, controller_roles::ControllerRoles },
	utils::{
		contract::{ create_entrypoint, setup_contract_info },
		response::controller,
	},
};

/// Authorities for administration contract
use common_lib::controllers::authorities;

/// Others Contracts Authority endpoints
#[no_mangle]
pub extern "C" fn set_contract_authority_list() {
	controller(
		service::contract_authority::set_contract_autority_list,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

#[no_mangle]
pub extern "C" fn add_contract_authority() {
	controller(
		service::contract_authority::add_contract_authority,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

#[no_mangle]
pub extern "C" fn get_contract_authority_list() {
	controller(
		service::contract_authority::get_contract_authority_list,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

#[no_mangle]
pub extern "C" fn remove_contract_authority() {
	controller(
		service::contract_authority::remove_contract_authority,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

/// List of Contracts endpoints
#[no_mangle]
pub extern "C" fn get_contract() {
	controller(
		service::contracts::get_contract,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

#[no_mangle]
pub extern "C" fn add_contract() {
	controller(
		service::contracts::add_contract,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

#[no_mangle]
pub extern "C" fn increment_contract() {
	controller(
		service::contracts::increment_contract,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

#[no_mangle]
pub extern "C" fn decrement_contract() {
	controller(
		service::contracts::decrement_contract,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

/// Extension Endpoints (.cspr, .com ....)
#[no_mangle]
pub extern "C" fn set_allowed_extensions() {
	controller(
		service::extensions::set_allowed_extensions,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

#[no_mangle]
pub extern "C" fn get_allowed_extensions() {
	controller(
		service::extensions::get_allowed_extensions,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

#[no_mangle]
pub extern "C" fn add_extension() {
	controller(
		service::extensions::add_extension,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

#[no_mangle]
pub extern "C" fn remove_extension() {
	controller(
		service::extensions::remove_extension,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

/// Limits Endpoints (min chars count for extension: .cspr - 3, ...)
#[no_mangle]
pub extern "C" fn get_chars_min_count() {
	controller(
		service::limits::get_chars_min_count,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

#[no_mangle]
pub extern "C" fn set_chars_min_count() {
	controller(
		service::limits::set_chars_min_count,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

/// Database and NFT contract items limit .... (10 000 items for example can hold)
#[no_mangle]
pub extern "C" fn get_listing_limit() {
	controller(
		service::limits::get_listing_limit,
		vec![
			ControllerRoles::OnlyLocalOperators,
			ControllerRoles::OnlyAuthorizedContracts
		]
	)
}

#[no_mangle]
pub extern "C" fn set_listing_limit() {
	controller(
		service::limits::set_listing_limit,
		vec![ControllerRoles::OnlyLocalOperators]
	)
}

/// Endpoints
/// 1. set_authority_list
/// 2. add_contract_authority
/// 3. get_contract_authority_list
/// 4. get_contract
/// 5. add_contract
/// 5. increment_contract
/// 6. decrement_contract
/// 7. set_allowed_extensions
/// 8. get_allowed_extensions
/// 9. add_extension
/// 10. remove_extension
/// 11. get_chars_min_count
/// 12. set_chars_min_count
/// 13. get_listing_limit
/// 14. set_listing_limit

#[no_mangle]
pub extern "C" fn call() {
	use service::init::init;

	init()
}

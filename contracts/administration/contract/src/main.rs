#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
	"target arch should be wasm32: compile with '--target wasm32-unknown-unknown'"
);

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

mod db;
mod types;
mod service;
mod utils;

use alloc::{ string::{ ToString, String }, vec };

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
	utils::{
		response::controller,
		contract::{ create_entrypoint, setup_contract_info },
	},
	constants::common_keys::{ AdministrationEndpoints, AdministrationArgs },
	enums::contracts_enum::ContractKind,
};

/// Authorities endpoints
#[no_mangle]
pub extern "C" fn set_authority_list() {
	controller(service::authority::set_contract_autority_list)
}

#[no_mangle]
pub extern "C" fn add_contract_authority() {
	controller(service::authority::add_contract_authority)
}

#[no_mangle]
pub extern "C" fn get_contract_authority_list() {
	controller(service::authority::get_contract_authority_list)
}

/// Contracts endpoints
#[no_mangle]
pub extern "C" fn get_contract() {
	controller(service::contracts::get_contract)
}

#[no_mangle]
pub extern "C" fn add_contract() {
	controller(service::contracts::add_contract)
}

#[no_mangle]
pub extern "C" fn increment_contract() {
	controller(service::contracts::increment_contract)
}

#[no_mangle]
pub extern "C" fn decrement_contract() {
	controller(service::contracts::decrement_contract)
}

/// Extension Endpoints
#[no_mangle]
pub extern "C" fn set_allowed_extensions() {
	controller(service::extensions::set_allowed_extensions)
}

#[no_mangle]
pub extern "C" fn get_allowed_extensions() {
	controller(service::extensions::get_allowed_extensions)
}

#[no_mangle]
pub extern "C" fn add_extension() {
	controller(service::extensions::add_extension)
}

#[no_mangle]
pub extern "C" fn remove_extension() {
	controller(service::extensions::remove_extension)
}

/// Limits Endpoints
#[no_mangle]
pub extern "C" fn get_chars_min_count() {
	controller(service::limits::get_chars_min_count)
}

#[no_mangle]
pub extern "C" fn set_chars_min_count() {
	controller(service::limits::set_chars_min_count)
}

#[no_mangle]
pub extern "C" fn get_listing_limit() {
	controller(service::limits::get_listing_limit)
}

#[no_mangle]
pub extern "C" fn set_listing_limit() {
	controller(service::limits::set_listing_limit)
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
	let mut entrypoints = EntryPoints::new();
	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetAuthorityList.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					ContractHash::cl_type()
				),
				Parameter::new(
					&AdministrationArgs::ContractAuthorities.to_string(),
					CLType::Any
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::AddContractAuthority.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					CLType::Key
				),
				Parameter::new(
					&AdministrationArgs::ContractAuthority.to_string(),
					CLType::Key
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetContractAuthorityList.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					ContractHash::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(
					&AdministrationArgs::Extension.to_string(),
					Option::<String>::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::AddContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::Key.to_string(), CLType::Key)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::IncrementContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::Key.to_string(), CLType::Key)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::DecrementContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::Key.to_string(), CLType::Key)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetAllowedExtensions.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetAllowedExtensions.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::AddExtension.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::RemoveExtension.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetCharsMinCount.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetCharsMinCount.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::CharsCount.to_string(),
					CLType::String
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetListingLimit.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetListingLimit.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::CharsCount.to_string(), CLType::U32)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	let named_keys = NamedKeys::new();

	setup_contract_info(entrypoints, named_keys)
}

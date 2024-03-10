#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
	"target arch should be wasm32: compile with '--target wasm32-unknown-unknown'"
);

mod db;
mod service;
mod types;
mod utils;

mod price_fetcher;

extern crate alloc;

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

use common_lib::{
	constants::common_keys::{ CommonArgs, CommonKeys },
	enums::controller_roles::ControllerRoles,
	utils::{
		contract::{ create_entrypoint, setup_contract_info },
		response::{ controller, response_error, response_success },
	},
};
use common_lib::{
	constants::common_keys::{
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
use common_lib::{
	constants::common_keys::{
		ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS,
		ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS,
		KEY_PO_SIMPLE_OPERATIONS,
	},
	utils::maintainer::create_new_contract,
};

use common_lib::constants::common_keys::{
	ARG_PO_EXTENSION,
	ENDPOINT_PO_INIT,
	ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS,
	ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS,
	KEY_PO_SIMPLE_OPERATIONS,
};
use common_lib::utils::response::{ response_error, response_success };
use common_lib::{
	constants::common_keys::{
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
		registry::{ has_authority, is_maintainer },
		storage::{ get_stored_value_from_key, store_value_for_key },
	},
};

#[no_mangle]
pub extern "C" fn set_price() {
	controller(
		service::price_oracle::set_price,
		vec![ControllerRoles::OnlyAuthorizedCallers]
	);
}

#[no_mangle]
pub extern "C" fn get_price_simple_operations() {
	controller(
		service::price_oracle::get_price_simple_operations,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

#[no_mangle]
pub extern "C" fn set_price_simple_operations() {
	controller(
		service::price_oracle::set_price_simple_operations,
		vec![ControllerRoles::OnlyAuthorizedCallers]
	);
}

#[no_mangle]
pub extern "C" fn get_price() {
	controller(
		service::price_oracle::get_price,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

#[no_mangle]
pub extern "C" fn get_price_simple_operations() {
	controller(
		service::price_oracle::get_price_simple_operations,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

#[no_mangle]
pub extern "C" fn set_price_simple_operations() {
	controller(
		service::price_oracle::set_price_simple_operations,
		vec![ControllerRoles::OnlyAuthorizedCallers]
	);
}

#[no_mangle]
pub extern "C" fn call() {
	let mut entrypoints = EntryPoints::new();

	entrypoints.add_entry_point(
		create_entrypoint(
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
		)
	);
	entrypoints.add_entry_point(
		create_entrypoint(
			ENDPOINT_PO_GET_PRICE,
			vec![Parameter::new(ARG_PO_PRICE_TYPE_CHARS_COUNT, u8::cl_type())],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);
	entrypoints.add_entry_point(
		create_entrypoint(
			ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS,
			vec![],
			CLType::U512,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS,
			vec![Parameter::new(ARG_PO_PRICE, U512::cl_type())],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);
	entrypoints.add_entry_point(
		create_entrypoint(
			ENDPOINT_PO_REMOVE_AUTHORITY,
			vec![Parameter::new(ARG_PO_AUTHORITY, AccountHash::cl_type())],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	let mut price_oralce_named_keys = NamedKeys::new();

	let administration_contract_hash: ContractHash = runtime::get_named_arg(
		&CommonArgs::AdministrationContract.to_string()
	);
	let maintainer_uref = storage::new_uref(administration_contract_hash);

	price_oralce_named_keys.insert(
		&CommonKeys::AdministrationContract.to_string(),
		maintainer_uref.into()
	);

	setup_contract_info(entrypoints, price_oralce_named_keys);
}

// Helpers
fn is_maintainer_or_has_authority(account: &AccountHash) -> bool {
	let caller_has_authority = has_authority(KEY_PO_AUTHORITIES, &account);
	let is_maintainer = is_maintainer(KEY_PO_MAINTAINER, &account);

	is_maintainer || caller_has_authority
}

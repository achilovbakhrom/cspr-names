#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
	"target arch should be wasm32: compile with '--target wasm32-unknown-unknown'"
);

extern crate alloc;

mod db;
mod service;
mod types;

use alloc::string::{ String, ToString };
use alloc::vec;

use casper_types::{
	CLType,
	CLTyped,
	ContractHash,
	EntryPointAccess,
	EntryPointType,
	EntryPoints,
	Key,
	Parameter,
};
use common_lib::enums::controller_roles::ControllerRoles;
use common_lib::utils::contract::{ create_entrypoint, setup_contract_info };

use casper_types::contracts::NamedKeys;
use common_lib::constants::common_keys::{
	NFTContractEndpoints,
	NFTContractArgs,
	CommonEndpoints,
	CommonArgs,
};

use common_lib::utils::response::controller;

use crate::service::nft_operations as nft_service;

#[allow(unused_imports)]
use common_lib::controllers::authorities;

#[no_mangle]
pub extern "C" fn mint() {
	controller(nft_service::mint, vec![ControllerRoles::OnlyAuthorizedContracts])
}

#[no_mangle]
pub extern "C" fn transfer() {
	controller(
		nft_service::transfer,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn burn() {
	controller(nft_service::burn, vec![ControllerRoles::OnlyAuthorizedContracts])
}

#[no_mangle]
pub extern "C" fn list() {
	controller(nft_service::list, vec![ControllerRoles::OnlyAuthorizedContracts])
}

#[no_mangle]
pub extern "C" fn un_list() {
	controller(
		nft_service::un_list,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn buy() {
	controller(nft_service::buy, vec![ControllerRoles::OnlyAuthorizedContracts])
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
		create_entrypoint(
			&NFTContractEndpoints::Mint.to_string(),
			vec![
				Parameter::new(
					&NFTContractArgs::NftCoreContractHash.to_string(),
					ContractHash::cl_type()
				),
				Parameter::new(&NFTContractArgs::Owner.to_string(), Key::cl_type()),
				Parameter::new(
					&NFTContractArgs::Metadata.to_string(),
					String::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&NFTContractEndpoints::Burn.to_string(),
			vec![
				Parameter::new(
					&NFTContractArgs::NftCoreContractHash.to_string(),
					ContractHash::cl_type()
				),
				Parameter::new(&NFTContractArgs::TokenId.to_string(), String::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&NFTContractEndpoints::Burn.to_string(),
			vec![
				Parameter::new(
					&NFTContractArgs::NftCoreContractHash.to_string(),
					ContractHash::cl_type()
				),
				Parameter::new(
					&NFTContractArgs::TokenId.to_string(),
					String::cl_type()
				),
				Parameter::new(&NFTContractArgs::SourceKey.to_string(), Key::cl_type()),
				Parameter::new(
					&NFTContractArgs::DestinationKey.to_string(),
					Key::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&NFTContractEndpoints::List.to_string(),
			vec![
				Parameter::new(&NFTContractArgs::TokenId.to_string(), String::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&NFTContractEndpoints::UnList.to_string(),
			vec![
				Parameter::new(&NFTContractArgs::TokenId.to_string(), String::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&NFTContractEndpoints::Buy.to_string(),
			vec![
				Parameter::new(&NFTContractArgs::TokenId.to_string(), String::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::SetAuthorities.to_string(),
			vec![Parameter::new(&CommonArgs::Authorities.to_string(), CLType::Any)],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::AddAuthority.to_string(),
			vec![Parameter::new(&CommonArgs::Authority.to_string(), CLType::Key)],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::RemoveAuthority.to_string(),
			vec![Parameter::new(&CommonArgs::Authority.to_string(), CLType::Key)],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::GetAuthorities.to_string(),
			vec![],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	let named_keys = NamedKeys::new();

	setup_contract_info(entrypoints, named_keys)
}

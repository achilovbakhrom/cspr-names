use alloc::{ vec::Vec, string::ToString };
use casper_contract::contract_api::runtime;
use casper_types::{ Key, ContractHash };
use common_lib::constants::common_keys::AdministrationArgs;

use crate::{
	types::TResult,
	db::contract_authorities::{ ContractAuthoritiesStore, ContractAuthorities },
};

pub fn set_contract_autority_list() -> TResult<()> {
	let contract_hash: ContractHash = runtime::get_named_arg(
		&AdministrationArgs::ContractHash.to_string()
	);
	let contract_authorities: Vec<Key> = runtime::get_named_arg(
		&AdministrationArgs::ContractAuthorities.to_string()
	);
	let store = ContractAuthoritiesStore::instance();
	store.set_contract_authority_list(contract_hash, contract_authorities);
	Ok(())
}

pub fn add_contract_authority() -> TResult<()> {
	let contract_hash: ContractHash = runtime::get_named_arg(
		&AdministrationArgs::ContractHash.to_string()
	);
	let contract_authority: Key = runtime::get_named_arg(
		&AdministrationArgs::ContractAuthority.to_string()
	);
	let store = ContractAuthoritiesStore::instance();
	store.add_contract_authority(contract_hash, contract_authority);
	Ok(())
}

pub fn get_contract_authority_list() -> TResult<Vec<Key>> {
	let contract_hash: ContractHash = runtime::get_named_arg(
		&AdministrationArgs::ContractHash.to_string()
	);
	let store = ContractAuthoritiesStore::instance();
	Ok(store.get_contract_authority(contract_hash))
}

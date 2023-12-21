use alloc::{ string::ToString, vec::Vec };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::Key;

use common_lib::{
	enums::contracts_enum::ContractKind,
	constants::common_keys::AdministrationArgs,
	db::store::Store,
	errors::AdministrationErrors,
	models::registry_pointer::CompoundContract,
};

use crate::{
	db::{ contract_hash_list::ContractHashList, domain_limit::DomainLimit },
	types::TResult,
};

use crate::utils::get_extension_arg;

/// Parameters:
/// - contract_kind - required
/// - extension - optional for simple contracts, otherwise is required

pub fn get_contract() -> TResult<Key> {
	let kind: ContractKind = runtime::get_named_arg(
		&AdministrationArgs::ContractKind.to_string()
	);
	let store = Store::instance();

	if is_compound(kind) {
		let extension = get_extension_arg().unwrap_or_revert();

		let keys = store.get_compound_contracts(kind, &extension);
		let limit = store.get_listing_limit(kind);

		if !keys.is_empty() {
			let filtered = keys
				.iter()
				.filter(|item| (**item).count.unwrap_or(0) < limit)
				.map(|item| *item)
				.collect::<Vec<CompoundContract>>();
			if filtered.is_empty() {
				return Err(AdministrationErrors::ContractIsFilled);
			}
			let first = filtered.first().unwrap();
			Ok(first.key)
		} else {
			return Err(AdministrationErrors::ContractNotFound);
		}
	} else {
		let contract_key = store
			.get_simple_contract(kind)
			.unwrap_or_revert_with(AdministrationErrors::ContractNotFound);

		Ok(contract_key)
	}
}

/// Parameters:
/// - contract_kind: required
/// - key - required
/// - extension - optional for simple contracts, otherwise is required

pub fn add_contract() -> TResult<()> {
	let kind: ContractKind = runtime::get_named_arg(
		&AdministrationArgs::ContractKind.to_string()
	);
	let key: Key = runtime::get_named_arg(&AdministrationArgs::Key.to_string());
	let store = Store::instance();

	if is_compound(kind) {
		let extension = get_extension_arg().unwrap_or_revert();
		store.add_compound_contract(kind, &extension, CompoundContract {
			key: key,
			count: Some(0),
		});
	} else {
		store.set_simple_contract(kind, key);
	}
	Ok(())
}

/// Parameters:
/// - contract_kind: required
/// - key - required
/// - extension - optional for simple contracts, otherwise is required

pub fn increment_contract() -> TResult<()> {
	let kind: ContractKind = runtime::get_named_arg(
		&AdministrationArgs::ContractKind.to_string()
	);
	let key: Key = runtime::get_named_arg(&AdministrationArgs::Key.to_string());
	let store = Store::instance();

	if is_compound(kind) {
		let extension = get_extension_arg().unwrap_or_revert();
		store.change_count_of_compound_contracts(kind, &extension, key, 1);
	}

	Ok(())
}

/// Parameters:
/// - contract_kind: required
/// - key - required
/// - extension - optional for simple contracts, otherwise is required

pub fn decrement_contract() -> TResult<()> {
	let kind: ContractKind = runtime::get_named_arg(
		&AdministrationArgs::ContractKind.to_string()
	);
	let key: Key = runtime::get_named_arg(&AdministrationArgs::Key.to_string());
	let store = Store::instance();

	if is_compound(kind) {
		let extension = get_extension_arg().unwrap_or_revert();
		store.change_count_of_compound_contracts(kind, &extension, key, -1);
	}

	Ok(())
}

/// Helpers
fn is_compound(contract_kind: ContractKind) -> bool {
	[ContractKind::Database, ContractKind::NFTCore].contains(&contract_kind)
}

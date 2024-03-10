use alloc::{ string::ToString, vec::Vec };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ ContractHash, Key };

use common_lib::{
	constants::common_keys::AdministrationArgs,
	db::store::Store,
	enums::{
		caller_verification_type::CallerVerificationType,
		contracts_enum::ContractKind,
	},
	errors::{ AdministrationErrors, CommonError },
	models::registry_pointer::CompoundContract,
	utils::{
		authority::ensure_caller_has_permission,
		registry::get_verified_caller,
	},
};

use crate::{
	db::{ contract_hash_list::ContractHashList, domain_limit::DomainLimit },
	types::TResult,
};

use crate::utils::get_extension_arg;

/// This service is closed, only authorized calls are passed (including contracts)
/// Parameters:
/// - contract_kind - required
/// - extension - optional for simple contracts, otherwise it is required

pub fn get_contract() -> TResult<(Key, Option<u32>)> {
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
			Ok((first.key, first.count))
		} else {
			return Err(AdministrationErrors::ContractNotFound);
		}
	} else {
		let contract_key = store
			.get_simple_contract(kind)
			.unwrap_or_revert_with(AdministrationErrors::ContractNotFound);

		Ok((contract_key, None))
	}
}

/// This service is closed, only authorized calls are passed
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

/// This service is closed, only authorized calls are passed (including contracts)
/// Parameters:
/// - contract_kind: required
/// - key - required
/// - extension - optional for simple contracts, otherwise is required

pub fn increment_contract() -> TResult<()> {
	ensure_caller_has_permission().unwrap();
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

/// This service is closed, only authorized calls are passed (including contracts)
/// Parameters:
/// - contract_kind: required
/// - key - required
/// - extension - optional for simple contracts, otherwise is required

pub fn decrement_contract() -> TResult<()> {
	ensure_caller_has_permission().unwrap();
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

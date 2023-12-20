use alloc::string::{ ToString, String };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};

use common_lib::{
	db::store::Store,
	constants::common_keys::AdministrationArgs,
	enums::contracts_enum::ContractKind,
};

use crate::{
	types::TResult,
	utils::get_extension_arg,
	db::domain_limit::DomainLimit,
};

/// Parameters:
/// - extension - required
pub fn get_chars_min_count() -> TResult<u8> {
	let extension: String = get_extension_arg().unwrap_or_revert();
	let store = Store::instance();
	Ok(store.get_chars_min_count(&extension))
}

/// Parameters:
/// - extension - required
/// - count - required
pub fn set_chars_min_count() -> TResult<()> {
	let count: u8 = runtime::get_named_arg(
		&AdministrationArgs::CharsCount.to_string()
	);
	let extension: String = get_extension_arg().unwrap_or_revert();
	let store = Store::instance();
	store.set_chars_min_count(&extension, count);
	Ok(())
}

/// Parameters:
/// - contract_kind - required

pub fn get_listing_limit() -> TResult<u32> {
	let kind: ContractKind = runtime::get_named_arg(
		&AdministrationArgs::ContractKind.to_string()
	);
	let store = Store::instance();
	Ok(store.get_listing_limit(kind))
}

/// Parameters:
/// - contract_kind - required
/// - value - required
pub fn set_listing_limit() -> TResult<()> {
	let kind: ContractKind = runtime::get_named_arg(
		&AdministrationArgs::ContractKind.to_string()
	);
	let value = runtime::get_named_arg(
		&AdministrationArgs::CharsCount.to_string()
	);
	let store = Store::instance();
	store.set_listing_limit(kind, value);
	Ok(())
}

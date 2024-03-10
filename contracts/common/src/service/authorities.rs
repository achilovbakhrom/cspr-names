use alloc::{ vec::Vec, string::ToString };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::Key;

use crate::{
	constants::common_keys::CommonArgs,
	db::store::Store,
	utils::authority::ensure_caller_has_permission,
	errors::CommonError,
	types::types::CResult,
};

pub fn set_authorites() -> CResult<()> {
	ensure_caller_has_permission().unwrap();
	let keys: Vec<Key> = runtime::get_named_arg(
		&CommonArgs::Authorities.to_string()
	);
	let store = Store::instance();

	store.set_authorities(keys);
	Ok(())
}

pub fn add_authority() -> CResult<()> {
	ensure_caller_has_permission().unwrap();
	let key: Key = runtime::get_named_arg(&CommonArgs::Authority.to_string());
	let store = Store::instance();
	store.add_authority(key);
	Ok(())
}

pub fn remove_authority() -> CResult<()> {
	ensure_caller_has_permission().unwrap();
	let key: Key = runtime::get_named_arg(&CommonArgs::Authority.to_string());
	if key == runtime::get_caller().into() {
		return Err(CommonError::InvalidKey);
	}
	let store = Store::instance();
	store.remove_authority(key);
	Ok(())
}

pub fn get_authorities() -> CResult<Vec<Key>> {
	ensure_caller_has_permission().unwrap();
	let store = Store::instance();
	Ok(store.get_authorities())
}

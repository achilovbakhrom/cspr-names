use alloc::{ vec::Vec, string::{ String, ToString } };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};
use common_lib::{
	db::store::Store,
	constants::common_keys::AdministrationArgs,
	utils::authority::ensure_caller_has_permission,
	errors::AdministrationErrors,
};

use crate::{ types::TResult, db::allowed_extensions::AllowedExtensions };

pub fn set_allowed_extensions() -> TResult<()> {
	ensure_caller_has_permission().unwrap();
	let extensions: Vec<String> = runtime::get_named_arg(
		&AdministrationArgs::AllowedExtensions.to_string()
	);

	let store = Store::instance();
	store.set_allowed_extensions(extensions);
	Ok(())
}

pub fn get_allowed_extensions() -> TResult<Vec<String>> {
	let store = Store::instance();
	Ok(store.get_allowed_extensions())
}

pub fn add_extension() -> TResult<()> {
	ensure_caller_has_permission().unwrap();
	let extension: String = runtime::get_named_arg(
		&AdministrationArgs::AllowedExtension.to_string()
	);
	let store = Store::instance();
	store.add_extension(extension);
	Ok(())
}

pub fn remove_extension() -> TResult<()> {
	ensure_caller_has_permission().unwrap();
	let extension: String = runtime::get_named_arg(
		&AdministrationArgs::AllowedExtension.to_string()
	);
	let store = Store::instance();
	store.remove_extension(extension);
	Ok(())
}

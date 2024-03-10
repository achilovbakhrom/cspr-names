use alloc::string::{ ToString, String };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::ContractHash;
use common_lib::{
	constants::common_keys::AdministrationArgs,
	enums::caller_verification_type::CallerVerificationType,
	errors::AdministrationErrors,
	utils::registry::get_verified_caller,
};

use crate::{
	db::contract_authorities::{ ContractAuthorities, ContractAuthoritiesStore },
	types::TResult,
};

pub fn get_extension_arg() -> TResult<String> {
	let arg_extension: Option<String> = runtime::get_named_arg(
		&AdministrationArgs::Extension.to_string()
	);

	Ok(
		arg_extension.unwrap_or_revert_with(
			AdministrationErrors::ProvideExtensionArgument
		)
	)
}

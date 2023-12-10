use alloc::string::ToString;
use casper_contract::contract_api::runtime;
use common_lib::{
	constants::common_keys::AdministrationArgs,
	errors::AdministrationErrors,
};

use crate::types::TResult;

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

pub fn has_permission() -> bool {
	false
}

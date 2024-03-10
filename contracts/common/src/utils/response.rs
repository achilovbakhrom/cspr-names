use alloc::vec::Vec;
use casper_contract::contract_api::runtime;
use casper_types::{ bytesrepr::ToBytes, ApiError, CLTyped, CLValue };

use crate::{
	constants::common_keys::DEFAULT_RESPONSE_ERROR_MESSAGE,
	enums::controller_roles::ControllerRoles,
	errors::CommonError,
};

use super::{
	authority::ensure_caller_has_permission,
	contract::ensure_caller_has_permission_external,
	maintainer::is_caller_maintainer,
};

pub fn response_success_default<T: ToBytes + CLTyped>(arg: T) {
	response_success(arg, DEFAULT_RESPONSE_ERROR_MESSAGE)
}

pub fn response_success<T: ToBytes + CLTyped>(arg: T, error_msg: &str) {
	let result = CLValue::from_t(arg).expect(error_msg);
	runtime::ret(result);
}

pub fn response_error<T: Into<ApiError>>(error: T) {
	runtime::revert(error);
}

pub fn controller<F, T, E>(handler: F, access: Vec<ControllerRoles>)
	where F: Fn() -> Result<T, E>, T: ToBytes + CLTyped, E: Into<ApiError>
{
	let mut passed = false;
	if access.contains(&ControllerRoles::OnlyMaintainer) {
		if !is_caller_maintainer() {
			response_error(CommonError::InvalidCaller);
			return;
		}
		passed = true;
	}

	if !passed && access.contains(&ControllerRoles::OnlyAuthorizedCallers) {
		ensure_caller_has_permission_external(None);
		passed = true;
	}

	if !passed && access.contains(&ControllerRoles::OnlyAuthorizedContracts) {
		ensure_caller_has_permission_external(Some(true));
		passed = true;
	}

	if !passed && access.contains(&ControllerRoles::OnlyLocalOperators) {
		ensure_caller_has_permission().unwrap();
	}

	match handler() {
		Ok(res) => { response_success_default(res) }
		Err(err) => { response_error(err) }
	}
}

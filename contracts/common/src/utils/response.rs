use casper_contract::contract_api::runtime;
use casper_types::{ bytesrepr::ToBytes, ApiError, CLTyped, CLValue };

use crate::constants::common_keys::DEFAULT_RESPONSE_ERROR_MESSAGE;

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

pub fn controller<F, T, E>(arg: F)
	where E: ApiError, F: Fn() -> Result<T: ToBytes + CLTyped, E>
{
	match arg {
		Ok(res) => { response_success_default(res) }
		Err(err) => { response_error(err) }
	}
}

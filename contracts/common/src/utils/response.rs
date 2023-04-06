use casper_contract::{
    contract_api::runtime,
};
use casper_types::{
    CLTyped,
    bytesrepr::ToBytes,
    CLValue,
    ApiError
};

pub fn response_success<T: ToBytes + CLTyped>(arg: T, error_msg: &str) {
    let result = CLValue::from_t(arg).expect(error_msg);
    runtime::ret(result);
}

pub fn response_error<T: Into<ApiError>>(error: T) {
    runtime::revert(error);
}

use crate::errors::CommonError;
use alloc::{ vec, vec::Vec };
use casper_contract::{
	contract_api::{ runtime, storage },
	ext_ffi,
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
	api_error,
	bytesrepr,
	bytesrepr::{ FromBytes, ToBytes },
	ApiError,
	CLTyped,
	Key,
	URef,
};
use core::convert::TryInto;

pub fn store_value_for_key<T: ToBytes + CLTyped>(name: &str, value: T) {
	match runtime::get_key(name) {
		Some(key) => {
			let key_ref = key.try_into().unwrap_or_revert();
			storage::write(key_ref, value);
		}
		None => {
			let key = storage::new_uref(value).into();
			runtime::put_key(name, key);
		}
	}
}

pub fn get_stored_value_from_key<T: CLTyped + FromBytes>(name: &str) -> Option<T> {
	match runtime::get_key(name) {
		Some(key) => {
			let key_ref = key.try_into().unwrap_or_revert();
			match storage::read(key_ref) {
				Ok(result) => result,
				Err(_) => None,
			}
		}
		None => None,
	}
}

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
	dictionary_name: &str,
	key: &str
) -> Option<T> {
	let seed_uref = get_uref(
		dictionary_name,
		CommonError::MissingStorageUref,
		CommonError::InvalidStorageUref
	);

	match storage::dictionary_get::<T>(seed_uref, key) {
		Ok(maybe_value) => maybe_value,
		Err(error) => runtime::revert(error),
	}
}

pub fn upsert_dictionary_value_from_key<T: CLTyped + FromBytes + ToBytes>(
	dictionary_name: &str,
	key: &str,
	value: T
) {
	let seed_uref = get_uref(
		dictionary_name,
		CommonError::MissingStorageUref,
		CommonError::InvalidStorageUref
	);

	match storage::dictionary_get::<T>(seed_uref, key) {
		Ok(None | Some(_)) => storage::dictionary_put(seed_uref, key, value),
		Err(error) => runtime::revert(error),
	}
}

pub fn get_uref(name: &str, missing: CommonError, invalid: CommonError) -> URef {
	let key = get_key_with_user_errors(name, missing, invalid);
	key.into_uref().unwrap_or_revert_with(CommonError::UnexpectedKeyVariant)
}

pub(crate) fn get_key_with_user_errors(
	name: &str,
	missing: CommonError,
	invalid: CommonError
) -> Key {
	let (name_ptr, name_size, _bytes) = to_ptr(name);
	let mut key_bytes = vec![0u8; Key::max_serialized_length()];
	let mut total_bytes: usize = 0;
	let ret = unsafe {
		ext_ffi::casper_get_key(
			name_ptr,
			name_size,
			key_bytes.as_mut_ptr(),
			key_bytes.len(),
			&mut total_bytes as *mut usize
		)
	};
	match api_error::result_from(ret) {
		Ok(_) => {}
		Err(ApiError::MissingKey) => runtime::revert(missing),
		Err(e) => runtime::revert(e),
	}
	key_bytes.truncate(total_bytes);

	bytesrepr::deserialize(key_bytes).unwrap_or_revert_with(invalid)
}

pub(crate) fn to_ptr<T: ToBytes>(t: T) -> (*const u8, usize, Vec<u8>) {
	let bytes = t.into_bytes().unwrap();
	let ptr = bytes.as_ptr();
	let size = bytes.len();
	(ptr, size, bytes)
}

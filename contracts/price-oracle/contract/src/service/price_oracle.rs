use alloc::string::ToString;
use casper_contract::contract_api::runtime;
use casper_types::U512;
use common_lib::{
	constants::common_keys::PriceOracleArgs,
	db::dictionary::Dictionary,
	enums::{
		caller_verification_type::CallerVerificationType,
		price_oracle_contract::PriceType,
	},
	errors::PriceOracleContractErrors,
	utils::{
		contract::ensure_caller_has_permission_external,
		registry::get_verified_caller,
	},
};

use crate::{
	types::PResult,
	db::price_oracle::PriceOracle,
	price_fetcher::PriceFetcher,
};

pub fn set_price() -> PResult<()> {
	ensure_caller_has_permission_external(Some(false));
	let mut db_instance = Dictionary::price_oracle_instance();
	let extension: String = runtime::get_named_arg(
		&PriceOracleArgs::Extension.to_string()
	);
	let price_type: PriceType = runtime::get_named_arg(
		&PriceOracleArgs::PriceType.to_string()
	);
	match price_type {
		PriceType::Fixed => {
			let price: U512 = runtime::get_named_arg(
				&PriceOracleArgs::Price.to_string()
			);
			db_instance.set_fixed_price(&extension, price);
		}
		PriceType::Dynamic => {
			let price: U512 = runtime::get_named_arg(
				&PriceOracleArgs::Price.to_string()
			);
			let price_mid: Vec<U512> = runtime::get_named_arg(
				&PriceOracleArgs::PriceMid.to_string()
			);
			let chars_count_mid: Vec<u64> = runtime::get_named_arg(
				&PriceOracleArgs::CharsCount.to_string()
			);
			if price_mid.len() != chars_count_mid.len() {
				return Err(
					PriceOracleContractErrors::PriceMidLengthAndMidCharsCountMismatch
				);
			}
			let price_more: U512 = runtime::get_named_arg(
				&PriceOracleArgs::PriceMore.to_string()
			);

			db_instance.set_dynamic_price(
				&extension,
				price,
				price_mid,
				chars_count_mid,
				price_more
			);
		}
	}
	Ok(())
}

pub fn get_price_simple_operations() -> PResult<U512> {
	ensure_caller_has_permission_external(None);
	let price_fetcher = PriceFetcher::instance();
	let price = price_fetcher.get_price_simple_operations();
	if price.is_none() {
		return Err(PriceOracleContractErrors::PriceSimpleOperationsIsNotSet);
	}
	Ok(price.unwrap())
}

pub fn set_price_simple_operations() -> PResult<()> {
	ensure_caller_has_permission_external(Some(false));
	let extension: String = runtime::get_named_arg(
		&PriceOracleArgs::Extension.to_string()
	);
	let price: U512 = runtime::get_named_arg(&PriceOracleArgs::Price.to_string());
	let db_instance = Dictionary::price_oracle_instance();
	db_instance.set_price_for_simple_operations(price);
	match price_type {
		PriceType::Fixed => {
			let price: U512 = runtime::get_named_arg(
				&PriceOracleArgs::Price.to_string()
			);
			db_instance.set_fixed_price(&extension, price);
		}
		PriceType::Dynamic => {
			let price: U512 = runtime::get_named_arg(
				&PriceOracleArgs::Price.to_string()
			);
			let price_mid: Vec<U512> = runtime::get_named_arg(
				&PriceOracleArgs::PriceMid.to_string()
			);
			let chars_count_mid: Vec<u64> = runtime::get_named_arg(
				&PriceOracleArgs::CharsCount.to_string()
			);
			if price_mid.len() != chars_count_mid.len() {
				return Err(
					PriceOracleContractErrors::PriceMidLengthAndMidCharsCountMismatch
				);
			}
			let price_more: U512 = runtime::get_named_arg(
				&PriceOracleArgs::PriceMore.to_string()
			);

			db_instance.set_dynamic_price(
				&extension,
				price,
				price_mid,
				chars_count_mid,
				price_more
			);
		}
	}
	Ok(())
}

pub fn get_price() -> PResult<U512> {
	ensure_caller_has_permission_external(None);
	let price_type = get_stored_value_from_key::<PriceType>(
		&PriceOracleArgs::PriceType.to_string()
	);
	if price_type.is_none() {
		return Err(PriceOracleContractErrors::PriceTypeIsNotFound);
	}
	let chars_count: u8 = runtime::get_named_arg(
		&PriceOracleArgs::CharsCount.to_string()
	);
	let extension: String = runtime::get_named_arg(
		&PriceOracleArgs::Extension.to_string()
	);

	let price_fetcher = PriceFetcher::instance();

	let price = price_fetcher.get_price_for(&extension, chars_count);

	match price_type.unwrap() {
		PriceType::Fixed => {
			let price = price_fetcher.get_fixed_price();
			if price.is_none() {
				response_error(PriceOracleContractErrors::PriceIsNotSet);
			}
			Ok(price.unwrap())
		}
		PriceType::Dynamic => {
			let chars_count: u64 = runtime::get_named_arg(
				&PriceOracleArgs::CharsCount.to_string()
			);
			let price = price_fetcher.get_price_dynamic(chars_count);
			if price.is_none() {
				return Err(PriceOracleContractErrors::PriceForCharsCountNotFound);
			}
			Ok(price.unwrap())
		}
	}
}

pub fn get_price_for_simple_operations() -> PResult<U512> {
	ensure_caller_has_permission_external(None);
	let price_fetcher = PriceFetcher::instance();
	let price = price_fetcher.get_price_simple_operations();
	if price.is_none() {
		return Err(PriceOracleContractErrors::PriceSimpleOperationsIsNotSet);
	}
	Ok(price.unwrap())
}

pub fn set_price_simple_operations() -> PResult<()> {
	ensure_caller_has_permission_external(Some(false));
	let price: U512 = runtime::get_named_arg(&PriceOracleArgs::Price.to_string());
	let db_instance = Dictionary::price_oracle_instance();
	db_instance.set_price_for_simple_operations(price);
	Ok(())
}

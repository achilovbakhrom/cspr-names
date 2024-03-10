use alloc::string::ToString;
use common_lib::constants::common_keys::PriceOracleKeys;
use common_lib::utils::helpers::concat;

pub(crate) fn get_price_type_key(ext: &str) -> String {
	concat(ext, "_", &PriceOracleKeys::PriceType.to_string())
}

pub(crate) fn get_price_key(ext: &str) -> String {
	concat(ext, "_", &PriceOracleKeys::Price.to_string())
}

pub(crate) fn get_price_mid_key(ext: &str) -> String {
	concat(ext, "_", &PriceOracleKeys::PriceMid.to_string())
}

pub(crate) fn get_chars_count_mid_key(ext: &str) -> String {
	concat(ext, "_", &PriceOracleKeys::CharsCount.to_string())
}

pub(crate) fn get_price_more_key(ext: &str) -> String {
	concat(ext, "_", &PriceOracleKeys::PriceMore.to_string())
}

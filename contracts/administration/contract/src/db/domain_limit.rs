use alloc::{ string::{ ToString, String }, format, fmt::format };
use common_lib::{
	constants::common_keys::AdministractionStoreKeys,
	db::{ store::Store, traits::Storable },
	enums::contracts_enum::ContractKind,
};

use crate::types::TResult;

const DEFAULT_DOMAIN_CHARS_COUNT: u8 = 3u8;
const DEFAULT_DOMAIN_LIST_LENGTH: u16 = 1000u16;

/**
 * 1. Domain chars min count
 * 2. Domain list limit (for Database Contract) - defines database contract's capacity.
 */
pub(crate) trait DomainLimit {
	fn get_chars_min_count(&self, extension: &str) -> u8;
	fn set_chars_min_count(&self, extension: &str, count: u8) -> ();
	fn get_listing_limit(&self, kind: ContractKind) -> u16;
	fn set_listing_limit(&self, kind: ContractKind, limit: u16) -> ();
}

fn to_domain_list_limit_key(kind: &ContractKind) -> String {
	let key = AdministractionStoreKeys::DomainListLimit.to_string();
	format!("{}:{}", key, kind)
}

impl DomainLimit for Store {
	fn get_chars_min_count(&self, extension: &str) -> u8 {
		let key = AdministractionStoreKeys::CharsCount.to_string();
		self.get::<u8>(&key).unwrap_or(DEFAULT_DOMAIN_CHARS_COUNT)
	}

	fn set_chars_min_count(&self, extension: &str, count: u8) -> () {
		let key = AdministractionStoreKeys::CharsCount.to_string();
		self.set(&key, count);
	}

	fn get_listing_limit(&self, kind: ContractKind) -> u16 {
		let key = to_domain_list_limit_key(&kind);
		self.get::<u16>(&key).unwrap_or(DEFAULT_DOMAIN_LIST_LENGTH)
	}

	fn set_listing_limit(&self, kind: ContractKind, limit: u16) -> () {
		let key = to_domain_list_limit_key(&kind);
		self.set(&key, limit);
	}
}

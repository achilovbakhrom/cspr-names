use alloc::string::{ String, ToString };
use casper_types::U512;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use common_lib::constants::common_keys::NFTContractKeys;

pub trait Listing {
	fn listing_initialize() -> ();
	fn listing_instance() -> Self;
	fn list(&mut self, token_id: String, price: U512) -> ();
	fn un_list(&mut self, token_id: String) -> ();
	fn is_listed(&self, token_id: String) -> bool;
	fn get_price(&self, token_id: String) -> Option<U512>;
}

impl Listing for Dictionary {
	fn listing_initialize() -> () {
		Dictionary::init(&NFTContractKeys::Listing.to_string())
	}

	fn listing_instance() -> Self {
		Dictionary::instance(&NFTContractKeys::Listing.to_string())
	}

	fn list(&mut self, token_id: String, price: U512) -> () {
		self.set(&token_id, price)
	}

	fn un_list(&mut self, token_id: String) -> () {
		self.remove::<U512>(&token_id)
	}

	fn is_listed(&self, token_id: String) -> bool {
		self.get::<U512>(&token_id).is_some()
	}

	fn get_price(&self, token_id: String) -> Option<U512> {
		self.get::<U512>(&token_id)
	}
}

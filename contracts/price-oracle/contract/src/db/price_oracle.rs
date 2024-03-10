use alloc::string::{ String, ToString };
use alloc::vec::Vec;
use casper_types::U512;
use common_lib::constants::common_keys::PriceOracleKeys;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use common_lib::enums::price_oracle_contract::PriceType;
use common_lib::models::price::{ Price, PriceItem };
use common_lib::utils::helpers::concat;

pub trait PriceOracle {
	fn price_oracle_initialize() -> ();
	fn price_oracle_instance() -> Self;
	fn set_fixed_price(&self, extension: &str, price: U512) -> ();
	fn set_dynamic_price(
		&self,
		extension: &str,
		price: U512,
		price_mid: Vec<U512>,
		chars_count_mid: Vec<u64>,
		price_more: U512
	) -> ();

	fn get_price_for(&self, extension: &str) -> Option<Price>;
	fn set_price_for_simple_operations(&self, price: U512) -> ();
	fn get_price_simple_operations(&self) -> Option<U512>;
}

impl PriceOracle for Dictionary {
	fn price_oracle_initialize() -> () {
		Dictionary::init(&PriceOracleKeys::Main.to_string())
	}

	fn price_oracle_instance() -> Self {
		Dictionary::instance(&PriceOracleKeys::Main.to_string())
	}

	fn set_fixed_price(&self, extension: &str, price: U512) -> () {
		let mut price_obj = Price::default();
		price_obj.price_type = PriceType::Fixed;
		price_obj.price = price;
		self.set(extension, price_obj);
	}

	fn set_dynamic_price(
		&self,
		extension: &str,
		price: U512,
		price_mid: Vec<U512>,
		chars_count_mid: Vec<u64>,
		price_more: U512
	) -> () {
		let mut price_obj = Price::default();
		price_obj.price_type = PriceType::Dynamic;
		price_obj.price = price;
		let price_by_count = price_mid
			.iter()
			.zip(chars_count_mid.iter())
			.map(|(price, char_count)| PriceItem {
				price: price.clone(),
				char_count: *char_count as u8,
			})
			.collect::<Vec<PriceItem>>();

		price_obj.price_by_count = price_by_count;
		price_obj.price_more = price_more;
		self.set(extension, price_obj);
	}

	fn get_price_for(&self, extension: &str) -> Option<Price> {
		self.get(extension)
	}

	fn set_price_for_simple_operations(&self, price: U512) -> () {
		self.set(&PriceOracleKeys::SimpleOperations.to_string(), price)
	}

	fn get_price_simple_operations(&self) -> Option<U512> {
		self.get(&PriceOracleKeys::SimpleOperations.to_string())
	}
}

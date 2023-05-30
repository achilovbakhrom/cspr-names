use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use casper_types::U512;
use common_lib::constants::{
    KEY_PO_CHARS_COUNT_MID, KEY_PO_PRICE, KEY_PO_PRICES, KEY_PO_PRICE_MID, KEY_PO_PRICE_MORE,
    KEY_PO_PRICE_TYPE, KEY_PO_SIMPLE_OPERATIONS,
};
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use common_lib::enums::price_oracle_contract::PriceType;
use common_lib::models::price::{Price, PriceItem};
use common_lib::utils::helpers::concat;

pub struct PriceOracleDb {
    store: Dictionary,
}

fn get_price_type_key(ext: &str) -> String {
    concat(ext, "_", KEY_PO_PRICE_TYPE)
}

fn get_price_key(ext: &str) -> String {
    concat(ext, "_", KEY_PO_PRICE)
}

fn get_price_mid_key(ext: &str) -> String {
    concat(ext, "_", KEY_PO_PRICE_MID)
}

fn get_chars_count_mid_key(ext: &str) -> String {
    concat(ext, "_", KEY_PO_CHARS_COUNT_MID)
}

fn get_price_more_key(ext: &str) -> String {
    concat(ext, "_", KEY_PO_PRICE_MORE)
}

impl PriceOracleDb {
    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_PO_PRICES),
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_PO_PRICES)
    }

    pub fn set_fixed_price(&self, extension: &str, price: U512) {
        let mut price_obj = Price::default();
        price_obj.price_type = PriceType::Fixed;
        price_obj.price = price;
        self.store.set(extension, price_obj);
    }

    pub fn set_dynamic_price(
        &self,
        extension: &str,
        price: U512,
        price_mid: Vec<U512>,
        chars_count_mid: Vec<u64>,
        price_more: U512,
    ) {
`        let mut price_obj = Price::default();
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
        self.store.set(extension, price_obj);
    }

    pub fn get_price_for(&self, extension: &str) -> Option<Price> {
        self.store.get(extension)
    }

    pub fn set_price_for_simple_operations(&self, price: U512) {
        self.store.set(KEY_PO_SIMPLE_OPERATIONS, price)
    }

    pub fn get_price_simple_operations(&self) -> Option<U512> {
        self.store.get(KEY_PO_SIMPLE_OPERATIONS)
    }
}

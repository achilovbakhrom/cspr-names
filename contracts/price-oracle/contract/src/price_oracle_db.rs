use alloc::vec::Vec;
use casper_types::U512;
use common_lib::constants::{
    KEY_PO_CHARS_COUNT_MID,
    KEY_PO_PRICE,
    KEY_PO_PRICE_MID,
    KEY_PO_PRICE_MORE,
    KEY_PO_PRICE_TYPE,
    KEY_PO_SIMPLE_OPERATIONS,
};
use common_lib::db::store::Store;
use common_lib::db::traits::Storable;
use common_lib::enums::price_oracle_contract::PriceType;

pub struct PriceOracleDb {
    state: Store
}

impl PriceOracleDb {

    pub fn instance() -> Self {
        Self {
            state: Store::instance()
        }
    }

    pub fn set_fixed_price(&self, price: U512) {
        self.state.set(KEY_PO_PRICE_TYPE, PriceType::Fixed);
        self.state.set(KEY_PO_PRICE, price);
    }

    pub fn set_dynamic_price(
        &self,
        price: U512,
        price_mid: Vec<U512>,
        chars_count_mid: Vec<u64>,
        price_more: U512
    ) {
        self.state.set(KEY_PO_PRICE_TYPE, PriceType::Dynamic);
        self.state.set(KEY_PO_PRICE, price);
        self.state.set(KEY_PO_PRICE_MID, price_mid);
        self.state.set(KEY_PO_CHARS_COUNT_MID, chars_count_mid);
        self.state.set(KEY_PO_PRICE_MORE, price_more);
    }

    pub fn set_simple_operations_price(&self, price: U512) {
        self.state.set(KEY_PO_SIMPLE_OPERATIONS, price);
    }

    pub fn get_price_type(&self) -> Option<PriceType> {
        self.state.get(KEY_PO_PRICE_TYPE)
    }

    pub fn get_price(&self) -> Option<U512> {
        self.state.get(KEY_PO_PRICE)
    }

    pub fn get_price_mid(&self) -> Option<Vec<U512>> {
        self.state.get(KEY_PO_PRICE_MID)
    }

    pub fn get_chars_count_mid(&self) -> Option<Vec<u64>> {
        self.state.get(KEY_PO_CHARS_COUNT_MID)
    }

    pub fn get_price_more(&self) -> Option<U512> {
        self.state.get(KEY_PO_PRICE_MORE)
    }

    pub fn get_price_simple_operations(&self) -> Option<U512> {
        self.state.get(KEY_PO_SIMPLE_OPERATIONS)
    }


}
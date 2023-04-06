use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::U512;
use common_lib::errors::PriceOracleContractErrors;
use crate::price_oracle_db::PriceOracleDb;

pub struct PriceFetcher {
    db: PriceOracleDb
}

impl PriceFetcher {

    pub fn instance() -> Self {
        Self {
            db: PriceOracleDb::instance()
        }
    }

    pub fn get_fixed_price(&self) -> Option<U512> {
        self.db.get_price()
    }

    pub fn get_price_simple_operations(&self) -> Option<U512> {
        self.db.get_price_simple_operations()
    }

    pub fn get_price_dynamic(&self, char_count: u64) -> Option<U512> {
        let chars_count_mid = self.db.get_chars_count_mid().unwrap_or_revert_with(
            PriceOracleContractErrors::PriceForCharsCountNotFound
        );

        let first_item = &chars_count_mid.first().expect("Error while getting the first object of chars_count_vec");
        let last_item = &chars_count_mid.last().expect("Error while getting the last object of chars_count_vec");

        return if *first_item > &char_count {
            self.db.get_price()
        } else if *last_item < &char_count {
            self.db.get_price_more()
        } else {
            match chars_count_mid.clone().iter().position(|item| { item == &char_count }) {
                Some(index) => {
                    let price_mid = self.db.get_price_mid()
                        .unwrap_or_revert_with(PriceOracleContractErrors::PriceMidIsNotSet);
                    let price = price_mid.get(index).expect("Error while getting price from price_mid by index");
                    Some(*price)
                },
                None => {
                    None
                }
            }
        }
    }

}
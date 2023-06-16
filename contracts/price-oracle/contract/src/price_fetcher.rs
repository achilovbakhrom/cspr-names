use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::U512;
use common_lib::enums::price_oracle_contract::PriceType;
use common_lib::errors::PriceOracleContractErrors;
use crate::price_oracle_db::PriceOracleDb;

pub struct PriceFetcher {
    db: PriceOracleDb,
}

impl PriceFetcher {
    pub fn instance() -> Self {
        Self {
            db: PriceOracleDb::instance(),
        }
    }

    pub fn get_price_for(&self, extension: &str, char_count: u8) -> Option<U512> {
        let price_obj = self.db.get_price_for(extension);
        if let Some(price) = price_obj {
            return match *&price.price_type {
                PriceType::Fixed => { Some(price.price) }
                PriceType::Dynamic => {
                    let found = &price.price_by_count
                        .iter()
                        .find(|item| item.char_count == char_count);

                    if found.is_some() {
                        let price_item = found.unwrap();
                        return Some(price_item.price);
                    } else if !price.price_by_count.is_empty() {
                        let first = price.price_by_count.first().unwrap();
                        let last = price.price_by_count.last().unwrap();

                        if char_count < first.char_count {
                            return Some(price.price);
                        } else if char_count > last.char_count {
                            return Some(price.price_more);
                        }
                    }
                    None
                }
            };
        }

        None
    }

    pub fn get_price_simple_operations(&self) -> Option<U512> {
        self.db.get_price_simple_operations()
    }
}

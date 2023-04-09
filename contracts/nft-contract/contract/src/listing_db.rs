use alloc::format;
use alloc::string::{String, ToString};
use common_lib::constants::KEY_NFT_DICTIONARY_LISTING;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;

pub struct ListingDb {
    store: Dictionary
}

impl ListingDb {

    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_NFT_DICTIONARY_LISTING),
        }
    }

    pub fn list(&mut self, token_id: u64) {
        self.store.set(&token_id.to_string(), true);
    }

    pub fn un_list(&mut self, token_id: u64) {
        self.store.set(&token_id.to_string(), false);
    }

    pub fn is_listed(&self, token_id: u64) -> bool {
        return match self.store.get::<bool>(&token_id.to_string()) {
            Some(res) => res,
            None => false
        }
    }

}
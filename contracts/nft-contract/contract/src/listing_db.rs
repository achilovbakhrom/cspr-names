use alloc::string::String;
use casper_types::U512;
use common_lib::constants::common_keys::KEY_NFT_DICTIONARY_LISTING;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;

pub struct ListingDb {
    store: Dictionary,
}

impl ListingDb {
    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_NFT_DICTIONARY_LISTING),
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_NFT_DICTIONARY_LISTING);
    }

    pub fn list(&mut self, token_id: String, price: U512) {
        self.store.set(&token_id, price);
    }

    pub fn un_list(&mut self, token_id: String) {
        self.store.remove::<U512>(&token_id);
    }

    pub fn is_listed(&self, token_id: String) -> bool {
        return match self.store.get::<bool>(&token_id) {
            Some(res) => res,
            None => false,
        };
    }

    pub fn get_price(&self, token_id: String) -> Option<U512> {
        self.store.get(&token_id)
    }
}

use alloc::string::String;
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

    pub fn list(&mut self, token_id: &str) {
        self.store.set(token_id, true);
    }

    pub fn un_list(&mut self, token_id: &str) {
        self.store.set(token_id, false);
    }

    pub fn is_listed(&self, token_id: &str) -> bool {
        return match self.store.get::<bool>(token_id) {
            Some(res) => res,
            None => false
        }
    }

}
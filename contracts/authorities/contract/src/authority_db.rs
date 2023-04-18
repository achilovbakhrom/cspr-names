use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use casper_types::{ContractHash, Key};
use common_lib::constants::KEY_AUTHORITY_AUTHORITIES;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::dictionary_key_list::DictionaryKeyList;
use common_lib::db::traits::Storable;

pub struct AuthorityDb {
    pub store: DictionaryKeyList,
}

impl AuthorityDb {
    pub fn instance() -> Self {
        Self {
            store: DictionaryKeyList::instance(KEY_AUTHORITY_AUTHORITIES)
        }
    }

    pub fn initialize() {
        DictionaryKeyList::initialize(KEY_AUTHORITY_AUTHORITIES)
    }

    pub fn get_authority_list(&self, contract_hash: ContractHash) -> Option<Vec<Key>> {
        self.store.get(&contract_hash.to_string())
    }

    pub fn set_authority_list(&self, contract_hash: ContractHash, list: Vec<Key>) {
        self.store.set(
            &contract_hash.to_string(),
            list
        )
    }

    pub fn add_authority_list(&self, contract_hash: ContractHash, list: Vec<Key>) {
        self.store.append(&contract_hash.to_string(), list)
    }

    pub fn remove_authority(&self, contract_hash: ContractHash, key: Key) {
        self.store.remove(&contract_hash.to_string(), key)
    }


}
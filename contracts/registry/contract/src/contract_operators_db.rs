use alloc::{
    string::ToString,
    vec::{self, Vec},
};
use casper_types::{ContractHash, Key};
use common_lib::{
    constants::KEY_REGISTRY_CONTRACT_OPERATORS,
    db::{dictionary::Dictionary, traits::Storable},
};

pub(crate) struct ContractOperatorsDb {
    store: Dictionary,
}

impl ContractOperatorsDb {
    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_REGISTRY_CONTRACT_OPERATORS),
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_REGISTRY_CONTRACT_OPERATORS)
    }

    pub fn add_operator(&self, contract_hash: ContractHash, key: Key) {
        let mut list: Vec<Key> = match self.store.get(&contract_hash.to_string()) {
            Some(res) => res,
            None => {
                let res = Vec::<Key>::new();
                self.store.set(&contract_hash.to_string(), res);
                res
            }
        };

        match list.iter().find(|k| k == key) {
            Some(res) => {}
            None => {
                list.push(key);
                self.store.set(&contract_hash.to_string(), res);
            }
        }
    }

    pub fn remove_operator(&self, contract_hash: ContractHash, key: Key) {
        let mut list: Vec<Key> = match self.store.get(&contract_hash.to_string()) {
            Some(res) => res,
            None => {
                let res = Vec::<Key>::new();
                self.store.set(&contract_hash.to_string(), res);
                res
            }
        };

        match list.iter().position(|k| k == key) {
            Some(res) => {
                list.remove(res);
                self.store.set(&contract_hash.to_string(), res);
            }
            None => {}
        }
    }

    pub fn contract_has_operator(&self, contract_hash: ContractHash, key: Key) -> bool {
        let mut list: Vec<Key> = match self.store.get(&contract_hash.to_string()) {
            Some(res) => res,
            None => {
                let res = Vec::<Key>::new();
                self.store.set(&contract_hash.to_string(), res);
                res
            }
        };

        let pos_option = list.iter().position(|k| k == key);

        pos_option.is_some()
    }
}

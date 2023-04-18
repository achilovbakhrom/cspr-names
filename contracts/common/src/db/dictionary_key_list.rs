use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types::CLTyped;
use crate::db::dictionary::Dictionary;
use crate::db::traits::Storable;
use crate::errors::CommonError;
use crate::utils::response::response_error;

pub struct DictionaryKeyList {
    store: Dictionary
}

impl DictionaryKeyList {

    pub fn instance(key: &str) -> Self {
        Self {
            store: Dictionary::instance(key),
        }
    }

    pub fn initialize(key: &str) {
        Dictionary::init(key)
    }

    pub fn set<V: CLTyped + ToBytes>(&self, key: &str, value: Vec<V>) {
        self.store.set(key, value)
    }

    pub fn get<V: CLTyped + FromBytes>(&self, key: &str) -> Option<Vec<V>> {
        self.store.get(key)
    }

    pub fn append<V: CLTyped + ToBytes + FromBytes + PartialEq>(&self, key: &str, value: Vec<V>) {
        let mut current = self.store.get(key).unwrap_or(vec![]);

        current.extend(value);

        current.dedup_by(|item1, item2| item1 == item2);


        self.set(key, current);
    }

    pub fn remove<V: CLTyped + FromBytes + ToBytes + PartialEq>(&self, key: &str, value: V) {
        let mut current: Vec<V> = self.get(key).unwrap_or(vec![]);
        let position = match current.iter().position(|item| item == &value) {
            Some(res) => res,
            None => return response_error(CommonError::ItemNotFound)
        };
        if position >= 0 {
            current.remove(position);
        }
        self.set(key, current);
    }

}
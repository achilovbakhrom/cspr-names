use super::traits::Storable;
use crate::utils::storage::{get_stored_value_from_key, store_value_for_key};
use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types::CLTyped;

pub struct Store;

impl Storable for Store {
    fn get<T: CLTyped + FromBytes>(&self, key: &str) -> Option<T> {
        get_stored_value_from_key(key)
    }

    fn set<T: CLTyped + ToBytes>(&self, key: &str, value: T) {
        store_value_for_key(key, value)
    }

    fn remove<T: CLTyped + ToBytes>(&self, key: &str) {
        store_value_for_key(key, None::<T>)
    }
}

// Constructors
impl Store {
    pub fn instance() -> Self {
        Self {}
    }
}

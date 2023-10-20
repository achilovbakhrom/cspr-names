use crate::db::dictionary::Dictionary;
use crate::db::traits::Storable;
use crate::errors::CommonError;
use crate::utils::response::response_error;
use alloc::vec::Vec;
use casper_types::bytesrepr::{ FromBytes, ToBytes };
use casper_types::CLTyped;

pub struct DictionaryKeyList {
	store: Dictionary,
}

impl DictionaryKeyList {
	pub fn instance(key: &str) -> Self {
		Self {
			store: Dictionary::instance(key),
		}
	}

	pub fn initialize(key: &str) {
		Dictionary::init(key);
	}

	pub fn set<V: CLTyped + ToBytes>(&self, key: &str, value: Vec<V>) {
		self.store.set(key, value)
	}

	pub fn get<V: CLTyped + FromBytes>(&self, key: &str) -> Option<Vec<V>> {
		self.store.get(key)
	}

	pub fn append<V: CLTyped + ToBytes + FromBytes + PartialEq>(&self, key: &str, value: Vec<V>) {
		let mut current = self.store.get::<Vec<V>>(key).unwrap_or_default();

		current.extend(value);

		current.dedup_by(|item1, item2| item1 == item2);

		self.set(key, current);
	}

	pub fn remove<V: CLTyped + FromBytes + ToBytes + PartialEq>(&self, key: &str, value: V) {
		let mut current = self.get::<V>(key).unwrap_or_default();
		let position = match current.iter().position(|item| item == &value) {
			Some(res) => res,
			None => {
				return response_error(CommonError::ItemNotFound);
			}
		};
		current.remove(position);
		self.set(key, current);
	}
}

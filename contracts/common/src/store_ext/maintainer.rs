use alloc::{ string::ToString, vec::Vec, vec };
use casper_types::{ account::AccountHash, ContractHash, Key };

use crate::{
	db::{ store::Store, traits::Storable },
	constants::common_keys::CommonKeys,
};

impl Store {
	pub fn get_maintainer(&self) -> Option<AccountHash> {
		self.get(&CommonKeys::Maintainer.to_string())
	}
	pub fn get_administration_contract_hash(&self) -> Option<ContractHash> {
		self.get(&CommonKeys::AdministrationContract.to_string())
	}

	pub fn set_keys(&self, key: &str, values: Vec<Key>) {
		self.set(key, values)
	}

	pub fn add_key(&self, key: &str, value: Key) {
		let store = Store::instance();
		let mut authorities = store.get::<Vec<Key>>(key).unwrap_or(vec![]);
		authorities.push(value);
		store.set(&CommonKeys::Authorities.to_string(), authorities);
	}

	pub fn remove_key(&self, key: &str, value: Key) {
		let store = Store::instance();
		let mut authorities = store.get::<Vec<Key>>(key).unwrap_or(vec![]);
		let position = authorities.iter().position(|item| item == &value);
		if let Some(pos) = position {
			authorities.remove(pos);
		}
		store.set(&CommonKeys::Authorities.to_string(), authorities);
	}

	pub fn get_keys(&self, key: &str) -> Vec<Key> {
		self.get::<Vec<Key>>(key).unwrap_or(vec![])
	}

	pub fn set_authorities(&self, values: Vec<Key>) {
		self.set_keys(&CommonKeys::Authorities.to_string(), values);
	}

	pub fn add_authority(&self, value: Key) {
		self.add_key(&CommonKeys::Authorities.to_string(), value);
	}

	pub fn remove_authority(&self, value: Key) {
		self.remove_key(&CommonKeys::Authorities.to_string(), value)
	}

	pub fn get_authorities(&self) -> Vec<Key> {
		self.get_keys(&CommonKeys::Authorities.to_string())
	}
}

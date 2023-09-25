use alloc::{ string::ToString, vec::{ Vec } };
use casper_types::{ Key, KeyTag, Tagged };
use common_lib::{
	constants::KEY_REGISTRY_CONTRACT_OPERATORS,
	db::{ dictionary::Dictionary, traits::Storable },
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

	pub fn add_operator(&self, key: Key, value: Key) {
		let mut list: Vec<Key> = match self.store.get(&key.to_string()) {
			Some(res) => res,
			None => {
				let res = Vec::<Key>::new();
				self.store.set(&key.to_string(), res.clone());
				res
			}
		};

		match list.iter().find(|k| k == &&value) {
			Some(res) => {}
			None => {
				list.push(value);
				self.store.set(&key.to_string(), list);
			}
		}
	}

	pub fn remove_operator(&self, key: Key, value: Key) {
		let mut list: Vec<Key> = match self.store.get(&key.to_string()) {
			Some(res) => res,
			None => {
				let res = Vec::<Key>::new();
				self.store.set(&key.to_string(), res.clone());
				res
			}
		};

		match list.iter().position(|k| k == &value) {
			Some(res) => {
				list.remove(res);
				self.store.set(&key.to_string(), list);
			}
			None => {}
		}
	}

	pub fn contract_has_operator(&self, key: Key, value: Key) -> bool {
		let mut list: Vec<Key> = match self.store.get(&key.to_string()) {
			Some(res) => res,
			None => {
				let tempList = Vec::<Key>::new();
				self.store.set(&key.to_string(), tempList.clone());
				tempList
			}
		};

		let pos_option = list.iter().position(|k| k == &value);

		pos_option.is_some()
	}

	pub fn get_operators(&self, key: Key, tag: Option<KeyTag>) -> Vec<Key> {
		match self.store.get::<Vec<Key>>(&key.to_string()) {
			Some(res) => {
				if let Some(t) = tag {
					let mut result = Vec::<Key>::new();

					res.iter().for_each(|x| {
						let tempTag: KeyTag = (*x).tag();
						if tempTag == t {
							result.push(x.clone())
						}
					});
					result
				} else {
					res
				}
			}
			None => {
				let res = Vec::<Key>::new();
				self.store.set(&key.to_string(), res.clone());
				res
			}
		}
	}
}

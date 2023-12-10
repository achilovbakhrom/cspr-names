use alloc::{ vec::{ Vec, self }, string::ToString };
use casper_types::Key;
use common_lib::{
	db::{ store::Store, traits::Storable },
	constants::common_keys::AdministractionStoreKeys,
};

pub(crate) trait Authorities {
	fn add_authorities(&self, keys: Vec<Key>) -> ();
	fn remove_authorities(&self, keys: Vec<Key>) -> ();
	fn get_authorities(&self) -> Vec<Key>;
}

impl Authorities for Store {
	fn add_authorities(&self, keys: Vec<Key>) -> () {
		let store_key = &AdministractionStoreKeys::Authorities.to_string();
		let authorities = self.get::<Vec<Key>>(store_key).unwrap_or(vec![]);

		keys.iter().for_each(|key| {
			if !authorities.contains(key) {
				authorities.push(*key)
			}
		});
		self.set(store_key, authorities)
	}

	fn remove_authorities(&self, keys: Vec<Key>) -> () {
		let store_key = &AdministractionStoreKeys::Authorities.to_string();
		let authorities = self.get::<Vec<Key>>(store_key).unwrap_or(vec![]);
		keys.iter().for_each(|key| {
			let pos = authorities.iter().position(|k| k == key);
			if let position = pos {
				authorities.remove(position)
			}
		});

		self.set(store_key, authorities)
	}

	fn get_authorities(&self) -> Vec<Key> {
		self
			.get(&AdministractionStoreKeys::Authorities.to_string())
			.unwrap_or(vec![])
	}
}

use alloc::string::{ String, ToString };
use alloc::{ vec, vec::Vec };
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::Key;
use casper_types::account::AccountHash;
use common_lib::constants::common_keys::KEY_DATABASE_DICTIONARY_OWNER_DOMAIN_LIST;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use common_lib::errors::DatabaseErrors;

pub(crate) struct OwnerDomainList {
	store: Dictionary,
}

impl OwnerDomainList {
	pub fn instance() -> Self {
		Self {
			store: Dictionary::instance(KEY_DATABASE_DICTIONARY_OWNER_DOMAIN_LIST),
		}
	}

	pub fn initialize() {
		Dictionary::init(KEY_DATABASE_DICTIONARY_OWNER_DOMAIN_LIST)
	}

	pub fn add_domain_name(&mut self, owner: Key, domain_name: &str) {
		let key = &owner.to_string();
		let mut domains: Vec<String> = self.store.get(key).unwrap_or(vec![]);
		domains.push(domain_name.to_string());
		self.store.set(key, domains)
	}

	pub fn remove_domain_name(&mut self, owner: Key, domain_name: &str) {
		let key = &owner.to_string();
		let mut domains: Vec<String> = self.store.get(key).unwrap_or(vec![]);
		domains
			.iter()
			.position(|x| x == domain_name)
			.and_then(|pos| {
				domains.remove(pos);
				None
			})
			.unwrap_or_revert_with(DatabaseErrors::DatabaseDomainDoesntExist)
	}

	pub fn get_domain_list(&self, owner: AccountHash) -> Vec<String> {
		let key = &owner.to_string();
		self.store.get(key).unwrap_or(vec![])
	}
}

use alloc::string::{ String, ToString };
use alloc::vec;
use alloc::vec::Vec;
use common_lib::{
	db::{ dictionary::Dictionary, traits::Storable },
	constants::common_keys::{ KEY_DATABASE_DICTIONARY_SUBDOMAIN_LIST, MAX_PAGE_SIZE },
};
use common_lib::errors::DatabaseErrors;
use common_lib::models::SubdomainName;

pub(crate) struct SubdomainList {
	store: Dictionary,
}

impl SubdomainList {
	pub fn instance() -> Self {
		Self {
			store: Dictionary::instance(KEY_DATABASE_DICTIONARY_SUBDOMAIN_LIST),
		}
	}

	pub fn initialize() {
		Dictionary::init(KEY_DATABASE_DICTIONARY_SUBDOMAIN_LIST)
	}

	pub fn add(&self, name: &str, subdomain_name: &SubdomainName) -> Result<(), DatabaseErrors> {
		let mut subdomains = match self.store.get::<Vec<SubdomainName>>(name) {
			Some(res) => res,
			None => vec![],
		};

		if subdomains.len() > MAX_PAGE_SIZE.into() {
			return Err(DatabaseErrors::DatabaseSubdomainMaxCountExceeded);
		}

		subdomains.push(subdomain_name.clone());
		self.store.set(name, subdomains);

		Ok(())
	}

	pub fn remove(&self, name: &str, subdomain_name: &str) -> Result<(), DatabaseErrors> {
		let mut subdomains = match self.store.get::<Vec<SubdomainName>>(name) {
			Some(res) => res,
			None => vec![],
		};

		let position = match subdomains.iter().position(|x| x.name == subdomain_name) {
			Some(pos) => pos,
			None => {
				return Err(DatabaseErrors::DatabaseSubdomainDoesntExist);
			}
		};
		subdomains.remove(position);
		self.store.set(name, subdomains);

		Ok(())
	}

	pub fn get_subdomains(&self, name: &str) -> Vec<String> {
		let subdomains = match self.store.get::<Vec<SubdomainName>>(name) {
			Some(res) => res,
			None => vec![],
		};
		subdomains
			.iter()
			.map(|x| x.name.to_string())
			.collect()
	}
}

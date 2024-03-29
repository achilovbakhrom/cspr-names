use casper_types::account::AccountHash;
use common_lib::constants::common_keys::KEY_DATABASE_DICTIONARY_SUBDOMAIN;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use common_lib::errors::DatabaseErrors;
use common_lib::models::SubdomainName;

pub(crate) struct SubdomainEntityStore {
	store: Dictionary,
}

impl SubdomainEntityStore {
	pub fn instance() -> Self {
		Self {
			store: Dictionary::instance(KEY_DATABASE_DICTIONARY_SUBDOMAIN),
		}
	}

	pub fn initialize() {
		Dictionary::init(KEY_DATABASE_DICTIONARY_SUBDOMAIN)
	}

	pub fn save(&self, sub_domain: SubdomainName) {
		self.store.set(&sub_domain.name, sub_domain.clone());
	}

	pub fn remove(&self, name: &str) {
		self.store.remove::<SubdomainName>(name);
	}

	pub fn update_resolver(&self, name: &str, resolver: AccountHash) -> Result<(), DatabaseErrors> {
		self.store
			.get::<SubdomainName>(name)
			.ok_or(DatabaseErrors::DatabaseSubdomainDoesntExist)
			.and_then(|mut subdomain| {
				subdomain.resolver = resolver;
				self.store.set(name, subdomain);
				Ok(())
			})
	}

	pub fn get(&self, name: &str) -> Option<SubdomainName> {
		self.store.get(name)
	}
}

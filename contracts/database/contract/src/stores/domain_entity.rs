use casper_types::account::AccountHash;
use common_lib::{
	constants::common_keys::KEY_DATABASE_DICTIONARY_DOMAIN,
	db::{ dictionary::Dictionary, traits::Storable },
	errors::DatabaseErrors,
	models::DomainName,
};

pub(crate) struct DomainEntityStore {
	store: Dictionary,
}

impl DomainEntityStore {
	pub fn instance() -> Self {
		Self {
			store: Dictionary::instance(KEY_DATABASE_DICTIONARY_DOMAIN),
		}
	}

	pub fn initialize() {
		Dictionary::init(KEY_DATABASE_DICTIONARY_DOMAIN);
	}

	fn update_domain_name<F: FnOnce(DomainName) -> DomainName>(
		&self,
		name: &str,
		err: DatabaseErrors,
		mapper: F
	) -> Result<(), DatabaseErrors> {
		self.store
			.get::<DomainName>(name)
			.ok_or(err)
			.map(mapper)
			.and_then(|res| {
				self.store.set(name, res);
				Ok(())
			})
	}

	pub fn save(&self, domain: DomainName) {
		self.store.set(domain.name.as_str(), domain.clone())
	}

	pub fn remove(&self, name: &str) {
		self.store.remove::<DomainName>(name);
	}

	pub fn get(&self, name: &str) -> Option<DomainName> {
		self.store.get(name)
	}

	pub fn update_owner(&self, name: &str, new_owner: AccountHash) -> Result<(), DatabaseErrors> {
		self.update_domain_name(name, DatabaseErrors::DatabaseDomainDoesntExist, |mut arg| {
			arg.owner = new_owner;
			arg
		})
	}

	pub fn update_expiration_date(&self, name: &str, end_time: u64) -> Result<(), DatabaseErrors> {
		self.update_domain_name(name, DatabaseErrors::DatabaseDomainDoesntExist, |mut arg| {
			arg.end_time = end_time;
			arg
		})
	}

	pub fn update_resolver_address(
		&self,
		name: &str,
		resolver: AccountHash
	) -> Result<(), DatabaseErrors> {
		self.update_domain_name(name, DatabaseErrors::DatabaseDomainDoesntExist, |mut arg| {
			arg.resolver = resolver;
			arg
		})
	}
}

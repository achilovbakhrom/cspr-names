use common_lib::{
	db::{ dictionary::Dictionary, traits::Storable },
	constants::common_keys::KEY_DATABASE_DICTIONARY_DOMAIN_MAP,
	errors::DatabaseErrors,
};

pub(crate) struct DomainPaginationMap {
	store: Dictionary,
}

impl DomainPaginationMap {
	pub fn instance() -> Self {
		Self {
			store: Dictionary::instance(KEY_DATABASE_DICTIONARY_DOMAIN_MAP),
		}
	}

	pub fn initialize() {
		Dictionary::init(KEY_DATABASE_DICTIONARY_DOMAIN_MAP)
	}

	pub fn map(&self, name: &str, page: u64) {
		self.store.set(name, page)
	}

	pub fn get_page(&self, name: &str) -> Result<u64, DatabaseErrors> {
		let page = match self.store.get::<u64>(name) {
			None => {
				return Err(DatabaseErrors::DatabaseDomainDoesntExist);
			}
			Some(res) => res,
		};
		Ok(page)
	}

	pub fn remove(&self, name: &str) {
		self.store.remove::<u64>(name);
	}
}

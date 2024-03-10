use alloc::string::{ String, ToString };
use alloc::vec;
use alloc::vec::Vec;
use common_lib::{
	db::{ dictionary::Dictionary, store::Store, traits::Storable },
	errors::DatabaseErrors,
	constants::common_keys::{
		KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
		KEY_DATABASE_DOMAIN_LIST_PAGINATION,
		MAX_PAGE_SIZE,
	},
};

pub(crate) struct DomainListStore {
	store: Dictionary,
	state: Store,
}

impl DomainListStore {
	pub fn instance() -> Self {
		Self {
			store: Dictionary::instance(KEY_DATABASE_DICTIONARY_DOMAIN_LIST),
			state: Store::instance(),
		}
	}

	pub fn initialize() {
		Dictionary::init(KEY_DATABASE_DICTIONARY_DOMAIN_LIST);
	}

	pub fn add(&self, name: &str) -> Result<u64, DatabaseErrors> {
		let pagination = self.get_pagination();
		let domain_list = self.get_domain_list(pagination[0].to_string().as_ref());

		Ok((domain_list.clone(), pagination.clone())).and_then(
			|(mut list, mut pagination)| {
				if list.len() >= MAX_PAGE_SIZE {
					if pagination.len() == 1 {
						pagination[0] += 1;
					} else {
						pagination.remove(0);
					}
					self.state.set(
						KEY_DATABASE_DOMAIN_LIST_PAGINATION,
						pagination.clone()
					);
				}
				list.push(name.to_string());
				self.store.set(pagination[0].to_string().as_ref(), list);

				Ok(pagination[0])
			}
		)
	}

	pub fn remove(&self, page: u64, name: &str) -> Result<(), DatabaseErrors> {
		let mut domain_list = self.get_domain_list(page.to_string().as_ref());

		domain_list
			.iter()
			.position(|x| x == name)
			.ok_or(DatabaseErrors::DatabaseDomainDoesntExist)
			.and_then(|pos| {
				domain_list.remove(pos);
				Ok(domain_list)
			})
			.and_then(|list| {
				self.store.set(page.to_string().as_ref(), list);
				Ok(())
			})
			.and_then(|_| {
				let mut pagination = self.get_pagination();
				if pagination.len() == 1 && pagination[0] == page {
					pagination[0] -= 1;
				} else {
					pagination.push(page);
					pagination.sort();
				}
				self.state.set(KEY_DATABASE_DOMAIN_LIST_PAGINATION, pagination);
				Ok(())
			})
	}

	pub fn get_domain_list(&self, page: &str) -> Vec<String> {
		self.store.get::<Vec<String>>(page).unwrap_or(Vec::<String>::new())
	}

	fn get_pagination(&self) -> Vec<u64> {
		self.state
			.get::<Vec<u64>>(KEY_DATABASE_DOMAIN_LIST_PAGINATION)
			.unwrap_or(vec![0u64])
	}
}

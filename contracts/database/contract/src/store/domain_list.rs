use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use common_lib::{
    db::{
        dictionary::Dictionary,
        store::Store,
        traits::Storable,
    },
    errors::DatabaseErrors,
    constants::{
        KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
        KEY_DATABASE_DOMAIN_LIST_PAGINATION,
        MAX_PAGE_SIZE,
    },
};

pub(crate) struct DomainList {
    store: Dictionary,
    state: Store,
}

impl DomainList {

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
        let mut pagination = self.get_pagination();
        let mut domain_list = self.get_domain_list(pagination[0].to_string().as_ref());

        if (domain_list.len() as u8) >= MAX_PAGE_SIZE {
            if pagination.len() == 1 {
                pagination[0] += 1;
            } else {
                pagination.remove(0);
            }
            self.state.set(KEY_DATABASE_DOMAIN_LIST_PAGINATION, pagination.clone());
            let mut domain_list = self.get_domain_list(pagination[0].to_string().as_ref());
            domain_list.push(name.to_string());
            self.store.set::<Vec<String>>(pagination[0].to_string().as_ref(), domain_list);
        } else {
            domain_list.push(name.to_string());
            self.store.set(pagination[0].to_string().as_ref(), domain_list);
        }

        Ok(pagination[0])
    }

    pub fn remove(&self, name: &str, page: u64) -> Result<(), DatabaseErrors> {
        let mut pagination = self.get_pagination();
        let mut domain_list = self.get_domain_list(page.to_string().as_ref());
        if !domain_list.contains(&name.to_string()) {
            return Err(DatabaseErrors::DatabaseDomainDoesntExist)
        }

        let position = domain_list.iter().position(|x| x == name).unwrap_or(0);
        domain_list.remove(position);
        self.store.set(page.to_string().as_ref(), domain_list);

        if !pagination.contains(&page) {
            pagination.push(page);
            pagination.sort();
            self.state.set(KEY_DATABASE_DOMAIN_LIST_PAGINATION, pagination);
        }

        Ok(())
    }

    pub fn get_domain_list(&self, page: &str) -> Vec<String> {
        match self.store.get::<Vec<String>>(page) {
            None => Vec::<String>::new(),
            Some(res) => res
        }
    }

    fn get_pagination(&self) -> Vec<u64> {
        match self.state.get::<Vec<u64>>(KEY_DATABASE_DOMAIN_LIST_PAGINATION) {
            None => vec![0u64],
            Some(res) => res
        }
    }

}
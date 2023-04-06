use alloc::string::ToString;
use casper_types::account::AccountHash;
use common_lib::{
    db::{
        dictionary::Dictionary,
        traits::Storable,
    },
    constants::KEY_DATABASE_DICTIONARY_DOMAIN,
    models::DomainName,
    errors::DatabaseErrors,
};

pub(crate) struct DomainMap {
    store: Dictionary
}

impl DomainMap {

    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_DATABASE_DICTIONARY_DOMAIN)
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_DATABASE_DICTIONARY_DOMAIN);
    }

    pub fn save(&self, domain: DomainName) {
        self.store.set(domain.name.as_str(), domain.clone())
    }

    pub fn remove(&self, name: &str) {
        self.store.remove::<DomainName>(name);
    }

    pub fn update_owner(&self, name: &str, new_owner: AccountHash) -> Result<(), DatabaseErrors> {
        let mut domain_name = match self.store.get::<DomainName>(name) {
            Some(res) => res,
            None => return Err(DatabaseErrors::DatabaseDomainDoesntExist)
        };
        domain_name.owner = new_owner;
        self.store.set(name, domain_name);
        Ok(())
    }

    pub fn update_expiration_date(&self, name: &str, end_time: u64) -> Result<(), DatabaseErrors> {
        let mut domain_name = match self.store.get::<DomainName>(name) {
            Some(res) => res,
            None => return Err(DatabaseErrors::DatabaseDomainDoesntExist)
        };
        domain_name.end_time = end_time;
        self.store.set(name, domain_name);
        Ok(())
    }

    pub fn update_resolver_address(&self, name: &str, resolver: AccountHash) -> Result<(), DatabaseErrors> {
        let mut domain_name = match self.store.get::<DomainName>(name) {
            Some(res) => res,
            None => return Err(DatabaseErrors::DatabaseDomainDoesntExist)
        };
        domain_name.resolver = resolver;
        self.store.set(name, domain_name);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<DomainName> {
        self.store.get(name)
    }

}
use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use casper_types::account::AccountHash;
use common_lib::constants::KEY_DATABASE_DICTIONARY_OWNER_DOMAIN_LIST;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use casper_contract::{ contract_api::runtime };

pub(crate) struct OwnerDomainList {
    store: Dictionary
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

    pub fn add_domain_name(&mut self, owner: AccountHash, domain_name: &str) {
        let key = &owner.to_string();
        let mut domains: Vec<String> = match self.store.get(key) {
            Some(res) => res,
            None => vec![],
        };
        domains.push(domain_name.to_string());
        self.store.set(key, domains)
    }

    pub fn remove_domain_name(&mut self, owner: AccountHash, domain_name: &str) {
        let key = &owner.to_string();
        let mut domains: Vec<String> = match self.store.get(key) {
            Some(res) => res,
            None => vec![],
        };
        let position = domains
            .iter()
            .position(|x| x == domain_name);
        if let Some(pos) = position {
            if pos >= 0 {
                domains.remove(pos);
            }
        }
    }

    pub fn get_domain_list(&self, owner: AccountHash) -> Vec<String> {
        let key = &owner.to_string();
        return match self.store.get(key) {
            Some(res) => res,
            None => vec![],
        }
    }
}

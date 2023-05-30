use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::ContractHash;
use common_lib::constants::{KEY_AUTHORITY_CONTRACT_DB, KEY_MAIN_DATABASE_CONTRACT_HASH_MAP, MAX_DOMAIN_NAME_COUNT_PER_DATABASE};
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;
use common_lib::enums::contracts_enum::ContractKind;
use common_lib::models::contract_hash_database_map::ContractHashDatabaseMap;

pub(crate) struct ContractHashDb {
    store: Dictionary
}

impl ContractHashDb {

    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_AUTHORITY_CONTRACT_DB)
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_AUTHORITY_CONTRACT_DB)
    }

    pub fn set_contract_hash(&self, contract_type: ContractKind, contract_hash: ContractHash, extension: Option<String>) {
        match contract_type {
            ContractKind::Database => {
                let key = &format!("{}_{}",  (contract_type as u8).to_string(), extension.unwrap());
                self.attach_contract_hash_to_extension(
                    key,
                    contract_hash
                )
            }
            _ => {

                self.store.set(&(contract_type as u8).to_string(), vec![contract_hash])
            }
        }
    }

    pub fn remove_contract_hash(&self, contract_type: ContractKind, contract_hash: ContractHash, extension: Option<String>) {
        match contract_type {
            ContractKind::Database => {
                let key = &format!("{}_{}",  (contract_type as u8).to_string(), extension.unwrap());
                self.detach_contract_hash_from_extension(
                    key,
                    contract_hash
                )
            }
            _ => {
                self.store.remove(&(contract_type as u8).to_string())
            }
        }
    }

    pub fn get_contract_hash(&self, contract_type: ContractKind, extension: Option<String>) -> Option<ContractHash> {
        match contract_type {
            ContractKind::Database => {
                let key = &format!("{}_{}",  (contract_type as u8).to_string(), extension.unwrap());
                self.get_contract_hash_for_key(key)
            }
            _ => {
                let list: Vec<ContractHash> = self.store.get(&(contract_type as u8).to_string()).unwrap_or(vec![]);
                if list.len() > 0 {
                    return Some(list[0])
                }
                None
            }
        }
    }

    pub fn attach_contract_hash_to_extension(&self, db_key: &str, contract_hash: ContractHash) {
        let mut list = self.get_contract_hash_list_for_extension(db_key);
        match list.iter().find(|item| item.contract_hash == &contract_hash) {
            Some(_) => {},
            None => {
                list.push(ContractHashDatabaseMap{ contract_hash, count: 0 });
                self.store.set(db_key, list);
            }
        };
    }

    pub fn detach_contract_hash_from_extension(&self, db_key: &str, contract_hash: ContractHash) {
        let mut list = self.get_contract_hash_list_for_extension(db_key);
        match list.iter().find(|item| item.contract_hash == &contract_hash) {
            Some(_) => {
                let pos = list.iter().position(|item| item == &contract_hash);
                if let Some(position) = pos {
                    list.remove(position);
                    self.store.set(db_key, list);
                }
            },
            None => {}
        };
    }

    pub fn get_contract_hash_list_for_extension(&self, db_key: &str) -> Vec<ContractHashDatabaseMap> {
        match self.store.get(db_key) {
            Some(res) => res,
            None => vec![]
        }
    }

    pub fn get_contract_hash_for_key(&self, db_key: &str) -> Option<ContractHash> {
        let mut list = self.get_contract_hash_list_for_extension(db_key);
        if !list.is_empty() {
            list.sort_by(|a, b| b.count.cmp(*a.count));
            let found = list.iter().find(|item| item.count < MAX_DOMAIN_NAME_COUNT_PER_DATABASE);
            if let Some (f) = found {
                Some(f.contract_hash)
            }
        }
        None
    }

}
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::ContractHash;
use common_lib::constants::{KEY_MAIN_ALLOWED_EXTENSIONS, KEY_MAIN_REGISTRY_CONTRACT_HASH};
use common_lib::db::store::Store;
use common_lib::db::traits::Storable;
use common_lib::errors::MainContractErrors;

pub struct ConfigDb {
    store: Store,
}

impl ConfigDb {
    pub fn instance() -> Self {
        Self {
            store: Store::instance(),
        }
    }

    pub fn set_allowed_extensions(&self, extensions: Vec<String>) {
        self.store.set(KEY_MAIN_ALLOWED_EXTENSIONS, extensions)
    }

    pub fn get_allowed_extensions(&self) -> Option<Vec<String>> {
        self.store.get(KEY_MAIN_ALLOWED_EXTENSIONS)
    }

    pub fn add_extension(&self, extension: String) {
        let mut extensions = self.get_allowed_extensions().unwrap_or_default();
        extensions.push(extension);
        self.set_allowed_extensions(extensions)
    }

    pub fn remove_extension(&self, extension: String) {
        let mut extensions = self.get_allowed_extensions().unwrap_or_default();
        let pos = extensions
            .iter()
            .position(|item| item == &extension)
            .unwrap_or_revert_with(MainContractErrors::InvalidExtension);
        extensions.remove(pos);
        self.set_allowed_extensions(extensions)
    }

    pub fn set_registry_contract_hash(&self, contract_hash: ContractHash) {
        self.store
            .set(KEY_MAIN_REGISTRY_CONTRACT_HASH, contract_hash);
    }

    pub fn get_registry_contract_hash(&self) -> Option<ContractHash> {
        self.store.get(KEY_MAIN_REGISTRY_CONTRACT_HASH)
    }
}

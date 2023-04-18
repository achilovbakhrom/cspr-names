use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use common_lib::constants::KEY_MAIN_ALLOWED_EXTENSIONS;
use common_lib::db::store::Store;
use common_lib::db::traits::Storable;
use common_lib::errors::{CommonError, MainContractErrors};

pub struct LocalDb {
    store: Store
}

impl LocalDb {

    pub fn instance() -> Self {
        Self {
            store: Store::instance()
        }
    }

    pub fn set_allowed_extensions(&self, extensions: Vec<String>) {
        self.store.set(KEY_MAIN_ALLOWED_EXTENSIONS, extensions)
    }

    pub fn get_allowed_extensions(&self) -> Option<Vec<String>> {
        self.store.get(KEY_MAIN_ALLOWED_EXTENSIONS)
    }

    pub fn add_extension(&self, extension: String) {
        let mut extensions = self.get_allowed_extensions().unwrap_or(vec![]);
        extensions.push(extension);
        self.set_allowed_extensions(extensions)
    }

    pub fn remove_extension(&self, extension: String) {
        let mut extensions = self.get_allowed_extensions().unwrap_or(vec![]);
        let pos = extensions.iter().position(|item| item == &extension).unwrap_or_revert_with(
            MainContractErrors::InvalidExtension
        );
    }

}
use alloc::{ vec::Vec, string::String };
use casper_contract::{
	unwrap_or_revert::UnwrapOrRevert,
	contract_api::runtime::revert,
};
use common_lib::{
	db::{ store::Store, traits::Storable },
	constants::common_keys::AdministractionStoreKeys,
	errors::AdministrationErrors,
};

pub(crate) trait AllowedExtensions {
	fn get_allowed_extensions(&self) -> Vec<String>;
	fn set_allowed_extensions(&self, vec: Vec<String>) -> ();
	fn add_extension(&self, ext: String) -> ();
	fn remove_extension(&self, ext: String) -> ();
}

impl AllowedExtensions for Store {
	fn get_allowed_extensions(&self) -> Vec<String> {
		self
			.get(AdministractionStoreKeys::AllowedExtensions)
			.unwrap_or(Vec::<String>::new())
	}

	fn set_allowed_extensions(&self, vec: Vec<String>) -> () {
		self.set(AdministractionStoreKeys::AllowedExtensions, vec)
	}

	fn add_extension(&self, ext: String) -> () {
		let mut extensions = self.get_allowed_extensions();
		if !extensions.contains(&ext) {
			extensions.push(ext);
			self.set(AdministractionStoreKeys::AllowedExtensions, extensions);
		}
	}

	fn remove_extension(&self, ext: String) -> () {
		let mut extensions = self.get_allowed_extensions();
		if extensions.contains(&ext) {
			let position = extensions
				.iter()
				.position(|item| item == &ext)
				.unwrap_or_revert_with(error);
			extensions.remove(ext);
			self.set(AdministractionStoreKeys::AllowedExtensions, extensions);
		}
	}
}

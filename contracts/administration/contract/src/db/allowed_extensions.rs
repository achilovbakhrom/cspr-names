use alloc::{ vec::Vec, string::{ String, ToString } };
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use common_lib::{
	db::{ store::Store, traits::Storable },
	constants::common_keys::AdministractionStoreKeys,
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
			.get(&AdministractionStoreKeys::AllowedExtensions.to_string())
			.unwrap_or(Vec::<String>::new())
	}

	fn set_allowed_extensions(&self, vec: Vec<String>) -> () {
		self.set(&AdministractionStoreKeys::AllowedExtensions.to_string(), vec)
	}

	fn add_extension(&self, ext: String) -> () {
		let mut extensions = self.get_allowed_extensions();
		if !extensions.contains(&ext) {
			extensions.push(ext);
			self.set(
				&AdministractionStoreKeys::AllowedExtensions.to_string(),
				extensions
			);
		}
	}

	fn remove_extension(&self, ext: String) -> () {
		let mut extensions = self.get_allowed_extensions();
		if extensions.contains(&ext) {
			let position = extensions
				.iter()
				.position(|item| item == &ext)
				.unwrap_or_revert();
			extensions.remove(position);
			self.set(
				&AdministractionStoreKeys::AllowedExtensions.to_string(),
				extensions
			);
		}
	}
}

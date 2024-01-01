use super::traits::Storable;
use casper_contract::{ contract_api::{ runtime, storage }, unwrap_or_revert::UnwrapOrRevert };
use casper_types::{ bytesrepr::{ FromBytes, ToBytes }, CLTyped, URef };

pub struct Dictionary {
	u_ref: URef,
}

impl Storable for Dictionary {
	fn get<T: CLTyped + FromBytes>(&self, key: &str) -> Option<T> {
		storage::dictionary_get(self.u_ref, key).unwrap_or_revert().unwrap_or_default()
	}

	fn set<T: CLTyped + ToBytes>(&self, key: &str, value: T) {
		storage::dictionary_put(self.u_ref, key, Some(value));
	}

	fn remove<T: CLTyped + ToBytes>(&self, key: &str) {
		storage::dictionary_put(self.u_ref, key, None::<T>);
	}
}

// Initialization
impl Dictionary {
	pub fn init(name: &str) {
		storage::new_dictionary(name).unwrap_or_revert();
	}
}

// Constructors
impl Dictionary {
	pub fn instance(name: &str) -> Self {
		let key = runtime::get_key(name).unwrap_or_revert();
		let u_ref = *key.as_uref().unwrap_or_revert();

		Dictionary { u_ref }
	}
}

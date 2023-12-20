use alloc::string::ToString;
use casper_types::{ account::AccountHash, ContractHash };

use crate::{
	db::{ store::Store, traits::Storable },
	constants::common_keys::CommonKeys,
};

impl Store {
	pub fn get_maintainer(&self) -> Option<AccountHash> {
		self.get(&CommonKeys::Maintainer.to_string())
	}
	pub fn get_administration_contract_hash(&self) -> Option<ContractHash> {
		self.get(&CommonKeys::AdministrationContract.to_string())
	}
}

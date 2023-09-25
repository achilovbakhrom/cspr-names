use alloc::vec::Vec;
use casper_types::ContractHash;
use common_lib::{
	constants::KEY_REGISTRY_WHITELIST_CONTRACT_HASH,
	db::{ store::Store, traits::Storable },
};

pub(crate) struct RegistryWhitelistStore {
	state: Store,
}

impl RegistryWhitelistStore {
	pub fn instance() -> Self {
		Self {
			state: Store::instance(),
		}
	}

	pub fn add_contract_hash(&self, contract_hash: ContractHash) {
		let mut list = self.get_contract_hash_list();
		match list.iter().position(|x| x == &contract_hash) {
			Some(pos) => {}
			None => {
				list.push(contract_hash);
				self.state.set(KEY_REGISTRY_WHITELIST_CONTRACT_HASH, list);
			}
		}
	}

	pub fn remove_contract_hash(&self, contract_hash: ContractHash) {
		let mut list = self.get_contract_hash_list();
		match list.iter().position(|x| x == &contract_hash) {
			Some(pos) => {
				list.remove(pos);
				self.state.set(KEY_REGISTRY_WHITELIST_CONTRACT_HASH, list);
			}
			None => {}
		}
	}

	pub fn get_contract_hash_list(&self) -> Vec<ContractHash> {
		match self.state.get(KEY_REGISTRY_WHITELIST_CONTRACT_HASH) {
			Some(res) => res,
			None => {
				let list = Vec::<ContractHash>::new();
				self.state.set(KEY_REGISTRY_WHITELIST_CONTRACT_HASH, list.clone());
				list
			}
		}
	}
}

use alloc::{ vec::Vec, vec, string::ToString };
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ ContractHash, Key };
use common_lib::{
	constants::common_keys::AdministractionStoreKeys,
	db::{ dictionary::Dictionary, traits::Storable },
	errors::CommonError,
};

pub struct ContractAuthoritiesStore {
	pub dictionary: Dictionary,
}

impl ContractAuthoritiesStore {
	pub fn instance() -> Self {
		Self {
			dictionary: Dictionary::instance(
				&AdministractionStoreKeys::ContractAuthority.to_string()
			),
		}
	}

	pub fn initialize() -> () {
		Dictionary::init(&AdministractionStoreKeys::ContractAuthority.to_string())
	}
}

pub trait ContractAuthorities {
	fn set_contract_authority_list(
		&self,
		contract_hash: ContractHash,
		keys: Vec<Key>
	) -> ();

	fn add_contract_authority(&self, contract_hash: ContractHash, key: Key) -> ();

	fn get_contract_authority(&self, contract_hash: ContractHash) -> Vec<Key>;

	fn remove_contract_authority(
		&self,
		contract_hash: ContractHash,
		key: Key
	) -> ();
}

impl ContractAuthorities for ContractAuthoritiesStore {
	fn set_contract_authority_list(
		&self,
		contract_hash: ContractHash,
		keys: Vec<Key>
	) -> () {
		self.dictionary.set(&contract_hash.to_string(), keys)
	}

	fn get_contract_authority(&self, contract_hash: ContractHash) -> Vec<Key> {
		self.dictionary.get(&contract_hash.to_string()).unwrap_or(vec![])
	}

	fn add_contract_authority(
		&self,
		contract_hash: ContractHash,
		key: Key
	) -> () {
		let mut authorities = self.get_contract_authority(contract_hash.clone());
		authorities.push(key);
		self.set_contract_authority_list(contract_hash, authorities)
	}

	fn remove_contract_authority(
		&self,
		contract_hash: ContractHash,
		key: Key
	) -> () {
		let mut authorities = self.get_contract_authority(contract_hash.clone());
		let position = authorities
			.iter()
			.position(|item| item == key)
			.unwrap_or_revert_with(CommonError::ItemNotFound);
		authorities.remove(position);
		self.set_contract_authority_list(contract_hash, authorities)
	}
}

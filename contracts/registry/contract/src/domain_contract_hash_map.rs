use alloc::string::String;
use common_lib::{
	constants::KEY_REGISTRY_DOMAIN_CONTRACT_HASH,
	db::{ dictionary::Dictionary, traits::Storable },
	models::registry_contract_hash_pair::RegistryContractHashPair,
};

pub(crate) struct DomainContractHashMap {
	dictionary: Dictionary,
}

impl DomainContractHashMap {
	pub fn instance() -> Self {
		Self {
			dictionary: Dictionary::instance(KEY_REGISTRY_DOMAIN_CONTRACT_HASH),
		}
	}
	pub fn initialize() {
		Dictionary::init(KEY_REGISTRY_DOMAIN_CONTRACT_HASH)
	}

	pub fn map_domain_name_to_contract_hash(
		&self,
		domain_name: String,
		contract_hash_obj: RegistryContractHashPair
	) {
		self.dictionary.set(&domain_name, contract_hash_obj)
	}

	pub fn get_contract_hash_for_domain_name(
		&self,
		domain_name: String
	) -> Option<RegistryContractHashPair> {
		self.dictionary.get(&domain_name)
	}

	pub fn remove_domain_name_map(&self, domain_name: String) {
		self.dictionary.remove::<RegistryContractHashPair>(&domain_name)
	}
}

use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use casper_types::ContractHash;
use common_lib::{
	constants::common_keys::RegistryArgs,
	models::registry_contract_hash_pair::RegistryContractHashPair,
};

use crate::{ domain_contract_hash_map::DomainContractHashMap, types::TResult };

pub fn map_domain_name_to_contract_hash() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&RegistryArgs::DomainName.to_string()
	);
	let database_contract_hash: ContractHash = runtime::get_named_arg(
		&RegistryArgs::DatabaseContractHash.to_string()
	);
	let nft_contract_hash: ContractHash = runtime::get_named_arg(
		&RegistryArgs::NftContractHash.to_string()
	);

	let registry_object = RegistryContractHashPair {
		db_contract_hash: database_contract_hash,
		nft_contract_hash,
	};

	DomainContractHashMap::instance().map_domain_name_to_contract_hash(
		domain_name.clone(),
		registry_object
	);
	Ok(())
}

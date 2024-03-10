use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use common_lib::{
	constants::common_keys::RegistryArgs,
	errors::RegistryErrors,
	models::registry_contract_hash_pair::RegistryContractHashPair,
};

use crate::{ domain_contract_hash_map::DomainContractHashMap, types::TResult };

pub fn get_contract_hash_for_domain_name() -> TResult<Option<RegistryContractHashPair>> {
	let domain_name: String = runtime::get_named_arg(
		&RegistryArgs::DomainName.to_string()
	);

	return match
		DomainContractHashMap::instance().get_contract_hash_for_domain_name(
			domain_name
		)
	{
		Some(res) => Ok(Some(res)),
		None => Ok(None),
	};
}

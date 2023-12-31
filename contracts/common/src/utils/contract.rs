use alloc::string::ToString;
use casper_contract::contract_api::{ storage, runtime };
use casper_types::{
	EntryPoint,
	contracts::{ Parameters, NamedKeys },
	CLType,
	EntryPointAccess,
	EntryPointType,
	EntryPoints,
	ContractHash,
};

use crate::constants::common_keys::{
	KEY_CONTRACT_PACKAGE_NAME,
	KEY_CONTRACT_ACCESS,
	CommonKeys,
};

pub fn create_entrypoint(
	key: &str,
	params: Parameters,
	ret: CLType,
	access: EntryPointAccess,
	entry_point_type: EntryPointType
) -> EntryPoint {
	EntryPoint::new(key, params, ret, access, entry_point_type)
}

// pub fn create_named_keys(values: Vec<(String, URef)>) -> NamedKeys {
// 	let mut keys = NamedKeys::new();

// 	values.iter().for_each(|(key, value)| {
// 		keys.insert(key, value);
// 	});

// 	keys
// }

pub fn create_contract(
	entrypoints: EntryPoints,
	named_keys: Option<NamedKeys>
) -> (ContractHash, u32) {
	storage::new_contract(
		entrypoints,
		named_keys,
		Some(KEY_CONTRACT_PACKAGE_NAME.to_string()),
		Some(KEY_CONTRACT_ACCESS.to_string())
	)
}

pub fn setup_contract_info(
	entrypoints: EntryPoints,
	mut named_keys: NamedKeys
) {
	let maintainer_uref = storage::new_uref(runtime::get_caller());
	named_keys.insert(CommonKeys::Maintainer.to_string(), maintainer_uref.into());

	let (contract_hash, contract_version) = create_contract(
		entrypoints,
		Some(named_keys)
	);
	let contract_hash_uref = storage::new_uref(contract_hash);
	runtime::put_key(
		&CommonKeys::ContractHash.to_string(),
		contract_hash_uref.into()
	);

	let contract_version_uref = storage::new_uref(contract_version);
	runtime::put_key(
		&CommonKeys::ContractVersion.to_string(),
		contract_version_uref.into()
	);
}

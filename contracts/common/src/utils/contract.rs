use alloc::{ string::ToString, vec::Vec };
use casper_contract::{
	contract_api::{ runtime, storage },
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
	contracts::{ NamedKeys, Parameters },
	runtime_args,
	CLType,
	ContractHash,
	EntryPoint,
	EntryPointAccess,
	EntryPointType,
	EntryPoints,
	Key,
};

use crate::{
	constants::common_keys::{
		AdministrationArgs,
		AdministrationEndpoints,
		CommonKeys,
		KEY_CONTRACT_ACCESS,
		KEY_CONTRACT_PACKAGE_NAME,
	},
	enums::caller_verification_type::CallerVerificationType,
	errors::CommonError,
};

use super::{ maintainer::is_caller_maintainer, registry::get_verified_caller };

pub fn create_entrypoint(
	key: &str,
	params: Parameters,
	ret: CLType,
	access: EntryPointAccess,
	entry_point_type: EntryPointType
) -> EntryPoint {
	EntryPoint::new(key, params, ret, access, entry_point_type)
}

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

pub fn get_current_contract_hash() -> ContractHash {
	runtime
		::get_key(&CommonKeys::ContractHash.to_string())
		.unwrap_or_revert_with(CommonError::MissingContractHash)
		.into()
}

pub fn get_administration_contract_hash() -> ContractHash {
	runtime
		::get_key(&CommonKeys::AdministrationContract.to_string())
		.unwrap_or_revert_with(CommonError::MissingAdministrationContractHash)
		.into()
}

pub fn ensure_caller_has_permission_external(is_contract: Option<bool>) {
	if !is_caller_maintainer() {
		let administration_contract_hash: ContractHash = runtime
			::get_key(&CommonKeys::AdministrationContract.to_string())
			.unwrap_or_revert_with(CommonError::MissingAdministrationContractHash)
			.into();

		let current_hash = get_current_contract_hash();

		let authorities: Vec<Key> = runtime::call_contract(
			administration_contract_hash,
			&AdministrationEndpoints::GetContractAuthorityList.to_string(),
			runtime_args! {
			AdministrationArgs::ContractHash => current_hash
		}
		);

		let mut ver_type = CallerVerificationType::All;

		if let Some(v) = is_contract {
			ver_type = if v {
				CallerVerificationType::OnlyContractHash
			} else {
				CallerVerificationType::OnlyAccountHash
			};
		}

		let caller = get_verified_caller(ver_type).unwrap();

		authorities
			.iter()
			.find(|key| *key == &caller)
			.unwrap_or_revert_with(CommonError::InvalidCaller);
	}
}

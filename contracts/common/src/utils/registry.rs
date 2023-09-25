use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use casper_contract::contract_api::runtime;
use casper_types::system::CallStackElement;

use crate::constants::{
	ARG_AUTHORITY_CONTRACT_TYPE,
	ARG_AUTHORITY_EXTENSION,
	ENDPOINT_AUTHORITY_GET_CONTRACT,
};
use crate::enums::caller_verification_type::CallerVerificationType;
use crate::enums::contracts_enum::ContractKind;
use crate::errors::CommonError;
use crate::utils::helpers::is_array_contain;
use crate::utils::storage::get_stored_value_from_key;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
	account::AccountHash,
	runtime_args,
	runtime_args::RuntimeArgs,
	ApiError,
	ContractHash,
	Key,
};

pub fn has_authority(key: &str, account: &AccountHash) -> bool {
	let maintainers = get_stored_value_from_key::<Vec<AccountHash>>(key).unwrap_or_revert_with(
		CommonError::NoAuthority
	);

	is_array_contain(&maintainers, account)
}

pub fn is_maintainer(key: &str, account: &AccountHash) -> bool {
	let maintainer = get_stored_value_from_key::<AccountHash>(key).unwrap_or_revert_with(
		CommonError::NoAuthority
	);
	&maintainer == account
}

pub fn get_contract_hash_from_authority_contract(
	authorities_contract_hash: ContractHash,
	kind: ContractKind,
	extension: Option<String>
) -> Result<Option<ContractHash>, ApiError> {
	Ok(
		runtime::call_contract(
			authorities_contract_hash,
			ENDPOINT_AUTHORITY_GET_CONTRACT,
			runtime_args! {
            ARG_AUTHORITY_CONTRACT_TYPE => kind,
            ARG_AUTHORITY_EXTENSION => extension
        }
		)
	)
}

pub fn get_verified_caller(
	caller_verification_type: CallerVerificationType
) -> Result<Key, CommonError> {
	match *runtime::get_call_stack().iter().nth_back(1).to_owned().unwrap_or_revert() {
		CallStackElement::Session { account_hash: calling_account_hash } => {
			if let CallerVerificationType::OnlyContractHash = caller_verification_type {
				return Err(CommonError::InvalidCaller);
			}
			Ok(Key::Account(calling_account_hash))
		}
		| CallStackElement::StoredSession { contract_hash, .. }
		| CallStackElement::StoredContract { contract_hash, .. } => {
			if let CallerVerificationType::OnlyAccountHash = caller_verification_type {
				return Err(CommonError::InvalidCaller);
			}
			Ok(contract_hash.into())
		}
	}
}

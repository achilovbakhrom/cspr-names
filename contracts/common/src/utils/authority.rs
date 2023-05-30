use alloc::{ vec::Vec };
use alloc::string::String;
use casper_contract::contract_api::runtime;

use casper_types::{account::AccountHash, ApiError, ContractHash, runtime_args, runtime_args::RuntimeArgs};
use crate::utils::storage::{
    get_stored_value_from_key
};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use crate::constants::{
    ENDPOINT_AUTHORITY_GET_CONTRACT,
    ARG_AUTHORITY_CONTRACT_TYPE,
    ARG_AUTHORITY_EXTENSION,
};
use crate::enums::contracts_enum::ContractKind;
use crate::errors::CommonError;
use crate::utils::helpers::is_array_contain;

pub fn has_authority(
    key: &str,
    account: &AccountHash
) -> bool {
    let maintainers = get_stored_value_from_key::<Vec<AccountHash>>(key)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    
    
    is_array_contain(&maintainers, account)
}

pub fn is_maintainer(
    key: &str,
    account: &AccountHash
) -> bool {
    let maintainer = get_stored_value_from_key::<AccountHash>(key)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    &maintainer == account
}

pub fn get_contract_hash_from_authority_contract(authorities_contract_hash: ContractHash, kind: ContractKind, extension: Option<String>) -> Result<Option<ContractHash>, ApiError> {
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
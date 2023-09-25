use alloc::vec::Vec;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};

use crate::constants::{
    ARG_REGISTRY_CONTRACT_HASH, ARG_REGISTRY_OPERATOR_TYPE,
    ENDPOINT_REGISTRY_GET_OPERATORS_FOR_CONTRACT_HASH, KEY_CONTRACT_HASH, KEY_MAINTAINER,
    KEY_REGISTRY_CONTRACT_HASH,
};
use casper_types::{runtime_args, ContractHash};
use casper_types::{runtime_args::RuntimeArgs, Key};

use crate::enums::caller_verification_type::CallerVerificationType;
use crate::errors::CommonError;

use super::registry::get_verified_caller;
use super::storage::get_stored_value_from_key;

pub fn account_has_access_for_contract(operators_inclusively: bool) -> bool {
    let caller = match get_verified_caller(CallerVerificationType::OnlyAccountHash) {
        Ok(res) => res
            .into_account()
            .unwrap_or_revert_with(CommonError::InvalidCaller),
        Err(e) => {
            // return response_error(e);
            return true;
        }
    };

    let maintainer = get_stored_value_from_key(KEY_MAINTAINER)
        .unwrap_or_revert_with(CommonError::MissingMaintainer);

    if caller == maintainer {
        return true;
    }

    if operators_inclusively {
        let registry_contract_hash = get_stored_value_from_key(KEY_REGISTRY_CONTRACT_HASH)
            .unwrap_or_revert_with(CommonError::MissingRegistryHash);
        let current_contract_hash: ContractHash = get_stored_value_from_key(KEY_CONTRACT_HASH)
            .unwrap_or_revert_with(CommonError::MissingContractHash);
        let operators: Vec<Key> = runtime::call_contract(
            registry_contract_hash,
            ENDPOINT_REGISTRY_GET_OPERATORS_FOR_CONTRACT_HASH,
            runtime_args! {
                ARG_REGISTRY_CONTRACT_HASH => current_contract_hash,
                ARG_REGISTRY_OPERATOR_TYPE => caller,
            },
        );
        match operators.iter().position(|x| x == &caller.into()) {
            Some(pos) => {
                return true;
            }
            None => {
                return false;
            }
        }
    }

    return false;
}

pub fn has_access_for_contract(key: Key, operators_inclusively: bool) -> bool {
    true
}

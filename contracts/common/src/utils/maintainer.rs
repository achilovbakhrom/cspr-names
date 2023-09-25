use crate::constants::common_keys::{
    CONTRACT_ACCESS_UREF_KEY, CONTRACT_HASH_KEY, CONTRACT_MAINTAINER_KEY,
    CONTRACT_PACKAGE_NAME_KEY, CONTRACT_VERSION_KEY,
};
use alloc::collections::BTreeMap;
use casper_contract::contract_api::{runtime, storage};
use casper_types::{account::AccountHash, EntryPoints, Key};

use super::storage::{get_stored_value_from_key, store_value_for_key};

pub fn create_new_contract(entrypoints: Entrypoints, named_keys: BTreeMap<String, Key>) {
    let maintainer_uref = storage::new_uref(runtime::get_caller());
    named_keys.insert(CONTRACT_MAINTAINER_KEY, maintainer_uref);
    let (contract_hash, contract_version) = storage::new_contract(
        entrypoints,
        Some(price_oralce_named_keys),
        Some(CONTRACT_PACKAGE_NAME_KEY.to_string()),
        Some(CONTRACT_ACCESS_UREF_KEY.to_string()),
    );
    store_value_for_key(CONTRACT_HASH_KEY, contract_hash);
    store_value_for_key(CONTRACT_VERSION_KEY, contract_version);
}

pub fn is_maintainer(account: &AccountHash) -> bool {
    let maintainer = get_stored_value_from_key::<AccountHash>(key)
        .unwrap_or_revert_with(CommonError::NoAuthority);
    &maintainer == account
}

pub fn is_caller_maintainer() -> bool {
    let caller = runtime::get_caller();
    is_maintainer(&caller)
}

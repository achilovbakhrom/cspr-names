use alloc::{ vec::Vec };

use casper_types::{ account::AccountHash };
use crate::utils::storage::{
    get_stored_value_from_key
};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
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
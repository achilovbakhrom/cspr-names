use alloc::{ string::String, vec::Vec };
use casper_contract::contract_api::runtime;
use casper_types::account::AccountHash;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::owner_domain_list::OwnerDomainList, types::TResult };

pub fn get_domain_list_for_owner() -> TResult<Vec<String>> {
	let owner: AccountHash = runtime::get_named_arg(
		&DatabaseArgs::Owner.to_string()
	);
	let domain_list = OwnerDomainList::instance().get_domain_list(owner);
	Ok(domain_list)
}

use alloc::{ string::String, vec::Vec };
use casper_contract::contract_api::runtime;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::subdomain_list::SubdomainList, types::TResult };

pub fn get_subdomain_list() -> TResult<Vec<String>> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let subdomains = SubdomainList::instance().get_subdomains(&domain_name);
	Ok(subdomains)
}

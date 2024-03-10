use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{
	db::{
		state::TotalState,
		subdomain_entity::SubdomainEntityStore,
		subdomain_list::SubdomainList,
	},
	types::TResult,
};

pub fn remove_subdomain_name() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let subdomain_name: String = runtime::get_named_arg(
		&DatabaseArgs::SubdomainName.to_string()
	);
	SubdomainEntityStore::instance().remove(&subdomain_name);
	match SubdomainList::instance().remove(&domain_name, &subdomain_name) {
		Ok(()) => {}
		Err(e) => {
			return Err(e);
		}
	}
	TotalState::instance().decrement_subdomains_count();
	Ok(())
}

use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use common_lib::{ constants::common_keys::DatabaseArgs, models::SubdomainName };

use crate::{
	db::{
		state::TotalState,
		subdomain_entity::SubdomainEntityStore,
		subdomain_list::SubdomainList,
	},
	types::TResult,
};

pub fn save_subdomain_name() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let subdomain_name: SubdomainName = runtime::get_named_arg(
		&DatabaseArgs::SubdomainName.to_string()
	);
	SubdomainEntityStore::instance().save(subdomain_name.clone());
	match SubdomainList::instance().add(&domain_name, &subdomain_name) {
		Ok(()) => {}
		Err(e) => {
			return Err(e);
		}
	}
	TotalState::instance().increment_subdomains_count();
	Ok(())
}

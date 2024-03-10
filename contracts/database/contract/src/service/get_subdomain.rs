use casper_contract::contract_api::runtime;
use common_lib::{ constants::common_keys::DatabaseArgs, models::SubdomainName };

use crate::{ db::subdomain_entity::SubdomainEntityStore, types::TResult };

pub fn get_subdomain() -> TResult<Option<SubdomainName>> {
	let subdomain_name: String = runtime::get_named_arg(
		&DatabaseArgs::SubdomainName.to_string()
	);
	let subdomain = SubdomainEntityStore::instance().get(&subdomain_name);
	Ok(subdomain)
}

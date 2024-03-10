use alloc::string::String;
use casper_contract::contract_api::runtime;
use common_lib::{ constants::common_keys::DatabaseArgs, models::DomainName };

use crate::{ db::domain_entity::DomainEntityStore, types::TResult };

pub fn get_domain() -> TResult<Option<DomainName>> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let domain = DomainEntityStore::instance().get(&domain_name);
	Ok(domain)
}

use alloc::string::ToString;
use casper_contract::contract_api::runtime;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::subdomain_entity::SubdomainEntityStore, types::TResult };

pub fn set_subdomain_resolver() -> TResult<()> {
	let subdomain_name: String = runtime::get_named_arg(
		&DatabaseArgs::SubdomainName.to_string()
	);
	let resolver: AccountHash = runtime::get_named_arg(
		&DatabaseArgs::Resolver.to_string()
	);

	match
		SubdomainEntityStore::instance().update_resolver(&subdomain_name, resolver)
	{
		Ok(()) => {}
		Err(e) => response_error(e),
	}
	Ok(())
}

use alloc::string::ToString;
use casper_contract::contract_api::runtime;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::domain_entity::DomainEntityStore, types::TResult };

pub fn set_domain_resolver() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let resolver: AccountHash = runtime::get_named_arg(
		&DatabaseArgs::Resolver.to_string()
	);

	match
		DomainEntityStore::instance().update_resolver_address(
			&domain_name,
			resolver
		)
	{
		Ok(()) => {}
		Err(e) => {
			return Err(e);
		}
	}
	Ok(())
}

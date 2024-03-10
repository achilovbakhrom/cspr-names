use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::domain_entity::DomainEntityStore, types::TResult };

pub fn set_domain_expiration() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let expiration_date: u64 = runtime::get_named_arg(
		&DatabaseArgs::ExpirationDate.to_string()
	);

	match
		DomainEntityStore::instance().update_expiration_date(
			&domain_name,
			expiration_date
		)
	{
		Ok(()) => {}
		Err(e) => {
			return Err(e);
		}
	}
	Ok(())
}

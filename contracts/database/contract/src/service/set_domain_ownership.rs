use alloc::string::ToString;
use casper_contract::contract_api::runtime;
use casper_types::Key;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::domain_entity::DomainEntityStore, types::TResult };

pub fn set_domain_ownership() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let owner: Key = runtime::get_named_arg(&DatabaseArgs::Owner.to_string());

	match DomainEntityStore::instance().update_owner(&domain_name, owner) {
		Ok(()) => {}
		Err(e) => {
			return Err(e);
		}
	}
	Ok(())
}

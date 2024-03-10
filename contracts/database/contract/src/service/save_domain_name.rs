use alloc::string::ToString;
use casper_contract::contract_api::runtime;
use common_lib::{ constants::common_keys::DatabaseArgs, models::DomainName };

use crate::{
	db::{
		domain_entity::DomainEntityStore,
		domain_list::DomainListStore,
		domain_pagination_map::DomainPaginationMapStore,
		owner_domain_list::OwnerDomainList,
		state::TotalState,
	},
	types::TResult,
};

pub fn save_domain_name() -> TResult<()> {
	// 100% sure that data is correct, no need extra validations
	let domain_name: DomainName = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);

	DomainEntityStore::instance().save(domain_name.clone());
	let page = match DomainListStore::instance().add(&domain_name.name) {
		Ok(page) => page,
		Err(e) => {
			return Err(e);
		}
	};
	DomainPaginationMapStore::instance().map(&domain_name.name, page);

	OwnerDomainList::instance().add_domain_name(
		domain_name.owner,
		&domain_name.name
	);
	TotalState::instance().increment_domains_count();

	Ok(())
}

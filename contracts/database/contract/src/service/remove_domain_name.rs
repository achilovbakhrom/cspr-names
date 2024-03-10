use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use common_lib::{
	constants::common_keys::DatabaseArgs,
	errors::DatabaseErrors,
};

use crate::{
	db::{
		domain_entity::DomainEntityStore,
		domain_list::DomainListStore,
		domain_pagination_map::DomainPaginationMapStore,
		owner_domain_list::OwnerDomainList,
		state::TotalState,
		subdomain_list::SubdomainList,
	},
	types::TResult,
};

pub fn remove_domain_name() -> TResult<()> {
	let domain_name: String = runtime::get_named_arg(
		&DatabaseArgs::DomainName.to_string()
	);
	let domain_map = DomainEntityStore::instance();
	let domain = domain_map.get(&domain_name).expect("Domain is not found");
	domain_map.remove(&domain_name);

	let domain_pagination_map = DomainPaginationMapStore::instance();
	let page_binding = &domain_pagination_map.get_page(&domain_name);
	let page = match page_binding {
		Ok(res) => res,
		Err(e) => {
			return response_error(*e);
		}
	};
	match DomainListStore::instance().remove(*page, &domain_name) {
		Ok(()) => {}
		Err(e) => {
			return Err(e);
		}
	}
	let _ = &domain_pagination_map.remove(&domain_name);
	let subdomain_list = SubdomainList::instance();
	let subdomains = &subdomain_list.get_subdomains(&domain_name);
	subdomains.iter().for_each(|x| {
		SubdomainEntityStore::instance().remove(x);
		let _ = &subdomain_list
			.remove(&domain_name, x)
			.unwrap_or_revert_with(DatabaseErrors::DatabaseUnexpected);
	});
	OwnerDomainList::instance().remove_domain_name(domain.owner, &domain_name);
	let total_state = TotalState::instance();
	total_state.decrement_domains_count();
	let count = subdomains.len() as u64;
	total_state.decrement_subdomains_count_by(count);
	Ok(())
}

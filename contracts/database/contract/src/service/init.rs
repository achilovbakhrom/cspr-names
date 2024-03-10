use crate::{
	db::{
		domain_entity::DomainEntityStore,
		domain_list::DomainListStore,
		domain_pagination_map::DomainPaginationMapStore,
		owner_domain_list::OwnerDomainList,
		subdomain_entity::SubdomainEntityStore,
		subdomain_list::SubdomainList,
	},
	types::TResult,
};

pub fn init() -> TResult<()> {
	DomainListStore::initialize();
	DomainEntityStore::initialize();
	DomainPaginationMapStore::initialize();
	OwnerDomainList::initialize();
	SubdomainList::initialize();
	SubdomainEntityStore::initialize();
	Ok(())
}

#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
mod store;

use alloc::string::{ String, ToString };
use alloc::vec;
use core::u64;

use casper_contract::{ contract_api::{ runtime, storage }, unwrap_or_revert::UnwrapOrRevert };
use casper_types::account::AccountHash;
use casper_types::contracts::NamedKeys;
use casper_types::{
	CLType,
	CLTyped,
	EntryPoint,
	EntryPointAccess,
	EntryPointType,
	EntryPoints,
	Parameter,
};
use common_lib::constants::{
	ARG_DATABASE_EXPIRATION_DATE,
	ARG_DATABASE_OWNER,
	ARG_DATABASE_PAGE,
	ARG_DATABASE_RESOLVER,
	ARG_DATABASE_SUBDOMAIN_NAME,
	ENDPOINT_DATABASE_GET_DOMAIN,
	ENDPOINT_DATABASE_GET_DOMAIN_LIST,
	ENDPOINT_DATABASE_GET_DOMAIN_LIST_FOR_OWNER,
	ENDPOINT_DATABASE_GET_SUBDOMAIN,
	ENDPOINT_DATABASE_GET_SUBDOMAIN_LIST,
	ENDPOINT_DATABASE_GET_TOTALS,
	ENDPOINT_DATABASE_INIT,
	ENDPOINT_DATABASE_REMOVE_DOMAIN_NAME,
	ENDPOINT_DATABASE_REMOVE_SUBDOMAIN_NAME,
	ENDPOINT_DATABASE_SAVE_DOMAIN_NAME,
	ENDPOINT_DATABASE_SAVE_SUBDOMAIN_NAME,
	ENDPOINT_DATABASE_SET_DOMAIN_EXPIRATION,
	ENDPOINT_DATABASE_SET_DOMAIN_OWNERSHIP,
	ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER,
	ENDPOINT_DATABASE_SET_SUBDOMAIN_RESOLVER,
	KEY_DATABASE_CONTRACT_ACCESS_UREF,
	KEY_DATABASE_CONTRACT_HASH,
	KEY_DATABASE_CONTRACT_PACKAGE_NAME,
	KEY_DATABASE_CONTRACT_VERSION,
	KEY_DATABASE_TOTALS_DOMAIN_COUNT,
	KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT,
	KEY_MAINTAINER,
};
use common_lib::errors::DatabaseErrors;
use common_lib::models::SubdomainName;
use common_lib::utils::response::{ response_error, response_success };
use common_lib::{ constants::ARG_DATABASE_DOMAIN_NAME, models::DomainName };

use crate::store::owner_domain_list::OwnerDomainList;
use crate::store::state::TotalState;
use store::{
	domain_list::DomainList,
	domain_map::DomainMap,
	domain_pagination_map::DomainPaginationMap,
	subdomain_list::SubdomainList,
	subdomain_map::SubdomainMap,
};

#[no_mangle]
pub extern "C" fn save_domain_name() {
	// 100% sure that data is correct, no need extra validations
	let domain_name: DomainName = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);

	DomainMap::instance().save(domain_name.clone());
	let page = match DomainList::instance().add(&domain_name.name) {
		Ok(page) => page,
		Err(e) => {
			return response_error(e);
		}
	};
	DomainPaginationMap::instance().map(&domain_name.name, page);

	OwnerDomainList::instance().add_domain_name(domain_name.owner, &domain_name.name);
	TotalState::instance().increment_domains_count();
}

#[no_mangle]
pub extern "C" fn save_subdomain_name() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let subdomain_name: SubdomainName = runtime::get_named_arg(ARG_DATABASE_SUBDOMAIN_NAME);
	SubdomainMap::instance().save(subdomain_name.clone());
	match SubdomainList::instance().add(&domain_name, &subdomain_name) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
	TotalState::instance().increment_subdomains_count();
}

#[no_mangle]
pub extern "C" fn remove_domain_name() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let domain_map = DomainMap::instance();
	let domain = domain_map.get(&domain_name).expect("Domain is not found");
	domain_map.remove(&domain_name);

	let domain_pagination_map = DomainPaginationMap::instance();
	let page_binding = &domain_pagination_map.get_page(&domain_name);
	let page = match page_binding {
		Ok(res) => res,
		Err(e) => {
			return response_error(*e);
		}
	};
	match DomainList::instance().remove(&domain_name, *page) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
	let _ = &domain_pagination_map.remove(&domain_name);
	let subdomain_list = SubdomainList::instance();
	let subdomains = &subdomain_list.get_subdomains(&domain_name);
	subdomains.iter().for_each(|x| {
		SubdomainMap::instance().remove(x);
		let _ = &subdomain_list
			.remove(&domain_name, x)
			.unwrap_or_revert_with(DatabaseErrors::DatabaseUnexpected);
	});

	OwnerDomainList::instance().remove_domain_name(domain.owner, &domain_name);
	let total_state = TotalState::instance();
	total_state.decrement_domains_count();
	let count = subdomains.len() as u64;
	total_state.decrement_subdomains_count_by(count)
}

#[no_mangle]
pub extern "C" fn remove_subdomain_name() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let subdomain_name: String = runtime::get_named_arg(ARG_DATABASE_SUBDOMAIN_NAME);
	SubdomainMap::instance().remove(&subdomain_name);
	match SubdomainList::instance().remove(&domain_name, &subdomain_name) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
	TotalState::instance().decrement_subdomains_count();
}

#[no_mangle]
pub extern "C" fn set_domain_ownership() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let subdomain_name: AccountHash = runtime::get_named_arg(ARG_DATABASE_OWNER);

	match DomainMap::instance().update_owner(&domain_name, subdomain_name) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
}

#[no_mangle]
pub extern "C" fn set_domain_expiration() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let expiration_date: u64 = runtime::get_named_arg(ARG_DATABASE_EXPIRATION_DATE);

	match DomainMap::instance().update_expiration_date(&domain_name, expiration_date) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
}

#[no_mangle]
pub extern "C" fn set_domain_resolver() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let resolver: AccountHash = runtime::get_named_arg(ARG_DATABASE_RESOLVER);

	match DomainMap::instance().update_resolver_address(&domain_name, resolver) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
}

#[no_mangle]
pub extern "C" fn set_subdomain_resolver() {
	let subdomain_name: String = runtime::get_named_arg(ARG_DATABASE_SUBDOMAIN_NAME);
	let resolver: AccountHash = runtime::get_named_arg(ARG_DATABASE_RESOLVER);

	match SubdomainMap::instance().update_resolver(&subdomain_name, resolver) {
		Ok(()) => {}
		Err(e) => response_error(e),
	}
}

#[no_mangle]
pub extern "C" fn get_domain_list_for_owner() {
	let owner: AccountHash = runtime::get_named_arg(ARG_DATABASE_OWNER);
	let domain_list = OwnerDomainList::instance().get_domain_list(owner);
	response_success(domain_list, "Error while converting CL_Value");
}

#[no_mangle]
pub extern "C" fn get_domain_list() {
	let page: u64 = runtime::get_named_arg(ARG_DATABASE_PAGE);
	let domains = DomainList::instance().get_domain_list(page.to_string().as_ref());
	response_success(domains, "Error while converting CL_Value");
}

#[no_mangle]
pub extern "C" fn get_subdomain_list() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let subdomains = SubdomainList::instance().get_subdomains(&domain_name);
	response_success(subdomains, "Error while converting CL_Value");
}

#[no_mangle]
pub extern "C" fn get_totals() {
	let totals = TotalState::instance().get_totals();
	response_success(totals, "Error while converting CL_Value");
}

#[no_mangle]
pub extern "C" fn get_domain() {
	let domain_name: String = runtime::get_named_arg(ARG_DATABASE_DOMAIN_NAME);
	let domain = DomainMap::instance().get(&domain_name);
	response_success(domain, "Error while converting CL_Value");
}

#[no_mangle]
pub extern "C" fn get_subdomain() {
	let subdomain_name: String = runtime::get_named_arg(ARG_DATABASE_SUBDOMAIN_NAME);
	let subdomain = SubdomainMap::instance().get(&subdomain_name);
	response_success(subdomain, "Error while converting CL_Value");
}

#[no_mangle]
pub extern "C" fn init() {
	DomainList::initialize();
	DomainMap::initialize();
	DomainPaginationMap::initialize();
	OwnerDomainList::initialize();
	SubdomainList::initialize();
	SubdomainMap::initialize();
}

/**
 * Endpoints:
 * 1. save_domain_name
 * 2. save_subdomain_name
 * 3. remove_domain_name
 * 4. remove_subdomain_name
 * 5. set_domain_ownership
 * 6. set_domain_expiration
 * 7. set_domain_resolver
 * 8. set_subdomain_resolver
 * 9. get_domain_list
 * 10. get_subdomain_list
 * 11. get_totals
 * 12. get_domain
 * 13. get_subdomain
 */
#[no_mangle]
pub extern "C" fn call() {
	let mut entrypoints = EntryPoints::new();

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_SAVE_DOMAIN_NAME,
			vec![Parameter::new(ARG_DATABASE_DOMAIN_NAME, DomainName::cl_type())],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_SAVE_SUBDOMAIN_NAME,
			vec![
				Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type()),
				Parameter::new(ARG_DATABASE_SUBDOMAIN_NAME, SubdomainName::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_REMOVE_DOMAIN_NAME,
			vec![Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type())],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_REMOVE_SUBDOMAIN_NAME,
			vec![
				Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type()),
				Parameter::new(ARG_DATABASE_SUBDOMAIN_NAME, String::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_SET_DOMAIN_OWNERSHIP,
			vec![
				Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type()),
				Parameter::new(ARG_DATABASE_OWNER, AccountHash::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_SET_DOMAIN_EXPIRATION,
			vec![
				Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type()),
				Parameter::new(ARG_DATABASE_EXPIRATION_DATE, u64::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER,
			vec![
				Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type()),
				Parameter::new(ARG_DATABASE_RESOLVER, AccountHash::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_SET_SUBDOMAIN_RESOLVER,
			vec![
				Parameter::new(ARG_DATABASE_SUBDOMAIN_NAME, String::cl_type()),
				Parameter::new(ARG_DATABASE_RESOLVER, AccountHash::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_GET_DOMAIN_LIST_FOR_OWNER,
			vec![Parameter::new(ARG_DATABASE_OWNER, AccountHash::cl_type())],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_GET_DOMAIN_LIST,
			vec![Parameter::new(ARG_DATABASE_PAGE, u64::cl_type())],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_GET_SUBDOMAIN_LIST,
			vec![Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type())],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_GET_TOTALS,
			vec![],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_GET_DOMAIN,
			vec![Parameter::new(ARG_DATABASE_DOMAIN_NAME, String::cl_type())],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_GET_SUBDOMAIN,
			vec![Parameter::new(ARG_DATABASE_SUBDOMAIN_NAME, String::cl_type())],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			ENDPOINT_DATABASE_INIT,
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	let mut database_named_keys = NamedKeys::new();
	let maintainer_uref = storage::new_uref(runtime::get_caller());
	database_named_keys.insert(KEY_MAINTAINER.to_string(), maintainer_uref.into());

	let domains_count_uref = storage::new_uref(0);
	database_named_keys.insert(
		KEY_DATABASE_TOTALS_DOMAIN_COUNT.to_string(),
		domains_count_uref.into()
	);

	let subdomains_count_uref = storage::new_uref(0);
	database_named_keys.insert(
		KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT.to_string(),
		subdomains_count_uref.into()
	);

	let (contract_hash, version) = storage::new_contract(
		entrypoints,
		Some(database_named_keys),
		Some(KEY_DATABASE_CONTRACT_PACKAGE_NAME.to_string()),
		Some(KEY_DATABASE_CONTRACT_ACCESS_UREF.to_string())
	);

	let contract_hash_uref = storage::new_uref(contract_hash);
	runtime::put_key(KEY_DATABASE_CONTRACT_HASH, contract_hash_uref.into());

	let contract_version_uref = storage::new_uref(version);
	runtime::put_key(KEY_DATABASE_CONTRACT_VERSION, contract_version_uref.into());
}

#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!(
	"target arch should be wasm32: compile with '--target wasm32-unknown-unknown'"
);

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;
mod types;
mod db;
mod init_call;
mod service;

use alloc::string::{ String, ToString };
use alloc::vec;

use casper_contract::{
	contract_api::{ runtime, storage },
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
	Key,
	CLType,
	CLTyped,
	account::AccountHash,
	contracts::{
		EntryPoint,
		EntryPoints,
		EntryPointType,
		EntryPointAccess,
		Parameter,
		NamedKeys,
	},
};

use common_lib::constants::common_keys::{
	DatabaseArgs,
	DatabaseEndpoints,
	ARG_DATABASE_DOMAIN_NAME,
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
use common_lib::enums::controller_roles::ControllerRoles;
use common_lib::errors::DatabaseErrors;
use common_lib::models::{ SubdomainName, DomainName };
use common_lib::utils::contract::{ create_entrypoint, setup_contract_info };
use common_lib::utils::response::{
	controller,
	response_error,
	response_success,
};

use crate::db::state::TotalState;
use db::{
	domain_list::DomainListStore,
	domain_entity::DomainEntityStore,
	domain_pagination_map::DomainPaginationMapStore,
	subdomain_list::SubdomainList,
	subdomain_entity::SubdomainEntityStore,
	owner_domain_list::OwnerDomainList,
};

#[no_mangle]
pub extern "C" fn save_domain_name() {
	controller(
		service::save_domain_name::save_domain_name,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

#[no_mangle]
pub extern "C" fn save_subdomain_name() {
	controller(
		service::save_subdomain_name::save_subdomain_name,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

#[no_mangle]
pub extern "C" fn remove_domain_name() {
	controller(
		service::remove_domain_name::remove_domain_name,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn remove_subdomain_name() {
	controller(
		service::remove_subdomain_name::remove_subdomain_name,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn set_domain_ownership() {
	controller(
		service::set_domain_ownership::set_domain_ownership,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn set_domain_expiration() {
	controller(
		service::set_domain_expiration::set_domain_expiration,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn set_domain_resolver() {
	controller(
		service::set_domain_resolver::set_domain_resolver,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn set_subdomain_resolver() {
	controller(
		service::set_subdomain_resolver::set_subdomain_resolver,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn get_domain_list_for_owner() {
	controller(
		service::get_domain_list_for_owner::get_domain_list_for_owner,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn get_domain_list() {
	controller(
		service::get_domain_list::get_domain_list,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn get_subdomain_list() {
	controller(
		service::get_subdomain_list::get_subdomain_list,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn get_totals() {
	controller(
		service::get_totals::get_totals,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn get_domain() {
	controller(
		service::get_domain::get_domain,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn get_subdomain() {
	controller(
		service::get_subdomain::get_subdomain,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	)
}

#[no_mangle]
pub extern "C" fn init() {
	controller(
		service::get_subdomain::get_subdomain,
		vec![ControllerRoles::OnlyMaintainer]
	)
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
		create_entrypoint(
			&DatabaseEndpoints::SaveDomainName.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::DomainName.to_string(),
					DomainName::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::SaveSubdomainName.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::DomainName.to_string(),
					String::cl_type()
				),
				Parameter::new(
					&DatabaseArgs::SubdomainName.to_string(),
					SubdomainName::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		EntryPoint::new(
			&DatabaseEndpoints::RemoveDomainName.to_string(),
			vec![
				Parameter::new(&DatabaseArgs::DomainName.to_string(), String::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::RemoveSubdomainName.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::DomainName.to_string(),
					String::cl_type()
				),
				Parameter::new(
					&DatabaseArgs::SubdomainName.to_string(),
					String::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::SetDomainOwnership.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::DomainName.to_string(),
					String::cl_type()
				),
				Parameter::new(&DatabaseArgs::Owner.to_string(), AccountHash::cl_type())
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::SetDomainExpiration.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::DomainName.to_string(),
					String::cl_type()
				),
				Parameter::new(
					&DatabaseArgs::ExpirationDate.to_string(),
					u64::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::SetDomainResolver.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::DomainName.to_string(),
					String::cl_type()
				),
				Parameter::new(
					&DatabaseArgs::Resolver.to_string(),
					AccountHash::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::SetSubdomainResolver.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::SubdomainName.to_string(),
					String::cl_type()
				),
				Parameter::new(
					&DatabaseArgs::Resolver.to_string(),
					AccountHash::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::GetDomainListForOwner.to_string(),
			vec![
				Parameter::new(&DatabaseArgs::Owner.to_string(), AccountHash::cl_type())
			],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::GetDomainList.to_string(),
			vec![Parameter::new(&DatabaseArgs::Page.to_string(), u64::cl_type())],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::GetSubdomainList.to_string(),
			vec![
				Parameter::new(&DatabaseArgs::DomainName.to_string(), String::cl_type())
			],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::GetTotals.to_string(),
			vec![],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::GetDomain.to_string(),
			vec![
				Parameter::new(&DatabaseArgs::DomainName.to_string(), String::cl_type())
			],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::GetSubdomain.to_string(),
			vec![
				Parameter::new(
					&DatabaseArgs::SubdomainName.to_string(),
					String::cl_type()
				)
			],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&DatabaseEndpoints::Init.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	let mut database_named_keys = NamedKeys::new();
	let maintainer_uref = storage::new_uref(runtime::get_caller());
	database_named_keys.insert(
		KEY_MAINTAINER.to_string(),
		maintainer_uref.into()
	);

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

	setup_contract_info(entrypoints, database_named_keys);
}

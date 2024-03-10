#![no_std]
#![no_main]

// Contract Hash List per Extension
// Domain <-> Contract Hash
// Get Actual Contract Hash

mod contract_operators_db;
mod domain_contract_hash_map;
mod max_value_db;
mod operators_db;
mod pointer_db;
mod registry_whitelist_db;
mod service;
mod types;

extern crate alloc;

use alloc::{ string::String, collections::BTreeMap };
use alloc::vec::{ self, Vec };

use casper_contract::{
	contract_api::runtime::{ self },
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ account::AccountHash, ContractHash, Key, KeyTag, Tagged };
use common_lib::utils::response::controller;
use common_lib::{
	constants::common_keys::{
		ARG_REGISTRY_ATTR_KEY,
		ARG_REGISTRY_CONTRACT_HASH,
		ARG_REGISTRY_CONTRACT_HASH_LIST,
		ARG_REGISTRY_CONTRACT_HASH_OPERATOR,
		ARG_REGISTRY_CONTRACT_KIND,
		ARG_REGISTRY_DATABASE_CONTRACT_HASH,
		ARG_REGISTRY_DOMAIN_NAME,
		ARG_REGISTRY_NFT_CONTRACT_HASH,
		ARG_REGISTRY_OPERATOR,
		ARG_REGISTRY_OPERATOR_TYPE,
		KEY_REGISTRY_MAINTAINER,
	},
	enums::{
		caller_verification_type::CallerVerificationType,
		contracts_enum::ContractKind,
		controller_roles::ControllerRoles,
	},
	errors::{ CommonError, RegistryErrors },
	models::{
		registry_contract_hash_list::RegistryContractHashList,
		registry_contract_hash_pair::RegistryContractHashPair,
		registry_pointer::CompoundContract,
	},
	utils::{
		registry::get_verified_caller,
		response::{ response_error, response_success },
		storage::get_stored_value_from_key,
	},
};
use contract_operators_db::ContractOperatorsDb;
use max_value_db::MaxValueDb;
use pointer_db::PointStore;
use registry_whitelist_db::RegistryWhitelistStore;

use crate::{
	domain_contract_hash_map::DomainContractHashMap,
	operators_db::OperatorsDb,
};

/**
 *
 * Binds domain name to contract hashes:
 *  - DatabaseContract
 *  - NFTContract
 *
 */

pub extern "C" fn map_domain_name_to_contract_hash() {
	controller(
		service::map_domain_name_to_contract_hash::map_domain_name_to_contract_hash,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

pub extern "C" fn get_contract_hash_for_domain_name() {
	controller(
		service::get_contract_hash_for_domain_name::get_contract_hash_for_domain_name,
		vec![ControllerRoles::OnlyAuthorizedContracts]
	);
}

#[no_mangle]
pub extern "C" fn call() {}

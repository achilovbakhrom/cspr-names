#![no_std]
#![no_main]

// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

mod contract_operators_db;
mod domain_contract_hash_map;
mod max_value_db;
mod operators_db;
mod pointer_db;
mod registry_whitelist_db;

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use casper_contract::{ contract_api::{ runtime::{ self } }, unwrap_or_revert::UnwrapOrRevert };
use casper_types::{ account::AccountHash, ContractHash, Key, KeyTag, Tagged };
use common_lib::{
	constants::{
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
	enums::{ caller_verification_type::CallerVerificationType, contracts_enum::ContractKind },
	errors::{ CommonError, RegistryErrors },
	models::{
		registry_contract_hash_list::RegistryContractHashList,
		registry_contract_hash_pair::RegistryContractHashPair,
		registry_pointer::RegistryPointer,
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

use crate::{ domain_contract_hash_map::DomainContractHashMap, operators_db::OperatorsDb };

pub extern "C" fn map_domain_name_to_contract_hash() {
	let domain_name: String = runtime::get_named_arg(ARG_REGISTRY_DOMAIN_NAME);
	let database_contract_hash: ContractHash = runtime::get_named_arg(
		ARG_REGISTRY_DATABASE_CONTRACT_HASH
	);
	let nft_contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_NFT_CONTRACT_HASH);

	let maintainer: AccountHash = match get_stored_value_from_key(KEY_REGISTRY_MAINTAINER) {
		Some(res) => res,
		None => {
			return response_error(RegistryErrors::MaintainerIsNotSet);
		}
	};

	if runtime::get_caller() != maintainer {
		return response_error(CommonError::InvalidMaintainer);
	}

	let registry_object = RegistryContractHashPair {
		db_contract_hash: database_contract_hash,
		nft_contract_hash,
	};

	DomainContractHashMap::instance().map_domain_name_to_contract_hash(
		domain_name.clone(),
		registry_object
	);
}

pub extern "C" fn get_contract_hash_for_domain_name() {
	let domain_name: String = runtime::get_named_arg(ARG_REGISTRY_DOMAIN_NAME);

	match DomainContractHashMap::instance().get_contract_hash_for_domain_name(domain_name) {
		Some(res) => response_success(res, "Error while converting CL_Typed value"),
		None => response_error(RegistryErrors::RegistryObjectNotFound),
	}
}

pub extern "C" fn set_contract_hash_list() {
	let contract_hash_list: Vec<RegistryContractHashList> = runtime::get_named_arg(
		ARG_REGISTRY_CONTRACT_HASH_LIST
	);

	let maintainer: AccountHash = match get_stored_value_from_key(KEY_REGISTRY_MAINTAINER) {
		Some(res) => res,
		None => {
			return response_error(RegistryErrors::MaintainerIsNotSet);
		}
	};

	if runtime::get_caller() != maintainer {
		return response_error(CommonError::InvalidMaintainer);
	}
	let store = PointStore::instance();

	contract_hash_list.iter().for_each(|contract_hash_item| {
		contract_hash_item.contract_hash_list.iter().for_each(|hash| {
			let pointer = match contract_hash_item.contract_type {
				ContractKind::Database | ContractKind::NFTCore =>
					RegistryPointer {
						contract_hash: *hash,
						count: Some(0),
					},
				_ =>
					RegistryPointer {
						contract_hash: *hash,
						count: None,
					},
			};
			store.add_contract_list(
				contract_hash_item.contract_type,
				pointer,
				contract_hash_item.attr_key.clone()
			)
		})
	})
}

pub extern "C" fn remove_contract_hash_list() {
	let contract_hash_list: Vec<RegistryContractHashList> = runtime::get_named_arg(
		ARG_REGISTRY_CONTRACT_HASH_LIST
	);

	let maintainer: AccountHash = match get_stored_value_from_key(KEY_REGISTRY_MAINTAINER) {
		Some(res) => res,
		None => {
			return response_error(RegistryErrors::MaintainerIsNotSet);
		}
	};

	if runtime::get_caller() != maintainer {
		return response_error(CommonError::InvalidMaintainer);
	}
	let store = PointStore::instance();

	contract_hash_list
		.iter()
		.for_each(|contract_hash_item| {
			contract_hash_item.contract_hash_list
				.iter()
				.for_each(|hash| {
					store.remove_contract_list(
						contract_hash_item.contract_type,
						hash.clone(),
						contract_hash_item.attr_key.clone()
					)
				})
		});
}

pub extern "C" fn get_contract() {
	let contract_kind: ContractKind = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_KIND);
	let attr_key: Option<String> = runtime::get_named_arg(ARG_REGISTRY_ATTR_KEY);

	let operatorsDb = OperatorsDb::instance();
	let list = operatorsDb.get_operators();
	let stack = runtime::get_call_stack();

	let caller_key = get_verified_caller(CallerVerificationType::OnlyContractHash).unwrap_or_revert();

	let tag: KeyTag = caller_key.tag();

	if tag != KeyTag::Hash {
		response_error(CommonError::InvalidCaller);
	}

	let calling_contract: ContractHash = caller_key
		.into_hash()
		.map(ContractHash::new)
		.unwrap_or_revert_with(CommonError::InvalidKey);

	let whitelist = RegistryWhitelistStore::instance().get_contract_hash_list();

	match whitelist.iter().find(|x| x == &&calling_contract) {
		Some(_) => {}
		None => response_error(CommonError::InvalidCaller),
	}

	let store = PointStore::instance();
	let pointer_list = store.get_contract_list(contract_kind, attr_key.clone());

	if pointer_list.is_empty() {
		return response_error(RegistryErrors::InvalidContractHash);
	}

	if let Some(_) = attr_key {
		let max_value = match MaxValueDb::instance().get_max_value(contract_kind) {
			Some(res) => res,
			None => {
				return response_error(RegistryErrors::ContractHashCountExceeded);
			}
		};

		match pointer_list.iter().find(|x| x.count.unwrap_or(0) < (max_value as i32)) {
			Some(res) => {
				return response_success(res.contract_hash, "Error while converting CL_Typed value!");
			}
			None => {
				return response_error(RegistryErrors::ContractHashCountExceeded);
			}
		}
	} else {
		match pointer_list.last() {
			Some(res) => {
				return response_success(res.contract_hash, "Error while converting CL_Typed value!");
			}
			None => {
				return response_error(CommonError::UnknowError);
			}
		};
	}
}

pub extern "C" fn increment_count_of_contract() {
	let contract_kind: ContractKind = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_KIND);
	let attr_key: String = runtime::get_named_arg(ARG_REGISTRY_ATTR_KEY);
	let contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH);

	let instance = PointStore::instance();
	match instance.increment_contract_hash_count(contract_kind, attr_key, contract_hash) {
		Ok(_) => {}
		Err(e) => {
			return response_error(e);
		}
	}
}

pub extern "C" fn decrement_count_of_contract() {
	let contract_kind: ContractKind = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_KIND);
	let attr_key: String = runtime::get_named_arg(ARG_REGISTRY_ATTR_KEY);
	let contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH);

	let instance = PointStore::instance();
	match instance.decrement_contract_hash_count(contract_kind, attr_key, contract_hash) {
		Ok(_) => {}
		Err(e) => {
			return response_error(e);
		}
	}
}

pub extern "C" fn add_operator() {
	let account_hash: AccountHash = runtime::get_named_arg(ARG_REGISTRY_OPERATOR);
	let instance = OperatorsDb::instance();

	let maintainer: AccountHash = match get_stored_value_from_key(KEY_REGISTRY_MAINTAINER) {
		Some(res) => res,
		None => {
			return response_error(RegistryErrors::MaintainerIsNotSet);
		}
	};
	let caller = runtime::get_caller();
	if caller != maintainer {
		let operators = instance.get_operators();
		match operators.iter().position(|x| x == &caller) {
			Some(res) => {}
			None => {
				return response_error(RegistryErrors::InvalidCaller);
			}
		}
	}

	instance.save_operator(account_hash)
}

pub extern "C" fn set_operators_for_contract_hash() {
	let contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH);
	let operators: Vec<Key> = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH_OPERATOR);

	let instance = ContractOperatorsDb::instance();

	operators.iter().for_each(|item| {
		instance.add_operator(contract_hash.into(), *item);
	})
}

pub extern "C" fn remove_operators_from_contract_hash() {
	let contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH);
	let operators: Vec<Key> = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH_OPERATOR);

	let instance = ContractOperatorsDb::instance();

	operators.iter().for_each(|item| {
		instance.remove_operator(contract_hash.into(), *item);
	})
}

pub extern "C" fn has_operator_for_contract_hash() {
	let contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH);
	let operator: Key = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH_OPERATOR);

	let instance = ContractOperatorsDb::instance();
	let is_operator: bool = instance.contract_has_operator(contract_hash.into(), operator);
	response_success(is_operator, "Error while converting to CL_Type value");
}

pub extern "C" fn get_operators_for_contract_hash() {
	let contract_hash: ContractHash = runtime::get_named_arg(ARG_REGISTRY_CONTRACT_HASH);
	let operator_type: CallerVerificationType = runtime::get_named_arg(ARG_REGISTRY_OPERATOR_TYPE);

	let instance = ContractOperatorsDb::instance();
	let operators = instance.get_operators(contract_hash.into(), Some(KeyTag::Account));
	response_success(operators, "Error while converting to CL_Type value")
}

pub extern "C" fn remove_operator() {
	let account_hash: AccountHash = runtime::get_named_arg(ARG_REGISTRY_OPERATOR);
	let instance = OperatorsDb::instance();

	let maintainer: AccountHash = match get_stored_value_from_key(KEY_REGISTRY_MAINTAINER) {
		Some(res) => res,
		None => {
			return response_error(RegistryErrors::MaintainerIsNotSet);
		}
	};
	let caller = runtime::get_caller();
	if caller != maintainer {
		let operators = instance.get_operators();
		match operators.iter().position(|x| x == &caller) {
			Some(res) => {}
			None => {
				return response_error(RegistryErrors::InvalidCaller);
			}
		}
	}

	instance.remove_operator(account_hash)
}

#[no_mangle]
pub extern "C" fn call() {}

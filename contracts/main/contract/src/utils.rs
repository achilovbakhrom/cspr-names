use alloc::{ string::{ String, ToString }, vec::{ self, Vec } };
use casper_contract::contract_api::runtime;
use casper_types::runtime_args;
use common_lib::{
	constants::common_keys::AdministrationEndpoints,
	utils::contract::get_administration_contract_hash,
};

pub fn get_allowed_extensions() -> Vec<String> {
	let contract_hash = get_administration_contract_hash();

	runtime::call_contract::<Vec<String>>(
		contract_hash,
		&AdministrationEndpoints::GetAllowedExtensions.to_string(),
		runtime_args! {}
	)
}

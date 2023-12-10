use alloc::vec::{ Vec, self };
use casper_contract::{
	contract_api::runtime,
	unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ Key, runtime_args };

use crate::{ db::store::Store, errors::CommonError };

fn is_key_maintainer(key: Key) -> bool {
	let store = Store::instance();
	let maintainer = store.get_maintainer();
	key == maintainer.into()
}

fn is_caller_maintainer() -> bool {
	let caller = runtime::get_caller();
	is_key_maintainer(caller.into())
}

fn has_permission_calling_contract() -> bool {
	if is_caller_maintainer() {
		return true;
	}
	let store = Store::instance();
	let contract_hash = store
		.get_administration_contract_hash()
		.unwrap_or_revert_with(CommonError::NoAdministrationContractHashStored);

	let authorities: Vec<Key> = runtime::call_contract(
		contract_hash,
		"get_contract_hash_authority",
		runtime_args! {}
	);

	let caller = runtime::get_caller();
	authorities.contains(&caller)
}

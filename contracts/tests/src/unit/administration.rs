use casper_types::{
	runtime_args,
	RuntimeArgs,
	Key,
	account::AccountHash,
	ContractHash,
	PublicKey,
};
use common_lib::{
	constants::common_keys::{
		AdministrationEndpoints,
		AdministrationArgs,
		AdministractionStoreKeys,
		CommonKeys,
	},
	enums::contracts_enum::ContractKind,
};

use crate::utils::UnitTestContext;

const CONTRACT_NAME: &str = "administration-contract.wasm";

/// 1. Test allowed extensions - done
/// 2. Test authority
/// 3. Test limits
/// 4. Test contracts list

#[test]
fn should_test_allowed_extensions() {
	let mut ctx = UnitTestContext::instance(
		2,
		CONTRACT_NAME,
		&CommonKeys::ContractHash.to_string()
	);
	let allowed_extensions: Vec<String> = vec![
		String::from("cspr"),
		String::from("csprx"),
		String::from("supp")
	];
	let args = runtime_args! {
		"allowed_extensions" => allowed_extensions
	};

	ctx.set_allowed_extensions_with_maintainer(args);

	let extensions = ctx.get_allowed_extensions();

	assert_eq!(extensions.len(), 3);
}

#[test]
fn should_test_not_allowed_extensions_with_authority() {
	let mut ctx = UnitTestContext::instance(
		2,
		CONTRACT_NAME,
		&CommonKeys::ContractHash.to_string()
	);

	ctx.fund_account(0);
	ctx.fund_account(1);

	ctx.add_user_to_authority_with_maintainer(
		ctx.accounts.get(0).unwrap().clone()
	);

	ctx.fund_account(1);

	let allowed_extensions: Vec<String> = vec![String::from("cspr")];
	ctx.set_allowed_extensions(
		&ctx.accounts.get(1).unwrap().clone(),
		runtime_args! {
		"allowed_extensions" => allowed_extensions.clone(),
	},
		false
	);

	ctx.set_allowed_extensions(
		&ctx.accounts.get(0).unwrap().clone(),
		runtime_args! {
			"allowed_extensions" => allowed_extensions.clone(),
		},
		true
	);

	let extensions = ctx.get_allowed_extensions();
	assert_eq!(extensions.len(), 1)
}

#[test]
fn should_test_authority() {
	let mut ctx = UnitTestContext::instance(
		2,
		CONTRACT_NAME,
		&CommonKeys::ContractHash.to_string()
	);
	ctx.fund_account(0);
	ctx.fund_account(1);

	ctx.add_user_to_authority(
		*ctx.accounts.get(0).unwrap(),
		*ctx.accounts.get(1).unwrap(),
		false
	);

	ctx.add_user_to_authority_with_maintainer(*ctx.accounts.get(1).unwrap());

	let authorities = ctx.get_authorities();
	assert_eq!(authorities.len(), 1);
}

#[test]
fn should_test_limits() {
	let mut ctx = UnitTestContext::instance(
		0,
		CONTRACT_NAME,
		&CommonKeys::ContractHash.to_string()
	);

	let extension = String::from("cspr");
	ctx.set_min_chars_count(extension.clone(), 10);

	let min_chars_count = ctx.get_min_chars_count(extension);

	assert_eq!(min_chars_count, 10);

	ctx.set_listing_limit(ContractKind::NFTCore, 120);
	let nft_limit = ctx.get_listing_limit(ContractKind::NFTCore);
	assert_eq!(nft_limit, 120);

	ctx.set_listing_limit(ContractKind::Database, 120);
	let db_limit = ctx.get_listing_limit(ContractKind::Database);
	assert_eq!(db_limit, 120);
}

#[test]
fn should_test_add_contract() {
	let mut ctx = UnitTestContext::instance(
		0,
		CONTRACT_NAME,
		&CommonKeys::ContractHash.to_string()
	);
	let administration_contract_hash = ContractHash::from([10; 32]);
	ctx.add_contract(
		ContractKind::Administration,
		administration_contract_hash.into(),
		None
	);

	let administration_result = ctx.get_contract(
		ContractKind::Administration,
		false,
		None
	);

	assert_eq!(administration_result.0, administration_contract_hash.into());
	assert_eq!(administration_result.1, None);

	let db_contract_hash = ContractHash::from([11; 32]);

	ctx.add_contract(
		ContractKind::Database,
		db_contract_hash.into(),
		Some(String::from("cspr"))
	);

	let db_contract_result = ctx.get_contract(
		ContractKind::Database,
		true,
		Some(String::from("cspr"))
	);

	assert_eq!(db_contract_result.0, db_contract_hash.into());
	assert_eq!(db_contract_result.1.unwrap(), 0u32);
}

#[test]
fn should_test_contract_increment_and_decrement() {
	let mut ctx = UnitTestContext::instance(
		0,
		CONTRACT_NAME,
		&CommonKeys::ContractHash.to_string()
	);

	let db_contract_hash = ContractHash::from([11; 32]);

	ctx.add_contract(
		ContractKind::Database,
		db_contract_hash.into(),
		Some(String::from("cspr"))
	);

	ctx.increment_contract(
		ContractKind::Database,
		db_contract_hash.into(),
		Some(String::from("cspr"))
	);

	let db_contract_result = ctx.get_contract(
		ContractKind::Database,
		true,
		Some(String::from("cspr"))
	);

	assert_eq!(db_contract_result.0, db_contract_hash.into());
	assert_eq!(db_contract_result.1.unwrap(), 1u32);

	ctx.decrement_contract(
		ContractKind::Database,
		db_contract_hash.into(),
		Some(String::from("cspr"))
	);

	let db_contract_result = ctx.get_contract(
		ContractKind::Database,
		true,
		Some(String::from("cspr"))
	);

	assert_eq!(db_contract_result.0, db_contract_hash.into());
	assert_eq!(db_contract_result.1.unwrap(), 0u32);
}

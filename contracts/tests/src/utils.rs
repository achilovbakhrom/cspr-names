use rand::Rng;
use std::path::PathBuf;
use casper_types::{
	ContractHash,
	URef,
	bytesrepr::{ FromBytes },
	CLTyped,
	runtime_args,
	system::mint,
	U512,
	account::AccountHash,
	ContractPackageHash,
	RuntimeArgs,
	Key,
};
use casper_engine_test_support::{
	WasmTestBuilder,
	InMemoryWasmTestBuilder,
	DEFAULT_ACCOUNT_ADDR,
	DEFAULT_RUN_GENESIS_REQUEST,
	DeployItemBuilder,
	ExecuteRequestBuilder,
	ARG_AMOUNT,
	DEFAULT_PAYMENT,
};
use casper_execution_engine::{
	storage::global_state::in_memory::InMemoryGlobalState,
	core::engine_state::ExecuteRequest,
};

pub(crate) fn get_contract_hash_for_contract_name(
	builder: &WasmTestBuilder<InMemoryGlobalState>,
	contract_name: &str
) -> ContractHash {
	builder
		.get_expected_account(*DEFAULT_ACCOUNT_ADDR)
		.named_keys()
		.get(contract_name)
		.expect("must have contract hash key as part of contract creation")
		.into_hash()
		.map(ContractHash::new)
		.expect("must get contract hash")
		.clone()
}

pub(crate) fn commit_request_with_expecting_success(
	builder: &mut WasmTestBuilder<InMemoryGlobalState>,
	request: ExecuteRequest
) {
	builder.exec(request).commit().expect_success();
}

pub(crate) fn commit_request_with_expecting_failure(
	builder: &mut WasmTestBuilder<InMemoryGlobalState>,
	request: ExecuteRequest
) {
	builder.exec(request).commit().expect_failure();
}

pub(crate) fn get_uref_from_runtime_for_key(
	builder: &WasmTestBuilder<InMemoryGlobalState>,
	contract_hash: ContractHash,
	key: &str
) -> URef {
	*builder
		.query(None, contract_hash.into(), &[])
		.expect("must have cns contract")
		.as_contract()
		.expect("must convert contract")
		.named_keys()
		.get(key)
		.expect(format!("must have key {}", key).as_str())
		.as_uref()
		.expect("must convert to seed uref")
}

pub(crate) fn get_t_from_runtime_for_uref<T: FromBytes + CLTyped>(
	builder: &WasmTestBuilder<InMemoryGlobalState>,
	key_uref: URef
) -> T {
	builder
		.query(None, key_uref.into(), &[])
		.expect("Should be saved address for the domain name")
		.as_cl_value()
		.expect("Should be cl_value")
		.clone()
		.into_t::<T>()
		.expect("Should be a public key")
}

pub(crate) fn create_wasm_builder_and_commit_genesis() -> WasmTestBuilder<InMemoryGlobalState> {
	let mut builder = InMemoryWasmTestBuilder::default();
	builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();
	builder
}

pub fn fund_account(account: &AccountHash) -> ExecuteRequest {
	let mut rng = rand::thread_rng();
	let deploy_item = DeployItemBuilder::new()
		.with_address(*DEFAULT_ACCOUNT_ADDR)
		.with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
		.with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT })
		.with_transfer_args(
			runtime_args! {
            mint::ARG_AMOUNT => U512::from(50_000_000_000_000_u64),
            mint::ARG_TARGET => *account,
            mint::ARG_ID => <Option::<u64>>::None
        }
		)
		.with_deploy_hash(rng.gen())
		.build();

	ExecuteRequestBuilder::from_deploy_item(deploy_item).build()
}

pub enum DeploySource {
	Code(PathBuf),
	ByContractHash {
		hash: ContractHash,
		entry_point: String,
	},
	ByPackageHash {
		package_hash: ContractPackageHash,
		entry_point: String,
	},
	ByContractName {
		name: String,
		entry_point: String,
	},
}

pub fn deploy(
	builder: &mut InMemoryWasmTestBuilder,
	deployer: &AccountHash,
	source: &DeploySource,
	args: RuntimeArgs,
	success: bool,
	block_time: Option<u64>
) {
	let mut rng = rand::thread_rng();
	// let deploy_hash = rng.gen();
	let mut deploy_builder = DeployItemBuilder::new()
		.with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT })
		.with_address(*deployer)
		.with_authorization_keys(&[*deployer])
		.with_deploy_hash(rng.gen());

	deploy_builder = match source {
		DeploySource::Code(path) => deploy_builder.with_session_code(path, args),
		DeploySource::ByContractHash { hash, entry_point } => {
			deploy_builder.with_stored_session_hash(*hash, entry_point, args)
		}
		DeploySource::ByPackageHash { package_hash, entry_point } =>
			deploy_builder.with_stored_versioned_contract_by_hash(
				package_hash.value(),
				None,
				entry_point,
				args
			),
		DeploySource::ByContractName { name, entry_point } => {
			deploy_builder.with_stored_session_named_key(name, entry_point, args)
		}
	};

	let mut execute_request_builder = ExecuteRequestBuilder::from_deploy_item(deploy_builder.build());
	if let Some(ustamp) = block_time {
		execute_request_builder = execute_request_builder.with_block_time(ustamp);
	}
	let exec = builder.exec(execute_request_builder.build());
	(if success { exec.expect_success() } else { exec.expect_failure() }).commit();
}

pub fn query<T: FromBytes + CLTyped>(
	builder: &InMemoryWasmTestBuilder,
	base: Key,
	path: &[String]
) -> T {
	builder
		.query(None, base, path)
		.expect("should be stored value.")
		.as_cl_value()
		.expect("should be cl value.")
		.clone()
		.into_t()
		.expect("Wrong type in query result.")
}

pub fn query_uref(
	builder: &InMemoryWasmTestBuilder,
	base: Key,
	path: &[String],
	dict_key: &str
) -> URef {
	*builder
		.query(None, base, path)
		.expect("should be stored uref value.")
		.as_contract()
		.expect("must convert contract")
		.named_keys()
		.get(dict_key)
		.expect("must have key")
		.as_uref()
		.expect("must convert to seed uref")
}

pub fn query_dictionary<T: FromBytes + CLTyped>(
	builder: &InMemoryWasmTestBuilder,
	base: URef,
	path: &str
) -> T {
	builder
		.query_dictionary_item(None, base, path)
		.expect("should be stored dictionary value.")
		.as_cl_value()
		.expect("should be cl value.")
		.clone()
		.into_t()
		.expect("Wrong type in query result.")
}

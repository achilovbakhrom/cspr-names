use std::path::PathBuf;
// Outlining aspects of the Casper test support crate to include.
use casper_engine_test_support::{
    ExecuteRequestBuilder,
    InMemoryWasmTestBuilder,
    DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,    
};
// Custom Casper types that will be used within this test.
use casper_types::{
    runtime_args,
    ContractHash,
    RuntimeArgs,
    PublicKey,
    SecretKey,
    URef,
    CLValue,
    CLType::Any,
    bytesrepr::ToBytes,
    U512, 
    account::AccountHash,
    Key,
    ApiError
};
use common_lib::{
    enums::price_oracle_contract::PriceType,
    constants::{
        PRICE_ORACLE_CONTRACT_NAME_WASM,
        PRICE_ORACLE_CONTRACT_NAME,
        KEY_PO_CONTRACT_HASH, ENDPOINT_PO_SET_PRICE, KEY_PO_PRICE, ENDPOINT_PO_ADD_AUTHORITY, ENDPOINT_PO_REMOVE_AUTHORITY,
    }
};

use crate::utils::{
    fund_account,
    deploy,
    DeploySource,
    query
};

struct PriceOracleContractContext {
    builder: InMemoryWasmTestBuilder,
    contract_hash: ContractHash,
    alice_account: AccountHash,
    bob_account: AccountHash,
    path_buf: PathBuf
}

impl PriceOracleContractContext {
    
    pub fn deploy() -> Self {
        let alice_public_key: PublicKey =
            PublicKey::from(&SecretKey::ed25519_from_bytes([1u8; 32]).unwrap());
        let bob_public_key: PublicKey =
            PublicKey::from(&SecretKey::ed25519_from_bytes([2u8; 32]).unwrap());
        
        let alice_account = AccountHash::from(&alice_public_key);
        let bob_account = AccountHash::from(&bob_public_key);

        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
        builder
            .exec(fund_account(&alice_account))
            .expect_success()
            .commit();
        builder
            .exec(fund_account(&bob_account))
            .expect_success()
            .commit();
        
        let code = PathBuf::from(PRICE_ORACLE_CONTRACT_NAME_WASM);

        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::Code(code.clone()),
            runtime_args! {},
            true,
            None,
        );
        
        let contract_hash = query(
            &builder,
            Key::Account(alice_account),
            &[KEY_PO_CONTRACT_HASH.to_string()],
        );

        Self { 
            builder,
            contract_hash,
            alice_account,
            bob_account,
            path_buf: code.clone()
        }
    }

    pub fn set_price_correct_data(&mut self) -> bool {

        let args = runtime_args! {
            "arg_price_type" => PriceType::Fixed,
            "arg_price" => U512::from(100),
            "arg_price_mid" => Vec::<u64>::new(),
            "arg_chars_count_mid" => Vec::<u64>::new(),
            "arg_price_more" => 0u64    
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
            },
            args,
            true,
            None
        );
        
        true
    }

    pub fn set_price_incorrect_data(&mut self) -> bool {
        let args = runtime_args! {
            "arg_price_type" => PriceType::Fixed,
            "arg_price" => U512::from(100),
            "arg_price_mid" => Vec::<u64>::new(),
            "arg_chars_count_mid" => Vec::<u64>::new(),
            "arg_price_more" => 0u64    
        };

        deploy(
            &mut self.builder,
            &self.bob_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
            },
            args,
            false,
            None
        );

        false
    }
    
    pub fn get_price(&mut self) -> U512 {
        let price: U512 = query(
            &mut self.builder, 
            self.contract_hash.into(), 
            &[KEY_PO_PRICE.to_string()]
        );
        price
    }

    pub fn add_bob_to_authority(&mut self) -> bool {
        let args = runtime_args! {
            "arg_authority" => self.bob_account
        };
        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_PO_ADD_AUTHORITY.to_string(),
            },
            args,
            true, None
        );
        
        true
    }

    pub fn remove_bob_from_authority(&mut self) -> bool {
        let args = runtime_args! {
            "arg_authority" => self.bob_account
        };
        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_PO_REMOVE_AUTHORITY.to_string(),
            },
            args,
            true, None
        );
        
        true
    }

    pub fn try_to_set_price_with_bobs_account(&mut self) -> bool {
        let args = runtime_args! {
            "arg_price_type" => PriceType::Fixed,
            "arg_price" => U512::from(101),
            "arg_price_mid" => Vec::<u64>::new(),
            "arg_chars_count_mid" => Vec::<u64>::new(),
            "arg_price_more" => 0u64
        };

        deploy(
            &mut self.builder,
            &self.bob_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
            },
            args,
            true,
            None
        );

        true
    }


}


/**
 * SCENARIOS:
 * 
 * 1. Test with correct data
 * 2. Test with incorrect data
 * 3. Try to fix the request with incorrect data
 *   a. add authority
 *   b. remove authority
 */

#[test]
fn should_test_set_price() {
    // deploy with Alice's account, so maintainer is Alice now
    let mut context = PriceOracleContractContext::deploy();
    
    // Try to set price with Alice's account
    let success_set_price_result = context.set_price_correct_data();
    assert_eq!(success_set_price_result, true);
    
    let price = context.get_price();
    assert_eq!(price, U512::from(100u64));

    // Try to set price with Bob's account
    let error_set_price_result = context.set_price_incorrect_data();
    assert_eq!(error_set_price_result, false);

    // Try to add Bob to the authorities table
    let add_authority_result = context.add_bob_to_authority();
    assert_eq!(add_authority_result, true);

    // Change price with Bob's account
    let set_data_bob_account_result = context.try_to_set_price_with_bobs_account();
    assert_eq!(set_data_bob_account_result, true);

    let price = context.get_price();
    assert_eq!(price, U512::from(101u64));

    // Try to remove Bob from the authorities table
    let remove_bob_from_authority_result = context.remove_bob_from_authority();
    assert_eq!(remove_bob_from_authority_result, true);

    // Check  whether Bob's account has access to change the price
    let error_set_price_result = context.set_price_incorrect_data();
    assert_eq!(error_set_price_result, false);
}

use std::{
    path::PathBuf,
    collections::BTreeMap
};

// Outlining aspects of the Casper test support crate to include.
use casper_engine_test_support::{
    DeployItemBuilder,
    ExecuteRequestBuilder,
    InMemoryWasmTestBuilder,
    DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST, DEFAULT_PAYMENT,
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
    U512,
    system::handle_payment::HandlePayment,
    Key,
    account::AccountHash, contracts::NamedKeys,
};

use common_lib::{
    models::{
        DomainName,
        SubdomainName, LocalMetadata
    },
    constants::{
        MAIN_CONTRACT_NAME,
        PRICE_ORACLE_CONTRACT_NAME,
        MAIN_CONTRACT_NAME_WASM,
        PRICE_ORACLE_CONTRACT_NAME_WASM,
        KEY_PO_CONTRACT_HASH,
        KEY_MAIN_CONTRACT_HASH,
        ENDPOINT_PO_SET_PRICE,
        ENTRYPOINT_MAIN_INIT,
        ENTRYPOINT_MAIN_REGISTER_DOMAIN,
        KEY_DATABASE_DICTIONARY_DOMAIN,
        KEY_MAIN_DICTIONARY_DOMAIN_METADATA,
        KEY_DATABASE_DICTIONARY_DOMAIN_LIST,
        ENTRYPOINT_MAIN_EXTEND,
        YEAR_IN_MILLIS, ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN, ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN, KEY_DATABASE_DICTIONARY_SUBDOMAIN, ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN, ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN
    }, enums::price_oracle_contract::PriceType
};
use rand::Rng;

use crate::utils::{
    deploy,
    DeploySource,
    query,
    fund_account,
    query_dictionary, query_uref
};

struct MainContractContext {
    builder: InMemoryWasmTestBuilder,
    main_contract_hash: ContractHash,
    price_oracle_contract_hash: ContractHash,
    alice_account: AccountHash,
    bob_account: AccountHash,
    main_contract_path_buf: PathBuf,
    price_oracle_contract_path_buf: PathBuf,
}

impl MainContractContext {

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

        let price_oracle_contract_path_buf = PathBuf::from(PRICE_ORACLE_CONTRACT_NAME_WASM);
        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::Code(price_oracle_contract_path_buf.clone()),
            runtime_args! {},
            true,
            None,
        );

        let price_oracle_contract_hash: ContractHash = query(
            &builder,
            Key::Account(alice_account),
            &[KEY_PO_CONTRACT_HASH.to_string()],
        );

        let main_contract_path_buf = PathBuf::from(MAIN_CONTRACT_NAME_WASM);
        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::Code(main_contract_path_buf.clone()),
            runtime_args! {
                "arg_price_oracle_contract_hash" => price_oracle_contract_hash
            },
            true,
            None
        );
        let main_contract_hash: ContractHash = query(
            &builder,
            Key::Account(alice_account),
            &[KEY_MAIN_CONTRACT_HASH.to_string()],
        );

        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::ByContractHash {
                hash: main_contract_hash.clone(),
                entry_point: ENTRYPOINT_MAIN_INIT.to_string(),
            },
            runtime_args! {},
            true,
            None
        );

        Self {
            builder,
            main_contract_hash,
            price_oracle_contract_hash,
            alice_account,
            bob_account,
            main_contract_path_buf,
            price_oracle_contract_path_buf
        }

    }

    pub fn set_price(&mut self) -> bool {
        let args = runtime_args! {
            "arg_price_type" => PriceType::Fixed,
            "arg_price" => U512::from(15000),
            "arg_price_mid" => Vec::<u64>::new(),
            "arg_chars_count_mid" => Vec::<u64>::new(),
            "arg_price_more" => 0u64
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.price_oracle_contract_hash,
                entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
            },
            args,
            true,
            None
        );

        true
    }

    pub fn register_domain(&mut self, name: &str) -> bool {

        let resolver_address_pk: PublicKey =
            PublicKey::from(&SecretKey::ed25519_from_bytes([3u8; 32]).unwrap());
        let resolver_account_hash = AccountHash::from(&resolver_address_pk);

        let args = runtime_args! {
            "arg_domain" => name,
            "arg_duration" => 3u8,
            "arg_resolver_address" => resolver_account_hash,
            "arg_amount" => U512::from(3 * 15000),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn query_domain(&mut self, name: &str) -> DomainName {
        let domains_uref = query_uref(
            &mut self.builder,
            self.main_contract_hash.into(),
            &[],
            KEY_DATABASE_DICTIONARY_DOMAIN
        );

        query_dictionary(&mut self.builder, domains_uref, name)
    }

    pub fn query_subdomain(&mut self, name: &str) -> Vec<SubdomainName> {
        let subdomains_uref = query_uref(
            &mut self.builder,
            self.main_contract_hash.into(),
            &[],
            KEY_DATABASE_DICTIONARY_SUBDOMAIN
        );

        query_dictionary(&mut self.builder, subdomains_uref, name)
    }

    pub fn query_metadata(&mut self) -> LocalMetadata {
        query(
            &mut self.builder,
            self.main_contract_hash.into(),
            &[KEY_MAIN_DICTIONARY_DOMAIN_METADATA.to_string()]
        )
    }

    pub fn query_domain_list_0_page(&mut self) -> Vec<String> {
        let domains_uref = query_uref(
            &mut self.builder,
            self.main_contract_hash.into(),
            &[],
            KEY_DATABASE_DICTIONARY_DOMAIN_LIST
        );
        query_dictionary(&mut self.builder, domains_uref, "0")
    }

    pub fn create_2_letter_with_bob(&mut self) -> bool {

        let args = runtime_args! {
            "arg_domain" => "ss.cspr",
            "arg_duration" => 3u8,
            "arg_resolver_address" => self.bob_account,
            "arg_amount" => U512::from(3 * 15000),
        };

        deploy(
            &mut self.builder,
            &self.bob_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            false,
            None,
        );
        false
    }

    pub fn create_2_letter_with_alice(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "ss.cspr",
            "arg_duration" => 3u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(3 * 15000),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );
        true
    }

    pub fn set_dynamic_pricing(&mut self) -> bool {
        let args = runtime_args! {
            "arg_price_type" => PriceType::Dynamic,
            "arg_price" => U512::from(500),
            "arg_price_mid" => vec![U512::from(600), U512::from(700), U512::from(800), U512::from(900)],
            "arg_chars_count_mid" => vec![4u64, 5u64, 6u64, 7u64],
            "arg_price_more" => U512::from(1000)
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.price_oracle_contract_hash,
                entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
            },
            args,
            true,
            None
        );

        true
    }

    pub fn register_domain_name_3_letters(&mut self) -> bool {

        let resolver_address_pk: PublicKey =
            PublicKey::from(&SecretKey::ed25519_from_bytes([4u8; 32]).unwrap());
        let resolver_account_hash = AccountHash::from(&resolver_address_pk);

        let args = runtime_args! {
            "arg_domain" => "sss.cspr",
            "arg_duration" => 1u8,
            "arg_resolver_address" => resolver_account_hash,
            "arg_amount" => U512::from(500),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_name_4_letters(&mut self) -> bool {

        let resolver_address_pk: PublicKey =
            PublicKey::from(&SecretKey::ed25519_from_bytes([5u8; 32]).unwrap());
        let resolver_account_hash = AccountHash::from(&resolver_address_pk);


        let args = runtime_args! {
            "arg_domain" => "ssss.cspr",
            "arg_duration" => 3u8,
            "arg_resolver_address" => resolver_account_hash,
            "arg_amount" => U512::from(3 * 600),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_name_5_letters(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "sssss.cspr",
            "arg_duration" => 1u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(700),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_name_6_letters(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "ssssss.cspr",
            "arg_duration" => 1u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(800),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_name_7_letters(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "sssssss.cspr",
            "arg_duration" => 3u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(3 * 900),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_name_9_letters(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "sssssssss.cspr",
            "arg_duration" => 1u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(1000),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_name_with_1_year_duration(&mut self, name: &str) -> bool {
        let args = runtime_args! {
            "arg_domain" => name,
            "arg_duration" => 1u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(15000u64),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn extend_test_domain_name(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "test.cspr",
            "arg_duration" => 1u8
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_EXTEND.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn register_domain_with_alice(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "good_name.cspr",
            "arg_duration" => 1u8,
            "arg_resolver_address" => self.alice_account,
            "arg_amount" => U512::from(15000u64),
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );
        true
    }

    pub fn change_resolver_to_bob_account(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "good_name.cspr",
            "arg_resolver_address" => self.bob_account
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );
        true
    }

    pub fn try_set_address_with_bobs_account(&mut self) -> bool {
        let args = runtime_args! {
            "arg_domain" => "good_name.cspr",
            "arg_resolver_address" => self.bob_account
        };

        deploy(
            &mut self.builder,
            &self.bob_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN.to_string(),
            },
            args,
            false,
            None,
        );
        false
    }

    pub fn register_subdomain_for(&mut self, subdomain_name: &str) -> bool {
        let args = runtime_args! {
            "arg_subdomain" => subdomain_name,
            "arg_resolver_address" => self.bob_account
        };
        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN.to_string(),
            },
            args,
            true,
            None,
        );
        true
    }

    pub fn change_subdomain_resolver_address(&mut self) -> bool {
        let args = runtime_args! {
            "arg_subdomain" => "sub1.super.cspr",
            "arg_resolver_address" => self.alice_account
        };
        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN.to_string(),
            },
            args,
            true,
            None,
        );

        true
    }

    pub fn try_to_change_subdomain_resolver_address_with_bob(&mut self) -> bool {
        let args = runtime_args! {
            "arg_subdomain" => "sub1.super.cspr",
            "arg_resolver_address" => self.alice_account
        };
        deploy(
            &mut self.builder,
            &self.bob_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN.to_string(),
            },
            args,
            false,
            None,
        );

        false
    }

    pub fn remove_subdomain(&mut self, name: &str) -> bool {
        let args = runtime_args! {
            "arg_subdomain" => name
        };
        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.main_contract_hash,
                entry_point: ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN.to_string(),
            },
            args,
            true,
            None,
        );
        true
    }

}
//
// #[test]
// fn should_test_register_domain_endpoint() {
//
//     let mut context = MainContractContext::deploy();
//     let set_price_result = context.set_price();
//     assert_eq!(set_price_result, true);
//
//     let dn1_result = context.register_domain("bakhrom.cspr");
//     assert_eq!(dn1_result, true);
//
//     let dn1 = context.query_domain("bakhrom.cspr");
//     assert_eq!(&dn1.name, "bakhrom.cspr");
//
//     let dn2_result = context.register_domain("test.cspr");
//     assert_eq!(dn2_result, true);
//
//     let dn2 = context.query_domain("test.cspr");
//     assert_eq!(&dn2.name, "test.cspr");
//
//     let dn3_result = context.register_domain("test3.cspr");
//     assert_eq!(dn3_result, true);
//
//     let dn4_result = context.register_domain("test4.cspr");
//     assert_eq!(dn4_result, true);
//
//     let dn5_result = context.register_domain("test5.cspr");
//     assert_eq!(dn5_result, true);
//
//     let dn6_result = context.register_domain("test6.cspr");
//     assert_eq!(dn6_result, true);
//
//     let dn7_result = context.register_domain("test7.cspr");
//     assert_eq!(dn7_result, true);
//
//     let dn8_result = context.register_domain("test8.cspr");
//     assert_eq!(dn8_result, true);
//
//     let dn9_result = context.register_domain("test9.cspr");
//     assert_eq!(dn9_result, true);
//
//     let dn10_result = context.register_domain("test10.cspr");
//     assert_eq!(dn10_result, true);
//
//     let dn11_result = context.register_domain("test11.cspr");
//     assert_eq!(dn11_result, true);
//
//     let metadata = context.query_metadata();
//
//     assert_eq!(metadata.total_count, 11);
//     assert_eq!(metadata.page, 0);
//
//     let domain_list = context.query_domain_list_0_page();
//     assert_eq!(domain_list.len(), 11);
//
//     assert_eq!(domain_list, [
//         "bakhrom.cspr",
//         "test.cspr",
//         "test3.cspr",
//         "test4.cspr",
//         "test5.cspr",
//         "test6.cspr",
//         "test7.cspr",
//         "test8.cspr",
//         "test9.cspr",
//         "test10.cspr",
//         "test11.cspr",
//     ]);
//
//     let bob_2_letters = context.create_2_letter_with_bob();
//     assert_eq!(bob_2_letters, false);
//
//     let alice_2_letters = context.create_2_letter_with_alice();
//     assert_eq!(alice_2_letters, true);
//
// }
//
// #[test]
// fn should_test_register_with_dynamic_pricing() {
//     let mut context = MainContractContext::deploy();
//     let price_result = context.set_dynamic_pricing();
//     assert_eq!(price_result, true);
//
//     let result_3 = context.register_domain_name_3_letters();
//     assert_eq!(result_3, true);
//
//     let result_4 = context.register_domain_name_4_letters();
//     assert_eq!(result_4, true);
//
//     let result_5 = context.register_domain_name_5_letters();
//     assert_eq!(result_5, true);
//
//     let result_6 = context.register_domain_name_6_letters();
//     assert_eq!(result_6, true);
//
//     let result_7 = context.register_domain_name_7_letters();
//     assert_eq!(result_7, true);
//
//     let result_8 = context.register_domain_name_9_letters();
//     assert_eq!(result_8, true);
// }
//
// #[test]
// fn should_test_extend() {
//     let mut context = MainContractContext::deploy();
//     let price_result = context.set_price();
//     assert_eq!(price_result, true);
//
//     let registration_result = context.register_domain_name_with_1_year_duration("test.cspr");
//     let domain_name = context.query_domain("test.cspr");
//
//     let end_time: u64 = domain_name.end_time;
//     assert_eq!(YEAR_IN_MILLIS, end_time);
//
//     context.extend_test_domain_name();
//
//     let domain_name = context.query_domain("test.cspr");
//     let end_time: u64 = domain_name.end_time;
//     assert_eq!(2 * YEAR_IN_MILLIS, end_time);
// }
//
// #[test]
// fn should_test_set_resolver_address() {
//     let mut context = MainContractContext::deploy();
//     let price_result = context.set_price();
//     assert_eq!(price_result, true);
//     context.register_domain_with_alice();
//     let domain_name = context.query_domain("good_name.cspr");
//     assert_eq!(domain_name.resolver, context.alice_account);
//
//     let bobs_attempt_result = context.try_set_address_with_bobs_account();
//     assert_eq!(bobs_attempt_result, false);
//
//     context.change_resolver_to_bob_account();
//     let domain_name = context.query_domain("good_name.cspr");
//     assert_eq!(domain_name.resolver, context.bob_account);
// }
//
// #[test]
// fn should_test_subdomain_endpoints() {
//     let mut context = MainContractContext::deploy();
//     let price_result = context.set_price();
//
//     let register_dn_result = context.register_domain("super.cspr");
//     assert_eq!(register_dn_result, true);
//
//     let register_sub_domain_name_result = context.register_subdomain_for("sub1.super.cspr");
//     assert_eq!(register_sub_domain_name_result, true);
//
//     let subdomains = context.query_subdomain("super.cspr");
//     assert_eq!(subdomains.len(), 1);
//
//     let subdomain = subdomains.first().expect("Error while unwrapping subdomain");
//     assert_eq!(subdomain.name, "sub1.super.cspr");
//     assert_eq!(subdomain.resolver, context.bob_account);
//
//     let error_change_result = context.try_to_change_subdomain_resolver_address_with_bob();
//     assert_eq!(error_change_result, false);
//
//     let set_resolver_address_of_subdomain_result = context.change_subdomain_resolver_address();
//     assert_eq!(set_resolver_address_of_subdomain_result, true);
//
//     let subdomains = context.query_subdomain("super.cspr");
//     assert_eq!(subdomains.len(), 1);
//
//     let subdomain = subdomains.first().expect("Error while unwrapping subdomain");
//     assert_eq!(subdomain.resolver, context.alice_account);
//
//     let remove_result = context.remove_subdomain("sub1.super.cspr");
//     assert_eq!(remove_result, true);
//     let subdomains = context.query_subdomain("super.cspr");
//     assert_eq!(subdomains.len(), 0);
//
// }

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
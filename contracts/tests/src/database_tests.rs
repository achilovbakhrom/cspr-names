use std::path::PathBuf;

use casper_engine_test_support::{
    DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder, ARG_AMOUNT,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_INITIAL_BALANCE, DEFAULT_GENESIS_CONFIG,
    DEFAULT_GENESIS_CONFIG_HASH, DEFAULT_PAYMENT, DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::core::engine_state::{
    run_genesis_request::RunGenesisRequest, GenesisAccount,
};
use casper_types::{account::AccountHash, runtime_args, Key, Motes, PublicKey, RuntimeArgs, SecretKey, U512, ContractHash};
use common_lib::constants::{CSPR_HASH, ENDPOINT_DATABASE_INIT, ENDPOINT_DATABASE_REMOVE_DOMAIN_NAME, ENDPOINT_DATABASE_REMOVE_SUBDOMAIN_NAME, ENDPOINT_DATABASE_SAVE_DOMAIN_NAME, ENDPOINT_DATABASE_SAVE_SUBDOMAIN_NAME, ENDPOINT_DATABASE_SET_DOMAIN_EXPIRATION, ENDPOINT_DATABASE_SET_DOMAIN_OWNERSHIP, ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER, ENDPOINT_DATABASE_SET_SUBDOMAIN_RESOLVER, KEY_DATABASE_CONTRACT_HASH, KEY_DATABASE_CONTRACT_VERSION, KEY_DATABASE_DICTIONARY_DOMAIN, KEY_DATABASE_DICTIONARY_DOMAIN_LIST, KEY_DATABASE_DICTIONARY_SUBDOMAIN, KEY_DATABASE_SUBDOMAIN_COUNT, KEY_DATABASE_TOTALS_DOMAIN_COUNT, KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT};

use common_lib::models::{DomainName, SubdomainName};
use common_lib::utils::domain_name::namehash_label;


use crate::utils::{
    deploy,
    DeploySource,
    query,
    fund_account,
    query_dictionary, query_uref
};


struct DatabaseContractContext {
    builder: InMemoryWasmTestBuilder,
    contract_hash: ContractHash,
    alice_account: AccountHash,
    bob_account: AccountHash
}

impl DatabaseContractContext {
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

        let database_contract_path_buf = PathBuf::from("database-contract.wasm");
        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::Code(database_contract_path_buf.clone()),
            runtime_args! {},
            true,
            None,
        );

        let contract_hash: ContractHash = query(
            &builder,
            Key::Account(alice_account),
            &[KEY_DATABASE_CONTRACT_HASH.to_string()],
        );

        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::ByContractHash {
                hash: contract_hash.clone(),
                entry_point: ENDPOINT_DATABASE_INIT.to_string(),
            },
            runtime_args! {},
            true,
            None
        );

        Self {
            builder,
            contract_hash,
            alice_account,
            bob_account,
        }
    }

    pub fn create_domain_name(
        &mut self,
        name: &str,
        end_time: u64,
        resolver: AccountHash,
        owner: AccountHash,
        deployer: AccountHash
    ) {
        let domain_name = DomainName {
            end_time,
            owner,
            resolver,
            token_id: "some_hash".to_string(),
            name: name.to_string(),
        };

        let args = runtime_args! {
            "arg_database_domain_name" => domain_name
        };

        deploy(
            &mut self.builder,
            &deployer,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_SAVE_DOMAIN_NAME.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn get_domain_name(&mut self, name: &str) -> Option<DomainName> {
        let domain_uref = query_uref(
            &mut self.builder,
            self.contract_hash.into(),
            &[],
            KEY_DATABASE_DICTIONARY_DOMAIN
        );

        query_dictionary(
            &mut self.builder,
            domain_uref,
            name
        )
    }

    pub fn create_subdomain_name(
        &mut self,
        domain_name: &str,
        subdomain_name: &str,
        resolver: AccountHash,
        deployer: AccountHash
    ) {
        let subdomain = SubdomainName {
            resolver,
            name: subdomain_name.to_string(),
        };
        let args = runtime_args! {
            "arg_database_domain_name" => domain_name,
            "arg_database_subdomain_name" => subdomain
        };

        deploy(
            &mut self.builder,
            &deployer,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_SAVE_SUBDOMAIN_NAME.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn get_subdomain_name(&mut self, name: &str) -> Option<SubdomainName> {
        let subdomain_uref = query_uref(
            &mut self.builder,
            self.contract_hash.into(),
            &[],
            KEY_DATABASE_DICTIONARY_SUBDOMAIN
        );

        query_dictionary(
            &mut self.builder,
            subdomain_uref,
            name
        )
    }

    pub fn remove_domain_name(&mut self, name: &str, deployer: AccountHash) {
        let args = runtime_args! {
            "arg_database_domain_name" => name,
        };

        deploy(
            &mut self.builder,
            &deployer,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_REMOVE_DOMAIN_NAME.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn remove_subdomain_name(&mut self, domain_name: &str, subdomain_name: &str, deployer: AccountHash) {
        let args = runtime_args! {
            "arg_database_domain_name" => domain_name,
            "arg_database_subdomain_name" => subdomain_name
        };

        deploy(
            &mut self.builder,
            &deployer,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_REMOVE_SUBDOMAIN_NAME.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn get_total_state(&mut self) -> (u64, u64) {
        let domain_count: u64 = query(
            &self.builder,
            self.contract_hash.into(),
            &[KEY_DATABASE_TOTALS_DOMAIN_COUNT.to_string()]
        );

        let subdomain_count: u64 = query(
            &self.builder,
            self.contract_hash.into(),
            &[KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT.to_string()]
        );

        (domain_count, subdomain_count)
    }

    pub fn set_domain_ownership(&mut self, name: &str, owner: AccountHash) {
        let args = runtime_args! {
            "arg_database_domain_name" => name,
            "arg_database_owner" => owner
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_SET_DOMAIN_OWNERSHIP.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn set_domain_expiration(&mut self, name: &str, expiration: u64) {
        let args = runtime_args! {
            "arg_database_domain_name" => name,
            "arg_database_expiration_date" => expiration
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_SET_DOMAIN_EXPIRATION.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn set_domain_resolver(&mut self, name: &str, resolver: AccountHash) {
        let args = runtime_args! {
            "arg_database_domain_name" => name,
            "arg_database_resolver" => resolver
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn set_subdomain_resolver(&mut self, name: &str, resolver: AccountHash) {
        let args = runtime_args! {
            "arg_database_subdomain_name" => name,
            "arg_database_resolver" => resolver
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.contract_hash,
                entry_point: ENDPOINT_DATABASE_SET_SUBDOMAIN_RESOLVER.to_string(),
            },
            args,
            true,
            None
        )
    }

    pub fn get_domain_list_for_page(&mut self, page: u64) -> Option<Vec<String>> {
        let domain_list_uref = query_uref(
            &mut self.builder,
            self.contract_hash.into(),
            &[],
            KEY_DATABASE_DICTIONARY_DOMAIN_LIST
        );

        query_dictionary(
            &mut self.builder,
            domain_list_uref,
            &page.to_string()
        )
    }
}


#[test]
fn should_test_save_domain_name() {
    let mut context = DatabaseContractContext::deploy();

    context.create_domain_name(
        "test.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );
    assert_eq!(context.alice_account, context.alice_account);

    let domain = context.get_domain_name("test.cspr").unwrap();
    assert_eq!(domain.name, "test.cspr");
    assert_eq!(domain.end_time, 1_000_000);
    assert_eq!(domain.resolver, context.alice_account);
    assert_eq!(domain.owner, context.alice_account);
}

#[test]
fn should_test_save_subdomain_name() {
    let mut context = DatabaseContractContext::deploy();

    let domain_name = "test.cspr";
    let subdomain_name = "sub.test.cspr";
    context.create_domain_name(
        domain_name,
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name,
        subdomain_name,
        context.alice_account,
        context.alice_account
    );

    let subdomain = context.get_subdomain_name(subdomain_name).unwrap();
    assert_eq!(subdomain.name, subdomain_name);
    assert_eq!(subdomain.resolver, context.alice_account);

}

#[test]
fn should_test_removals() {
    let mut context = DatabaseContractContext::deploy();

    let domain_name = "test.cspr";
    let subdomain_name = "sub.test.cspr";
    context.create_domain_name(
        domain_name,
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name,
        subdomain_name,
        context.alice_account,
        context.alice_account
    );

    let (domain_total, subdomain_total) = context.get_total_state();
    assert_eq!(domain_total, 1);
    assert_eq!(subdomain_total, 1);

    context.remove_subdomain_name(domain_name, subdomain_name, context.alice_account);
    context.remove_domain_name(domain_name, context.alice_account);

    let (domain_total, subdomain_total) = context.get_total_state();
    assert_eq!(domain_total, 0);
    assert_eq!(subdomain_total, 0);

}


#[test]
fn should_test_removals_with_deleting_domain_name_logic() {
    let mut context = DatabaseContractContext::deploy();

    let domain_name_1 = "test1.cspr";
    let subdomain_name_1 = "sub1.test1.cspr";
    let subdomain_name_2 = "sub2.test1.cspr";
    let subdomain_name_3 = "sub3.test1.cspr";
    let subdomain_name_4 = "sub4.test1.cspr";

    let domain_name_2 = "test2.cspr";
    let subdomain_name_5 = "sub1.test2.cspr";

    context.create_domain_name(
        domain_name_1,
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_domain_name(
        domain_name_2,
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name_1,
        subdomain_name_1,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name_1,
        subdomain_name_2,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name_1,
        subdomain_name_3,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name_1,
        subdomain_name_4,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        domain_name_2,
        subdomain_name_5,
        context.alice_account,
        context.alice_account
    );

    let (domain_total, subdomain_total) = context.get_total_state();
    assert_eq!(domain_total, 2);
    assert_eq!(subdomain_total, 5);

    context.remove_domain_name(domain_name_1, context.alice_account);

    let (domain_total, subdomain_total) = context.get_total_state();
    assert_eq!(domain_total, 1);
    assert_eq!(subdomain_total, 1);

}

#[test]
fn should_test_set_ownership() {
    let mut context = DatabaseContractContext::deploy();

    context.create_domain_name(
        "test.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.set_domain_ownership(
        "test.cspr",
        context.bob_account
    );

    let domain: DomainName = context.get_domain_name("test.cspr").unwrap();

    assert_eq!(domain.owner, context.bob_account);
}


#[test]
fn should_test_set_expiration_date() {
    let mut context = DatabaseContractContext::deploy();

    context.create_domain_name(
        "test.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.set_domain_expiration(
        "test.cspr",
        1_100_000
    );

    let domain: DomainName = context.get_domain_name("test.cspr").unwrap();

    assert_eq!(domain.end_time, 1_100_000);
}

#[test]
fn should_test_set_resolver() {
    let mut context = DatabaseContractContext::deploy();

    context.create_domain_name(
        "test.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.set_domain_resolver(
        "test.cspr",
        context.bob_account
    );

    let domain: DomainName = context.get_domain_name("test.cspr").unwrap();

    assert_eq!(domain.resolver, context.bob_account);
}

#[test]
fn should_test_get_domain_list() {
    let mut context = DatabaseContractContext::deploy();
    for x in 1..=25 {

        context.create_domain_name(
            &format!("test{}.cspr", x),
            1_000_000,
            context.alice_account,
            context.alice_account,
            if x > 10 {
                context.alice_account
            } else {
                context.bob_account
            }
        );
    }
    let domain_list: Vec<String> = context.get_domain_list_for_page(0u64).unwrap();
    assert_eq!(domain_list.len(), 10);

    let domain_list: Vec<String> = context.get_domain_list_for_page(1u64).unwrap();
    assert_eq!(domain_list.len(), 10);

    let domain_list: Vec<String> = context.get_domain_list_for_page(2u64).unwrap();
    assert_eq!(domain_list.len(), 5);

    context.remove_domain_name(
        "test1.cspr",
        context.alice_account
    );
    context.remove_domain_name(
        "test2.cspr",
        context.alice_account
    );
    let domain_list: Vec<String> = context.get_domain_list_for_page(0u64).unwrap();
    assert_eq!(domain_list.len(), 8);

    context.remove_domain_name(
        "test11.cspr",
        context.alice_account
    );
    context.remove_domain_name(
        "test12.cspr",
        context.alice_account
    );
    let domain_list: Vec<String> = context.get_domain_list_for_page(1u64).unwrap();
    assert_eq!(domain_list.len(), 8);

    context.create_domain_name(
        "test20.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );
    context.create_domain_name(
        "test21.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_domain_name(
        "test22.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_domain_name(
        "test11.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_domain_name(
        "test12.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    let domain_list: Vec<String> = context.get_domain_list_for_page(0u64).unwrap();
    assert_eq!(domain_list.len(), 10);

    let domain_list: Vec<String> = context.get_domain_list_for_page(1u64).unwrap();
    assert_eq!(domain_list.len(), 10);

    let domain_list: Vec<String> = context.get_domain_list_for_page(2u64).unwrap();
    assert_eq!(domain_list.len(), 6);
}

#[test]
fn should_test_subdomain_set_resolver() {
    let mut context = DatabaseContractContext::deploy();

    context.create_domain_name(
        "test.cspr",
        1_000_000,
        context.alice_account,
        context.alice_account,
        context.alice_account
    );

    context.create_subdomain_name(
        "test.cspr",
        "sub.test.cspr",
        context.alice_account,
        context.alice_account,
    );

    let subdomain: SubdomainName = context.get_subdomain_name("sub.test.cspr").unwrap();
    assert_eq!(subdomain.resolver, context.alice_account);

    context.set_subdomain_resolver(
        "sub.test.cspr",
        context.bob_account
    );

    let subdomain: SubdomainName = context.get_subdomain_name("sub.test.cspr").unwrap();

    assert_eq!(subdomain.resolver, context.bob_account);
}
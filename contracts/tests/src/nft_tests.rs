use std::collections::BTreeMap;
use std::path::PathBuf;
use casper_engine_test_support::{DEFAULT_ACCOUNT_ADDR, DEFAULT_RUN_GENESIS_REQUEST, InMemoryWasmTestBuilder};
use casper_types::account::AccountHash;
use casper_types::{ContractHash, Key, PublicKey, runtime_args, SecretKey};
use common_lib::constants::{ARG_NFT_CONTRACT_HASH, ARG_NFT_METADATA, ARG_NFT_TOKEN_OWNER, ENDPOINT_NFT_MINT, ENDPOINT_NFT_REGISTER_OWNER, ENDPOINT_NFT_SET_NFT_CONTRACT_HASH, get_custom_metadata_schema, KEY_NFT_CONTRACT_HASH};
use crate::utils::{deploy, DeploySource, fund_account, query};
use casper_types::RuntimeArgs;
use common_lib::errors::NFTErrors;
use common_lib::models::nft::Metadata;
use crate::nft_core::utility::constants::{ARG_TOKEN_OWNER, NFT_CONTRACT_WASM, NFT_TEST_COLLECTION, NFT_TEST_SYMBOL, NFT_TEST_TOKEN_SUPPLY, TEST_PRETTY_721_META_DATA};
use crate::nft_core::utility::installer_request_builder::{BurnMode, InstallerRequestBuilder, MetadataMutability, MintingMode, NamedKeyConventionMode, NFTHolderMode, NFTIdentifierMode, NFTKind, NFTMetadataKind, OwnerReverseLookupMode, OwnershipMode, TEST_CUSTOM_METADATA_SCHEMA, WhitelistMode};
use crate::nft_core::utility::support;
use crate::nft_core::utility::support::{get_nft_contract_hash, get_token_page_by_hash};

struct NftContractContext {
    builder: InMemoryWasmTestBuilder,
    nft_contract_hash: ContractHash,
    nft_core_contract_hash: ContractHash,
    alice_account: AccountHash,
    bob_account: AccountHash
}

impl NftContractContext {

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

        let nft_contract_path_buf = PathBuf::from("nft-contract.wasm");
        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::Code(nft_contract_path_buf.clone()),
            runtime_args! {},
            true,
            None,
        );

        let nft_contract_hash: ContractHash = query(
            &builder,
            Key::Account(alice_account),
            &[KEY_NFT_CONTRACT_HASH.to_string()],
        );
        let custom_schema = get_custom_metadata_schema();
        let contract_whitelist = vec![nft_contract_hash.clone()];
        let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_nft_kind(NFTKind::Virtual)
            .with_holder_mode(NFTHolderMode::Mixed)
            .with_whitelist_mode(WhitelistMode::Locked)
            .with_contract_whitelist(contract_whitelist.clone())
            .with_minting_mode(MintingMode::Public as u8)
            .with_nft_metadata_kind(NFTMetadataKind::CustomValidated)
            .with_json_schema(
                serde_json::to_string(&custom_schema)
                    .expect("must convert to json schema")
            )
            .with_identifier_mode(NFTIdentifierMode::Hash)
            .with_metadata_mutability(MetadataMutability::Immutable)
            .with_burn_mode(BurnMode::Burnable)
            // .with_reporting_mode(OwnerReverseLookupMode::NoLookUp)
            .with_named_key_convention_mode(NamedKeyConventionMode::DerivedFromCollectionName)
            .with_total_token_supply(NFT_TEST_TOKEN_SUPPLY)
            .with_collection_name(NFT_TEST_COLLECTION.to_string())
            .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
            .with_allowing_minting(true)
            .build();
        // let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, "nft-core-contract.wasm")
        //     .with_total_token_supply(2u64)
        //     .with_nft_metadata_kind(NFTMetadataKind::CustomValidated)
        //     .with_ownership_mode(OwnershipMode::Transferable)
        //     .with_json_schema(serde_json::to_string(&custom_schema).expect("must convert to json schema"))
        //     // .with_whitelist_mode(WhitelistMode::Unlocked)
        //     // .with_contract_whitelist(contract_whitelist.clone())
        //     // .with_allowing_minting(true)
        //     .build();
        // let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, "nft-core-contract.wasm")
        //     .with_total_token_supply(100u64)
        //     .with_holder_mode(NFTHolderMode::Contracts)
        //     .with_whitelist_mode(WhitelistMode::Locked)
        //     .with_ownership_mode(OwnershipMode::Minter)
        //     .with_minting_mode(MintingMode::Installer as u8)
        //     .with_reporting_mode(OwnerReverseLookupMode::NoLookUp)
        //     .with_contract_whitelist(contract_whitelist.clone())
        //     .with_nft_metadata_kind(NFTMetadataKind::CustomValidated)
        //     .with_json_schema(serde_json::to_string(&custom_schema).expect("must convert to json schema"))
        //     .build();

        builder
            .exec(install_request)
            .expect_success()
            .commit();

        let nft_core_contract_hash = get_nft_contract_hash(&builder);

        deploy(
            &mut builder,
            &alice_account,
            &DeploySource::ByContractHash {
                hash: nft_contract_hash.clone(),
                entry_point: ENDPOINT_NFT_SET_NFT_CONTRACT_HASH.to_string(),
            },
            runtime_args! {
                ARG_NFT_CONTRACT_HASH => nft_core_contract_hash.clone()
            },
            true,
            None
        );

        Self {
            builder,
            nft_contract_hash,
            alice_account,
            bob_account,
            nft_core_contract_hash,
        }
    }



    fn mint(&mut self, token_id: &str, account: AccountHash) -> String {
        let metadata = Metadata::new(token_id.to_string(), token_id.to_string());

        let metadata_res = serde_json::to_string_pretty(&metadata).unwrap();

        println!("metadata_res: {}", metadata_res.to_string());

        let args = runtime_args! {
            ARG_NFT_TOKEN_OWNER => account,
            ARG_NFT_METADATA => metadata_res.to_string()
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.nft_contract_hash.clone(),
                entry_point: ENDPOINT_NFT_MINT.to_string(),
            },
            args,
            true,
            None
        );

        self.metadata_to_hash(metadata_res.to_string())
    }

    fn register_owner(&mut self, account: AccountHash) {
        let token_owner_key: Key = account.into();
        let args = runtime_args! {
            ARG_TOKEN_OWNER => token_owner_key
        };

        deploy(
            &mut self.builder,
            &self.alice_account,
            &DeploySource::ByContractHash {
                hash: self.nft_core_contract_hash.clone(),
                entry_point: ENDPOINT_NFT_REGISTER_OWNER.to_string(),
            },
            args,
            true,
            None
        );
    }

    fn metadata_to_hash(&self, metadata: String) -> String {
        base16::encode_lower(&support::create_blake2b_hash(metadata))
    }

    fn get_page_by_hash(&mut self, hash: String) -> Vec<bool> {
        get_token_page_by_hash(
            &mut self.builder,
            &self.nft_core_contract_hash.into(),
            &Key::Account(self.alice_account),
            hash,
        )
    }
}

#[test]
fn should_test_mint() {
    let mut context = NftContractContext::deploy();
    context.register_owner(context.alice_account);
    let hash = context.mint("token_id", context.alice_account);
    println!("hash: {}", hash.to_string());
    let page = context.get_page_by_hash(hash);
    assert_eq!(page[0], true);
}

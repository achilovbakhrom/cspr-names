use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::storage::global_state::in_memory::InMemoryGlobalState;
use casper_types::{account::AccountHash, runtime_args, CLValue, ContractHash, Key, RuntimeArgs};

use super::utility::{
    constants::{
        ACCESS_KEY_NAME_1_0_0, ACCOUNT_USER_1, ARG_ACCESS_KEY_NAME_1_0_0, ARG_COLLECTION_NAME,
        ARG_HASH_KEY_NAME_1_0_0, ARG_IS_HASH_IDENTIFIER_MODE, ARG_NAMED_KEY_CONVENTION,
        ARG_NFT_CONTRACT_HASH, ARG_NFT_PACKAGE_HASH, ARG_SOURCE_KEY, ARG_TARGET_KEY,
        ARG_TOKEN_HASH, ARG_TOKEN_META_DATA, ARG_TOKEN_OWNER, CONTRACT_1_0_0_WASM,
        ENTRY_POINT_REGISTER_OWNER, MANGLE_NAMED_KEYS, MINT_1_0_0_WASM, MINT_SESSION_WASM,
        NFT_CONTRACT_WASM, NFT_TEST_COLLECTION, NFT_TEST_SYMBOL, PAGE_LIMIT, PAGE_SIZE,
        RECEIPT_NAME, TRANSFER_SESSION_WASM, UNMATCHED_HASH_COUNT, UPDATED_RECEIPTS_WASM,
    },
    installer_request_builder::{
        InstallerRequestBuilder, MetadataMutability, NFTIdentifierMode, NFTMetadataKind,
        NamedKeyConventionMode, OwnershipMode,
    },
    support,
};

const OWNED_TOKENS: &str = "owned_tokens";
const MANGLED_ACCESS_KEY_NAME: &str = "mangled_access_key";
const MANGLED_HASH_KEY_NAME: &str = "mangled_hash_key";

fn get_nft_contract_hash_1_0_0(builder: &WasmTestBuilder<InMemoryGlobalState>) -> ContractHash {
    let nft_hash_addr = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get("nft_contract")
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    ContractHash::new(nft_hash_addr)
}

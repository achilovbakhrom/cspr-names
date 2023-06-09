// Common usage constants

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MetadataSchemaProperty {
    name: String,
    description: String,
    required: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomMetadataSchema {
    properties: BTreeMap<String, MetadataSchemaProperty>,
}

pub fn get_custom_metadata_schema() -> CustomMetadataSchema {
    let mut properties = BTreeMap::new();
    properties.insert(
        "name".to_string(),
        MetadataSchemaProperty {
            name: "name".to_string(),
            description: "The name of domain name.".to_string(),
            required: true,
        },
    );
    properties.insert(
        "symbol".to_string(),
        MetadataSchemaProperty {
            name: "symbol".to_string(),
            description: "The symbol of the token.".to_string(),
            required: true,
        },
    );

    properties.insert(
        "token_id".to_string(),
        MetadataSchemaProperty {
            name: "token_id".to_string(),
            description: "Calculated id of the domain name.".to_string(),
            required: true,
        },
    );
    CustomMetadataSchema { properties }
}

pub const CSPR_HASH: [u8; 32] = [
    0xe0, 0x23, 0xb6, 0xc3, 0x8b, 0x8b, 0xcb, 0xf1, 0xcd, 0x48, 0xc7, 0xac, 0x35, 0xb0, 0xb2, 0x44,
    0x62, 0xfa, 0x8b, 0x66, 0x3b, 0x16, 0x63, 0x07, 0xd8, 0x80, 0xfa, 0xf2, 0x27, 0x1f, 0x67, 0xa6,
];

pub const MAX_DOMAIN_NAME_COUNT_PER_DATABASE: u16 = 10_000;

pub(crate) const GRACE_PERIOD: u64 = 1000 * 60 * 60 * 24 * 90;
pub(crate) const EXTENSION: &str = "cspr";
pub const YEAR_IN_MILLIS: u64 = 1000 * 60 * 60 * 24 * 365;
pub const MAX_PAGE_SIZE: u8 = 10;
pub const MAX_SUBDOMAIN_COUNT: u8 = 50;

pub const MAIN_CONTRACT_NAME_WASM: &str = "main-contract.wasm";
pub const PRICE_ORACLE_CONTRACT_NAME_WASM: &str = "price-oracle-contract.wasm";

pub const MAIN_CONTRACT_NAME: &str = "main-contract";
pub const PRICE_ORACLE_CONTRACT_NAME: &str = "price-oracle-contract";

pub const KEY_MAINTAINER: &str = "key_maintainer";

// ******* Main Contract constants *********

// ARGS
pub const ARG_MAIN_DOMAIN: &str = "arg_domain";
pub const ARG_MAIN_SUBDOMAIN: &str = "arg_subdomain";
pub const ARG_MAIN_DURATION: &str = "arg_duration";
pub const ARG_MAIN_RESOLVER_ADDRESS: &str = "arg_resolver_address";
pub const ARG_MAIN_DOMAIN_PAGE: &str = "arg_domain_page";
pub const ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH: &str = "arg_price_oracle_contract_hash";
pub const ARG_MAIN_REGISTER_AMOUNT: &str = "arg_amount";
pub const ARG_MAIN_AUTHORITY: &str = "arg_authority";
pub const ARG_MAIN_CUSTOMER_PURSE: &str = "arg_main_customer_purse";

// KEYS
pub const KEY_MAIN_MAINTAINER: &str = "key_main_maintainer";
pub const KEY_MAIN_AUTHORITIES: &str = "key_main_authorities";
pub const KEY_MAIN_DICTIONARY_DOMAIN_METADATA: &str = "key_domain_metadata";
pub const KEY_MAIN_DICTIONARY_SUBDOMAIN_METADATA: &str = "key_subdomain_metadata";
pub const KEY_MAIN_SUBDOMAIN_METADATA: &str = "key_subdomain_metadata";
pub const KEY_MAIN_CONTRACT_VERSION: &str = "key_main_contract_version";
pub const KEY_MAIN_CONTRACT_HASH: &str = "key_main_contract_hash";
pub const KEY_MAIN_CONTRACT_PACKAGE_NAME: &str = "key_main_contract_package_name";
pub const KEY_MAIN_CONTRACT_ACCESS_UREF: &str = "key_main_contract_access_uref";
pub const KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH: &str = "key_main_price_oracle_contract_hash";
pub const KEY_MAIN_ALLOWED_EXTENSIONS: &str = "key_main_allowed_extensions";
pub const KEY_MAIN_DATABASE_CONTRACT_HASH_MAP: &str = "key_main_database_contract_hash_map";
pub const KEY_MAIN_NAME_CONTRACT_HASH: &str = "key_main_name_contract_hash";
pub const KEY_MAIN_MAINTAINER_PURSE: &str = "key_main_maintainer_purse";
pub const KEY_MAIN_AUTHORITIES_CONTRACT_HASH: &str = "key_main_authorities_contract_hash";

// ENDPOINTS
pub const ENTRYPOINT_MAIN_REGISTER_DOMAIN: &str = "register_domain";
pub const ENTRYPOINT_MAIN_RESOLVE_DOMAIN: &str = "resolve_domain";
pub const ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN: &str = "set_resolver_address_for_domain";
pub const ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN: &str = "register_sub_domain";
pub const ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN: &str = "remove_subdomain";
pub const ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN: &str =
    "set_resolver_address_for_subdomain";
pub const ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN: &str = "get_sudomains_for_domain";
pub const ENTRYPOINT_MAIN_GET_DOMAIN_LIST: &str = "get_domain_list";
pub const ENTRYPOINT_MAIN_SET_AUTHORITIES_CONTRACT_HASH: &str = "set_authorities_contract_hash";
pub const ENTRYPOINT_MAIN_INIT: &str = "init";
pub const ENTRYPOINT_MAIN_ADD_AUTHORITY: &str = "add_authority";
pub const ENTRYPOINT_MAIN_REMOVE_AUTHORITY: &str = "remove_authority";
pub const ENTRYPOINT_MAIN_EXTEND: &str = "extend";

// ******* Price Oracle Contract constants **********

// ENDPOINTS
pub const ENDPOINT_PO_SET_PRICE: &str = "set_price";
pub const ENDPOINT_PO_GET_PRICE: &str = "get_price";
pub const ENDPOINT_PO_ADD_AUTHORITY: &str = "add_authority";
pub const ENDPOINT_PO_REMOVE_AUTHORITY: &str = "remove_authority";
pub const ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS: &str = "get_price_simple_operations";
pub const ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS: &str = "set_price_simple_operations";
pub const ENDPOINT_PO_INIT: &str = "init";

// ARGS
pub const ARG_PO_PRICE_TYPE: &str = "arg_price_type";
pub const ARG_PO_PRICE: &str = "arg_price";
pub const ARG_PO_CHARS_COUNT_MID: &str = "arg_chars_count_mid";
pub const ARG_PO_PRICE_MID: &str = "arg_price_mid";
pub const ARG_PO_PRICE_MORE: &str = "arg_price_more";
pub const ARG_PO_PRICE_TYPE_CHARS_COUNT: &str = "arg_price_type_chars_count";
pub const ARG_PO_AUTHORITY: &str = "arg_authority";
pub const ARG_PO_EXTENSION: &str = "arg_price_oracle_extension";

// KEYS
pub const KEY_PO_MAINTAINER: &str = "key_price_oracle_maintainer";
pub const KEY_PO_AUTHORITIES: &str = "key_price_oracle_authorities";
pub const KEY_PO_CONTRACT_HASH: &str = "key_storage_price_oracle_contract_hash";
pub const KEY_PO_CONTRACT_VERSION: &str = "key_storage_price_oracle_contract_version";
pub const KEY_PO_PRICE_TYPE: &str = "key_storage_price_type";
pub const KEY_PO_SIMPLE_OPERATIONS: &str = "key_price_oracle_simple_operations";
pub const KEY_PO_PRICE: &str = "key_storage_price";
pub const KEY_PO_PRICE_MID: &str = "key_storage_price_mid";
pub const KEY_PO_CHARS_COUNT_MID: &str = "key_storage_chars_count_mid";
pub const KEY_PO_PRICE_MORE: &str = "key_storage_price_more";
pub const KEY_PO_CONTRACT_PACKAGE_NAME: &str = "key_price_oracle_contract_package_name";
pub const KEY_PO_CONTRACT_ACCESS_UREF: &str = "key_price_oracle_contract_access_uref";
pub const KEY_PO_PRICES: &str = "key_price_oracle_prices";

// ******* Database Contract constants **********

// ARGS
pub const ARG_DATABASE_DOMAIN_NAME: &str = "arg_database_domain_name";
pub const ARG_DATABASE_SUBDOMAIN_NAME: &str = "arg_database_subdomain_name";
pub const ARG_DATABASE_RESOLVER: &str = "arg_database_resolver";
pub const ARG_DATABASE_EXPIRATION_DATE: &str = "arg_database_expiration_date";
pub const ARG_DATABASE_OWNER: &str = "arg_database_owner";
pub const ARG_DATABASE_PAGE: &str = "arg_database_page";

// KEYS
pub const KEY_DATABASE_DICTIONARY_SUBDOMAIN: &str = "key_database_subdomain";
pub const KEY_DATABASE_SUBDOMAIN_COUNT: &str = "key_database_subdomain_count";
pub const KEY_DATABASE_DICTIONARY_SUBDOMAIN_LIST: &str = "key_database_subdomain_list";
pub const KEY_DATABASE_DICTIONARY_DOMAIN: &str = "key_database_domain";
pub const KEY_DATABASE_DICTIONARY_DOMAIN_MAP: &str = "key_database_domain_map";
pub const KEY_DATABASE_DICTIONARY_DOMAIN_LIST: &str = "key_database_domain_list";
pub const KEY_DATABASE_DOMAIN_LIST_PAGINATION: &str = "key_database_domain_list_pagination";
pub const KEY_DATABASE_TOTALS_DOMAIN_COUNT: &str = "key_database_totals_domain_count";
pub const KEY_DATABASE_TOTALS_SUBDOMAIN_COUNT: &str = "key_database_totals_subdomain_count";
pub const KEY_DATABASE_CONTRACT_PACKAGE_NAME: &str = "key_database_contract_package_name";
pub const KEY_DATABASE_CONTRACT_ACCESS_UREF: &str = "key_database_contract_access_uref";
pub const KEY_DATABASE_CONTRACT_VERSION: &str = "key_database_contract_version";
pub const KEY_DATABASE_CONTRACT_HASH: &str = "key_database_contract_hash";
pub const KEY_DATABASE_DICTIONARY_OWNER_DOMAIN_LIST: &str =
    "key_database_dictionary_owner_domain_list";

// ENDPOINTS
pub const ENDPOINT_DATABASE_SAVE_DOMAIN_NAME: &str = "save_domain_name";
pub const ENDPOINT_DATABASE_SAVE_SUBDOMAIN_NAME: &str = "save_subdomain_name";
pub const ENDPOINT_DATABASE_REMOVE_DOMAIN_NAME: &str = "remove_domain_name";
pub const ENDPOINT_DATABASE_REMOVE_SUBDOMAIN_NAME: &str = "remove_subdomain_name";
pub const ENDPOINT_DATABASE_SET_DOMAIN_OWNERSHIP: &str = "set_domain_ownership";
pub const ENDPOINT_DATABASE_SET_DOMAIN_EXPIRATION: &str = "set_domain_expiration";
pub const ENDPOINT_DATABASE_SET_DOMAIN_RESOLVER: &str = "set_domain_resolver";
pub const ENDPOINT_DATABASE_SET_SUBDOMAIN_RESOLVER: &str = "set_subdomain_resolver";
pub const ENDPOINT_DATABASE_GET_DOMAIN_LIST: &str = "get_domain_list";
pub const ENDPOINT_DATABASE_GET_SUBDOMAIN_LIST: &str = "get_subdomain_list";
pub const ENDPOINT_DATABASE_GET_TOTALS: &str = "get_totals";
pub const ENDPOINT_DATABASE_GET_DOMAIN: &str = "get_domain";
pub const ENDPOINT_DATABASE_GET_SUBDOMAIN: &str = "get_subdomain";
pub const ENDPOINT_DATABASE_INIT: &str = "init";
pub const ENDPOINT_DATABASE_GET_DOMAIN_LIST_FOR_OWNER: &str = "get_domain_list_for_owner";

// ******* NFT Contract constants **********

// ARGS
pub const ARG_NFT_DOMAIN_NAME: &str = "arg_nft_domain_name";
pub const ARG_NFT_METADATA: &str = "arg_nft_metadata";
pub const ARG_NFT_CONTRACT_HASH: &str = "arg_nft_contract_hash";
pub const ARG_NFT_TOKEN_OWNER: &str = "arg_nft_token_owner";

// KEYS
pub const KEY_NFT_CORE_CONTRACT_HASH: &str = "key_nft_core_contract_hash";
pub const KEY_NFT_DICTIONARY_LISTING: &str = "key_nft_dictionary_listing";
pub const KEY_NFT_CONTRACT_PACKAGE_NAME: &str = "key_nft_contract_package_name";
pub const KEY_NFT_CONTRACT_ACCESS_UREF: &str = "key_nft_contract_access_uref";
pub const KEY_NFT_CONTRACT_VERSION: &str = "key_nft_contract_version";
pub const KEY_NFT_CONTRACT_HASH: &str = "key_nft_contract_hash";
pub const KEY_NFT_CONTRACT_HASH_NV: &str = "key_nft_contract_hash_nv";
pub const KEY_NFT_CONTRACT_OWNER: &str = "key_nft_contract_owner";
pub const KEY_NFT_OPERATORS: &str = "key_nft_operators";

// ENDPOINTS
pub const ENDPOINT_NFT_MINT: &str = "mint";
pub const ENDPOINT_NFT_TRANSFER: &str = "transfer";
pub const ENDPOINT_NFT_BURN: &str = "burn";
pub const ENDPOINT_NFT_METADATA: &str = "metadata";
pub const ENDPOINT_NFT_REGISTER_OWNER: &str = "register_owner";
pub const ENDPOINT_NFT_LIST: &str = "list";
pub const ENDPOINT_NFT_UN_LIST: &str = "un_list";
pub const ENDPOINT_NFT_BUY: &str = "buy";
pub const ENDPOINT_NFT_SET_NFT_CONTRACT_HASH: &str = "set_nft_contract_hash";
pub const ENDPOINT_NFT_APPROVE: &str = "approve";
pub const ENDPOINT_NFT_SET_CURRENT_CONTRACT_HASH: &str = "set_current_contract_hash";
pub const ENDPOINT_SET_APPROVAL_FOR_ALL: &str = "set_approval_for_all";
pub const ENDPOINT_NFT_INITIALIZE: &str = "init";

// ******* Authorities **********

// ARGS
pub const ARG_AUTHORITY_MUTATION_TYPE: &str = "arg_authority_mutation_type";
pub const ARG_AUTHORITY_CONTRACT_HASH: &str = "arg_authority_contract_hash";
pub const ARG_AUTHORITY_AUTHORITY_LIST: &str = "arg_authority_authority_list";
pub const ARG_AUTHORITY_EXTENSION: &str = "arg_authority_extension";
pub const ARG_AUTHORITY_CONTRACT_TYPE: &str = "arg_authority_contract_type";

// KEYS
pub const KEY_AUTHORITY_AUTHORITIES: &str = "key_authority_authorities";
pub const KEY_AUTHORITY_CONTRACTS: &str = "key_authority_contracts";
pub const KEY_AUTHORITY_CONTRACT_DB: &str = "key_authority_contract_db";
pub const KEY_AUTHORITY_MAINTAINER: &str = "key_authority_maintainer";
pub const KEY_AUTHORITY_CONTRACT_PACKAGE_NAME: &str = "key_authority_contract_package_name";
pub const KEY_AUTHORITY_CONTRACT_ACCESS_UREF: &str = "key_authority_contract_access_uref";
pub const KEY_AUTHORITY_CONTRACT_HASH: &str = "key_authority_contract_hash";
pub const KEY_AUTHORITY_CONTRACT_VERSION: &str = "key_authority_contract_version";

// ENDPOINTS
pub const ENDPOINT_AUTHORITY_SET_AUTHORITY: &str = "set_authority";
pub const ENDPOINT_AUTHORITY_GET_AUTHORITY: &str = "get_authority";
pub const ENDPOINT_AUTHORITY_REMOVE_AUTHORITY: &str = "remove_authority";
pub const ENDPOINT_AUTHORITY_SET_CONTRACT: &str = "set_contract";
pub const ENDPOINT_AUTHORITY_GET_CONTRACT: &str = "get_contract";
pub const ENDPOINT_AUTHORITY_REMOVE_CONTRACT: &str = "remove_contract";

// <-- Registry -->

// Args
pub const ARG_REGISTRY_DOMAIN_NAME: &str = "arg_registry_domain_name";
pub const ARG_REGISTRY_DATABASE_CONTRACT_HASH: &str = "arg_registry_database_contract_hash";
pub const ARG_REGISTRY_NFT_CONTRACT_HASH: &str = "arg_registry_nft_contract_hash";
pub const ARG_REGISTRY_CONTRACT_HASH_LIST: &str = "arg_registry_contract_hash_list";

// Keys
pub const KEY_REGISTRY_OPERATORS: &str = "key_registry_operators";
pub const KEY_REGISTRY_MAINTAINER: &str = "key_registry_maintainer";
pub const KEY_REGISTRY_DOMAIN_CONTRACT_HASH: &str = "key_registry_domain_contract_hash";

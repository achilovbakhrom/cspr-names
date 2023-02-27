


// Common usage constants
pub(crate) const GRACE_PERIOD: u64 = 1000 * 60 * 60 * 24 * 90;
pub(crate) const EXTENSION: &str = "cspr";
pub const YEAR_IN_MILLIS: u64 = 1000 * 60 * 60 * 24 * 365;
pub const MAX_PAGE_SIZE: u8 = 50;
pub const MAX_SUBDOMAIN_COUNT: u8 = 50;

pub const MAIN_CONTRACT_NAME_WASM: &str = "main-contract.wasm";
pub const PRICE_ORACLE_CONTRACT_NAME_WASM: &str = "price-oracle-contract.wasm";

pub const MAIN_CONTRACT_NAME: &str = "main-contract";
pub const PRICE_ORACLE_CONTRACT_NAME: &str = "price-oracle-contract";

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

// KEYS
pub const KEY_MAIN_MAINTAINER: &str = "key_main_maintainer";
pub const KEY_MAIN_AUTHORITIES: &str = "key_main_authorities";
pub const KEY_MAIN_DICTIONARY_DOMAIN: &str = "key_domains";
pub const KEY_MAIN_DICTIONARY_DOMAIN_METADATA: &str = "key_domain_metadata";
pub const KEY_MAIN_DICTIONARY_DOMAIN_LIST: &str = "key_domain_list";
pub const KEY_MAIN_DICTIONARY_SUBDOMAIN: &str = "key_subdomains";
pub const KEY_MAIN_DICTIONARY_SUBDOMAIN_METADATA: &str = "key_subdomain_metadata";
pub const KEY_MAIN_DICTIONARY_SUBDOMAIN_LIST: &str = "key_subdomain_list";
pub const KEY_MAIN_SUBDOMAIN_METADATA: &str = "key_subdomain_metadata";
pub const KEY_MAIN_CONTRACT_VERSION: &str = "key_main_contract_version";
pub const KEY_MAIN_CONTRACT_HASH: &str = "key_main_contract_hash";
pub const KEY_MAIN_CONTRACT_PACKAGE_NAME: &str = "key_main_contract_package_name";
pub const KEY_MAIN_CONTRACT_ACCESS_UREF: &str = "key_main_contract_access_uref";
pub const KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH: &str = "key_main_price_oracle_contract_hash";

// ENDPOINTS
pub const ENTRYPOINT_MAIN_REGISTER_DOMAIN: &str = "register_domain";
pub const ENTRYPOINT_MAIN_RESOLVE_DOMAIN: &str = "resolve_domain";
pub const ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN: &str = "set_resolver_address_for_domain";
pub const ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN: &str = "register_sub_domain";
pub const ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN: &str = "remove_subdomain";
pub const ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN: &str = "set_resolver_address_for_subdomain";
pub const ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN: &str = "get_sudomains_for_domain";
pub const ENTRYPOINT_MAIN_GET_DOMAIN_LIST: &str = "get_domain_list";
pub const ENTRYPOINT_MAIN_SET_PRICE_ORACLE_CONTRACT_HASH: &str = "set_price_oracle_contract_hash";
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

// ARGS
pub const ARG_PO_PRICE_TYPE: &str = "arg_price_type";
pub const ARG_PO_PRICE: &str = "arg_price";
pub const ARG_PO_CHARS_COUNT_MID: &str = "arg_chars_count_mid";
pub const ARG_PO_PRICE_MID: &str = "arg_price_mid";
pub const ARG_PO_PRICE_MORE: &str = "arg_price_more";
pub const ARG_PO_PRICE_TYPE_CHARS_COUNT: &str = "arg_price_type_chars_count";
pub const ARG_PO_AUTHORITY: &str = "arg_authority";

// KEYS
pub const KEY_PO_MAINTAINER: &str = "key_price_oracle_maintainer";
pub const KEY_PO_AUTHORITIES: &str = "key_price_oracle_authorities";
pub const KEY_PO_CONTRACT_HASH: &str = "key_storage_price_oracle_contract_hash";
pub const KEY_PO_CONTRACT_VERSION: &str = "key_storage_price_oracle_contract_version";
pub const KEY_PO_PRICE_TYPE: &str = "key_storage_price_type";
pub const KEY_PO_PRICE: &str = "key_storage_price";
pub const KEY_PO_PRICE_MID: &str = "key_storage_price_mid";
pub const KEY_PO_CHARS_COUNT_MID: &str = "key_storage_chars_count_mid";
pub const KEY_PO_PRICE_MORE: &str = "key_storage_price_more";
pub const KEY_PO_CONTRACT_PACKAGE_NAME: &str = "key_price_oracle_contract_package_name";
pub const KEY_PO_CONTRACT_ACCESS_UREF: &str = "key_price_oracle_contract_access_uref";


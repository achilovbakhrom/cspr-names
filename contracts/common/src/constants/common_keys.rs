// Common usage constants

use core::fmt::write;

use alloc::collections::BTreeMap;
use alloc::fmt;
use alloc::string::{ String, ToString };

use serde::{ Deserialize, Serialize };

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
	properties.insert("name".to_string(), MetadataSchemaProperty {
		name: "name".to_string(),
		description: "The name of domain name.".to_string(),
		required: true,
	});
	properties.insert("symbol".to_string(), MetadataSchemaProperty {
		name: "symbol".to_string(),
		description: "The symbol of the token.".to_string(),
		required: true,
	});

	properties.insert("token_id".to_string(), MetadataSchemaProperty {
		name: "token_id".to_string(),
		description: "Calculated id of the domain name.".to_string(),
		required: true,
	});
	CustomMetadataSchema { properties }
}

pub const CONTRACT_PACKAGE_NAME_KEY: &str = "CONTRACT_PACKAGE_NAME_KEY";
pub const CONTRACT_ACCESS_UREF_KEY: &str = "CONTRACT_ACCESS_UREF_KEY";
pub const CONTRACT_VERSION_KEY: &str = "CONTRACT_VERSION_KEY";
pub const CONTRACT_HASH_KEY: &str = "CONTRACT_HASH_KEY";
pub const CONTRACT_MAINTAINER_KEY: &str = "CONTRACT_MAINTAINER_KEY";
pub const CSPR_HASH: [u8; 32] = [
	0xe0, 0x23, 0xb6, 0xc3, 0x8b, 0x8b, 0xcb, 0xf1, 0xcd, 0x48, 0xc7, 0xac, 0x35, 0xb0,
	0xb2, 0x44, 0x62, 0xfa, 0x8b, 0x66, 0x3b, 0x16, 0x63, 0x07, 0xd8, 0x80, 0xfa, 0xf2,
	0x27, 0x1f, 0x67, 0xa6,
];

pub const MAX_DOMAIN_NAME_COUNT_PER_DATABASE: u16 = 10_000;

pub(crate) const GRACE_PERIOD: u64 = 1000 * 60 * 60 * 24 * 90;
pub(crate) const EXTENSION: &str = "cspr";
pub const YEAR_IN_MILLIS: u64 = 1000 * 60 * 60 * 24 * 365;
pub const MAX_PAGE_SIZE: usize = 10;
pub const MAX_SUBDOMAIN_COUNT: u8 = 50;
pub const DEFAULT_RESPONSE_ERROR_MESSAGE: &str = "Error while parsing argument";

pub const MAIN_CONTRACT_NAME_WASM: &str = "main-contract.wasm";
pub const PRICE_ORACLE_CONTRACT_NAME_WASM: &str = "price-oracle-contract.wasm";

pub const MAIN_CONTRACT_NAME: &str = "main-contract";
pub const PRICE_ORACLE_CONTRACT_NAME: &str = "price-oracle-contract";

pub const KEY_MAINTAINER: &str = "key_maintainer";

// ******* Common constants ********
pub const KEY_CONTRACT_MAINTAINER: &str = "key_contract_maintainer";
pub const KEY_CONTRACT_HASH: &str = "key_contract_hash";
pub const KEY_CONTRACT_VERSION: &str = "key_contract_version";
pub const KEY_REGISTRY_CONTRACT_HASH: &str = "key_registry_contract_hash";
pub const KEY_CONTRACT_PACKAGE_NAME: &str = "key_contract_package_name";
pub const KEY_CONTRACT_ACCESS: &str = "key_contract_access";

// ******* Main Contract constants *********

// ARGS
pub const ARG_MAIN_DOMAIN: &str = "arg_domain";
pub const ARG_MAIN_SUBDOMAIN: &str = "arg_subdomain";
pub const ARG_MAIN_DURATION: &str = "arg_duration";
pub const ARG_MAIN_RESOLVER_ADDRESS: &str = "arg_resolver_address";
pub const ARG_MAIN_DOMAIN_PAGE: &str = "arg_domain_page";
pub const ARG_MAIN_PRICE_ORACLE_CONTRACT_HASH: &str =
	"arg_price_oracle_contract_hash";
pub const ARG_MAIN_REGISTER_AMOUNT: &str = "arg_amount";
pub const ARG_MAIN_AUTHORITY: &str = "arg_authority";
pub const ARG_MAIN_CUSTOMER_PURSE: &str = "arg_main_customer_purse";

// KEYS
pub const KEY_MAIN_MAINTAINER: &str = "key_main_maintainer";
pub const KEY_MAIN_AUTHORITIES: &str = "key_main_authorities";
pub const KEY_MAIN_DICTIONARY_DOMAIN_METADATA: &str = "key_domain_metadata";
pub const KEY_MAIN_DICTIONARY_SUBDOMAIN_METADATA: &str =
	"key_subdomain_metadata";
pub const KEY_MAIN_SUBDOMAIN_METADATA: &str = "key_subdomain_metadata";
pub const KEY_MAIN_CONTRACT_VERSION: &str = "key_main_contract_version";
pub const KEY_MAIN_CONTRACT_HASH: &str = "key_main_contract_hash";
pub const KEY_MAIN_CONTRACT_PACKAGE_NAME: &str =
	"key_main_contract_package_name";
pub const KEY_MAIN_CONTRACT_ACCESS_UREF: &str = "key_main_contract_access_uref";
pub const KEY_MAIN_PRICE_ORACLE_CONTRACT_HASH: &str =
	"key_main_price_oracle_contract_hash";
pub const KEY_MAIN_ALLOWED_EXTENSIONS: &str = "key_main_allowed_extensions";
pub const KEY_MAIN_DATABASE_CONTRACT_HASH_MAP: &str =
	"key_main_database_contract_hash_map";
pub const KEY_MAIN_NAME_CONTRACT_HASH: &str = "key_main_name_contract_hash";
pub const KEY_MAIN_MAINTAINER_PURSE: &str = "key_main_maintainer_purse";
pub const KEY_MAIN_AUTHORITIES_CONTRACT_HASH: &str =
	"key_main_authorities_contract_hash";
pub const KEY_MAIN_REGISTRY_CONTRACT_HASH: &str =
	"key_main_registry_contract_hash";

// ENDPOINTS
pub const ENTRYPOINT_MAIN_REGISTER_DOMAIN: &str = "register_domain";
pub const ENTRYPOINT_MAIN_RESOLVE_DOMAIN: &str = "resolve_domain";
pub const ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_DOMAIN: &str =
	"set_resolver_address_for_domain";
pub const ENTRYPOINT_MAIN_REGISTER_SUB_DOMAIN: &str = "register_sub_domain";
pub const ENTRYPOINT_MAIN_REMOVE_SUBDOMAIN: &str = "remove_subdomain";
pub const ENTRYPOINT_MAIN_SET_RESOLVER_ADDRESS_FOR_SUBDOMAIN: &str =
	"set_resolver_address_for_subdomain";

pub const ENTRYPOINT_MAIN_GET_SUBDOMAINS_FOR_DOMAIN: &str =
	"get_sudomains_for_domain";
pub const ENTRYPOINT_MAIN_GET_DOMAIN_LIST: &str = "get_domain_list";
pub const ENTRYPOINT_MAIN_SET_AUTHORITIES_CONTRACT_HASH: &str =
	"set_authorities_contract_hash";
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
pub const ENDPOINT_PO_PRICE_GET_SIMPLE_OPERATIONS: &str =
	"get_price_simple_operations";
pub const ENDPOINT_PO_PRICE_SET_SIMPLE_OPERATIONS: &str =
	"set_price_simple_operations";
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
pub const KEY_PO_CONTRACT_VERSION: &str =
	"key_storage_price_oracle_contract_version";
pub const KEY_PO_PRICE_TYPE: &str = "key_storage_price_type";
pub const KEY_PO_SIMPLE_OPERATIONS: &str = "key_price_oracle_simple_operations";
pub const KEY_PO_PRICE: &str = "key_storage_price";
pub const KEY_PO_PRICE_MID: &str = "key_storage_price_mid";
pub const KEY_PO_CHARS_COUNT_MID: &str = "key_storage_chars_count_mid";
pub const KEY_PO_PRICE_MORE: &str = "key_storage_price_more";
pub const KEY_PO_CONTRACT_PACKAGE_NAME: &str =
	"key_price_oracle_contract_package_name";
pub const KEY_PO_CONTRACT_ACCESS_UREF: &str =
	"key_price_oracle_contract_access_uref";
pub const KEY_PO_PRICES: &str = "key_price_oracle_prices";

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
pub const ENDPOINT_NFT_SET_CURRENT_CONTRACT_HASH: &str =
	"set_current_contract_hash";
pub const ENDPOINT_SET_APPROVAL_FOR_ALL: &str = "set_approval_for_all";
pub const ENDPOINT_NFT_INITIALIZE: &str = "init";

// <-- Registry -->

// pub enum RegistryContractArgs {
// 	DomainName = "arg_registry_domain_name",
// 	DatabaseContractHash = "arg_registry_domain_name",
// 	NftContractHash = "arg_registry_database_contract_hash",
// 	ContractHashList = "arg_registry_contract_hash_list",
// 	ContractKind = "arg_registry_contract_kind",
// 	ContractHash = "arg_registry_contract_hash",
// 	OperatorType = "arg_registry_operator_type",
// 	AttributeKey = "arg_registry_contract_key",
// 	Operator = "arg_registry_operator",
// 	ContractHashOperator = "arg_registry_contract_hash_operator",
// }

// pub enum RegistryContractKeys {
// 	Operators = "key_registry_operators",
// 	Maintainer = "key_registry_maintainer",
// 	DomainContractHash = "key_registry_domain_contract_hash",
// 	WhitelistContractHash = "key_registry_whitelist_contract_hash",
// 	ContractOperators = "key_registry_contract_operators",
// }

// Args
pub const ARG_REGISTRY_DOMAIN_NAME: &str = "arg_registry_domain_name";
pub const ARG_REGISTRY_DATABASE_CONTRACT_HASH: &str =
	"arg_registry_database_contract_hash";
pub const ARG_REGISTRY_NFT_CONTRACT_HASH: &str =
	"arg_registry_nft_contract_hash";
pub const ARG_REGISTRY_CONTRACT_HASH_LIST: &str =
	"arg_registry_contract_hash_list";
pub const ARG_REGISTRY_CONTRACT_KIND: &str = "arg_registry_contract_kind";
pub const ARG_REGISTRY_CONTRACT_HASH: &str = "arg_registry_contract_hash";
pub const ARG_REGISTRY_OPERATOR_TYPE: &str = "arg_registry_operator_type";
pub const ARG_REGISTRY_ATTR_KEY: &str = "arg_registry_contract_key";
pub const ARG_REGISTRY_OPERATOR: &str = "arg_registry_operator";
pub const ARG_REGISTRY_CONTRACT_HASH_OPERATOR: &str =
	"arg_registry_contract_hash_operator";

// Keys
pub const KEY_REGISTRY_OPERATORS: &str = "key_registry_operators";
pub const KEY_REGISTRY_MAINTAINER: &str = "key_registry_maintainer";
pub const KEY_REGISTRY_DOMAIN_CONTRACT_HASH: &str =
	"key_registry_domain_contract_hash";
pub const KEY_REGISTRY_WHITELIST_CONTRACT_HASH: &str =
	"key_registry_whitelist_contract_hash";
pub const KEY_REGISTRY_CONTRACT_OPERATORS: &str =
	"key_registry_contract_operators";

// Endpoints
pub const ENDPOINT_REGISTRY_MAP_DOMAIN_NAME_TO_CONTRACT_HASH: &str =
	"map_domain_name_to_contract_hash";
pub const ENDPOINT_REGISTRY_GET_CONTRACT_HASH_FOR_DOMAIN_NAME: &str =
	"get_contract_hash_for_domain_name";
pub const ENDPOINT_REGISTRY_SET_CONTRACT_HASH_LIST: &str =
	"set_contract_hash_list";
pub const ENDPOINT_REGISTRY_REMOVE_CONTRACT_HASH_LIST: &str =
	"remove_contract_hash_list";
pub const ENDPOINT_REGISTRY_GET_CONTRACT: &str = "get_contract";
pub const ENDPOINT_REGISTRY_INCREMENT_COUNT_OF_CONTRACT: &str =
	"increment_count_of_contract";
pub const ENDPOINT_REGISTRY_DECREMENT_COUNT_OF_CONTRACT: &str =
	"decrement_count_of_contract";
pub const ENDPOINT_REGISTRY_ADD_OPERATOR: &str = "add_operator";
pub const ENDPOINT_REGISTRY_REMOVE_OPERATOR: &str = "remove_operator";
pub const ENDPOINT_REGISTRY_SET_OPERATORS_FOR_CONTRACT_HASH: &str =
	"set_operators_for_contract_hash";
pub const ENDPOINT_REGISTRY_REMOVE_OPERATORS_FROM_CONTRACT_HASH: &str =
	"remove_operators_from_contract_hash";
pub const ENDPOINT_REGISTRY_HAS_OPERATOR_FOR_CONTRACT_HASH: &str =
	"has_operator_for_contract_hash";
pub const ENDPOINT_REGISTRY_GET_OPERATORS_FOR_CONTRACT_HASH: &str =
	"get_operators_for_contract_hash";

// ADMINISTRATION SCOPE
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AdministractionStoreKeys {
	CharsCount,
	DomainListLimit,
	AllowedExtensions,
	ContractNotExist,
	Authorities,
	ContractAuthority,
}

impl fmt::Display for AdministractionStoreKeys {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::CharsCount => write!(f, "chars_count"),
			Self::DomainListLimit => write!(f, "domain_list_limit"),
			Self::AllowedExtensions => write!(f, "allowed_extensions"),
			Self::ContractNotExist => write!(f, "contract_not_exist"),
			Self::Authorities => write!(f, "authorities"),
			Self::ContractAuthority => write!(f, "contract_authority"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AdministrationArgs {
	AllowedExtensions,
	AllowedExtension,
	ContractKind,
	Extension,
	Key,
	CharsCount,
	ContractHash,
	ContractAuthorities,
	ContractAuthority,
}

impl fmt::Display for AdministrationArgs {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::AllowedExtension => write!(f, "allowed_extension"),
			Self::AllowedExtensions => write!(f, "allowed_extensions"),
			Self::ContractKind => write!(f, "contract_kind"),
			Self::Extension => write!(f, "extension"),
			Self::Key => write!(f, "key"),
			Self::CharsCount => write!(f, "chars_count"),
			Self::ContractHash => write!(f, "contract_hash"),
			Self::ContractAuthorities => write!(f, "contract_authorities"),
			Self::ContractAuthority => write!(f, "contract_authority"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AdministrationEndpoints {
	SetContractAuthorityList,
	AddContractAuthority,
	GetContractAuthorityList,
	RemoveContractAuthority,
	GetContract,
	AddContract,
	IncrementContract,
	DecrementContract,
	SetAllowedExtensions,
	GetAllowedExtensions,
	AddExtension,
	RemoveExtension,
	GetCharsMinCount,
	SetCharsMinCount,
	GetListingLimit,
	SetListingLimit,
}

impl fmt::Display for AdministrationEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::SetContractAuthorityList =>
				write!(f, "set_contract_authority_list"),
			Self::AddContractAuthority => write!(f, "add_contract_authority"),
			Self::GetContractAuthorityList =>
				write!(f, "get_contract_authority_list"),
			Self::RemoveContractAuthority => write!(f, "remove_contract_authority"),
			Self::GetContract => write!(f, "get_contract"),
			Self::AddContract => write!(f, "add_contract"),
			Self::IncrementContract => write!(f, "increment_contract"),
			Self::DecrementContract => write!(f, "decrement_contract"),
			Self::SetAllowedExtensions => write!(f, "set_allowed_extensions"),
			Self::GetAllowedExtensions => write!(f, "get_allowed_extensions"),
			Self::AddExtension => write!(f, "add_extension"),
			Self::RemoveExtension => write!(f, "remove_extension"),
			Self::GetCharsMinCount => write!(f, "get_chars_min_count"),
			Self::SetCharsMinCount => write!(f, "set_chars_min_count"),
			Self::GetListingLimit => write!(f, "get_listing_limit"),
			Self::SetListingLimit => write!(f, "set_listing_limit"),
		}
	}
}

/// NFT Contract Keys

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NFTContractKeys {
	Listing,
	Operator,
	NFTCoreContractHash,
	NFTCoreContractHashCurrent,
}

impl fmt::Display for NFTContractKeys {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Listing => write!(f, "listing"),
			Self::Operator => write!(f, "listing"),
			Self::NFTCoreContractHash => write!(f, "nft_core_contract_hash"),
			Self::NFTCoreContractHashCurrent =>
				write!(f, "nft_core_contract_hash_current"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NFTContractArgs {
	Owner,
	Metadata,
	NftCoreContractHash,
	TokenId,
	SourceKey,
	DestinationKey,
	TokenPrice,
}

impl fmt::Display for NFTContractArgs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Owner => write!(f, "owner"),
			Self::Metadata => write!(f, "metadata"),
			Self::NftCoreContractHash => write!(f, "contract_hash"),
			Self::TokenId => write!(f, "token_id"),
			Self::SourceKey => write!(f, "source_key"),
			Self::DestinationKey => write!(f, "destination_key"),
			Self::TokenPrice => write!(f, "token_price"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NFTContractEndpoints {
	Mint,
	Transfer,
	Burn,
	List,
	UnList,
	Buy,
}

impl fmt::Display for NFTContractEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Mint => write!(f, "mint"),
			Self::Transfer => write!(f, "transfer"),
			Self::Burn => write!(f, "burn"),
			Self::List => write!(f, "list"),
			Self::UnList => write!(f, "un_list"),
			Self::Buy => write!(f, "buy"),
		}
	}
}

pub enum NFTCoreContractEndpoints {
	Mint,
	Transfer,
	Burn,
}

impl fmt::Display for NFTCoreContractEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Mint => write!(f, "mint"),
			Self::Transfer => write!(f, "transfer"),
			Self::Burn => write!(f, "burn"),
		}
	}
}

/// Price Oracle Keys
#[derive(Debug, Clone, Copy)]
pub enum PriceOracleKeys {
	PriceType,
	Price,
	PriceMid,
	CharsCount,
	PriceMore,
	SimpleOperations,
	Main,
}

impl fmt::Display for PriceOracleKeys {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::PriceType => write!(f, "price_type"),
			Self::Price => write!(f, "price"),
			Self::PriceMid => write!(f, "price_mid"),
			Self::CharsCount => write!(f, "chars_count"),
			Self::PriceMore => write!(f, "price_more"),
			Self::SimpleOperations => write!(f, "simple_operations"),
			Self::Main => write!(f, "main"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PriceOracleArgs {
	Extension,
	PriceType,
	Price,
	PriceMid,
	CharsCount,
	PriceMore,
}

impl fmt::Display for PriceOracleArgs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::PriceType => write!(f, "price_type"),
			Self::Price => write!(f, "price"),
			Self::PriceMid => write!(f, "price_mid"),
			Self::CharsCount => write!(f, "chars_count"),
			Self::PriceMore => write!(f, "price_more"),
			Self::Extension => write!(f, "extension"),
		}
	}
}

/// Common Keys
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CommonKeys {
	Maintainer,
	AdministrationContract,
	ContractHash,
	ContractVersion,
	Authorities,
	AllowedContracts,
}

impl fmt::Display for CommonKeys {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Maintainer => write!(f, "maintainer"),
			Self::AdministrationContract => write!(f, "administration_contract"),
			Self::ContractHash => write!(f, "contract_hash"),
			Self::ContractVersion => write!(f, "contract_version"),
			Self::Authorities => write!(f, "authorities"),
			Self::AllowedContracts => write!(f, "allowed_contracts"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CommonArgs {
	Authorities,
	Authority,
	AdministrationContract,
}

impl fmt::Display for CommonArgs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Authorities => write!(f, "authorities"),
			Self::Authority => write!(f, "authority"),
			Self::AdministrationContract => write!(f, "administration_contract"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum CommonEndpoints {
	SetAuthorities,
	AddAuthority,
	RemoveAuthority,
	GetAuthorities,
}

impl fmt::Display for CommonEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::SetAuthorities => write!(f, "set_authorities"),
			Self::AddAuthority => write!(f, "add_authority"),
			Self::RemoveAuthority => write!(f, "remove_authority"),
			Self::GetAuthorities => write!(f, "get_authorities"),
		}
	}
}

/// Database Contract Keys

#[derive(Debug, Clone, Copy)]
pub enum DatabaseKeys {
	DictionarySubdomain = "key_database_subdomain",
	SubdomainCount = "key_database_subdomain_count",
	DictionarySubdomainList = "key_database_subdomain_list",
	DictionaryDomain = "key_database_domain",
	DictionaryDomainMap = "key_database_domain_map",
	DictionaryDomainList = "key_database_domain_list",
	DomainListPagination = "key_database_domain_list_pagination",
	TotalDomainCount = "key_database_totals_domain_count",
	TotalSubdomainCount = "key_database_totals_subdomain_count",
	DictionaryDomainOwer = "key_database_dictionary_owner_domain_list",
}

impl fmt::Display for DatabaseKeys {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::DictionarySubdomain => write!(f, "key_database_subdomain"),
			Self::SubdomainCount => write!(f, "key_database_subdomain_count"),
			Self::DictionarySubdomainList => write!(f, "key_database_subdomain_list"),
			Self::DictionaryDomain => write!(f, "key_database_domain"),
			Self::DictionaryDomainMap => write!(f, "key_database_domain_map"),
			Self::DictionaryDomainList => write!(f, "key_database_domain_list"),
			Self::DomainListPagination =>
				write!(f, "key_database_domain_list_pagination"),
			Self::TotalDomainCount => write!(f, "key_database_totals_domain_count"),
			Self::TotalSubdomainCount =>
				write!(f, "key_database_totals_subdomain_count"),
			Self::DictionaryDomainOwer =>
				write!(f, "key_database_dictionary_owner_domain_list"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseArgs {
	DomainName,
	SubdomainName,
	Resolver,
	ExpirationDate,
	Owner,
	Page,
}

impl fmt::Display for DatabaseArgs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::DomainName => write!(f, "arg_database_domain_name"),
			Self::SubdomainName => write!(f, "arg_database_subdomain_name"),
			Self::Resolver => write!(f, "arg_database_resolver"),
			Self::ExpirationDate => write!(f, "arg_database_expiration_date"),
			Self::Owner => write!(f, "arg_database_owner"),
			Self::Page => write!(f, "arg_database_page"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseEndpoints {
	SaveDomainName,
	SaveSubdomainName,
	RemoveDomainName,
	RemoveSubdomainName,
	SetDomainOwnership,
	SetDomainExpiration,
	SetDomainResolver,
	SetSubdomainResolver,
	GetDomainList,
	GetSubdomainList,
	GetTotals,
	GetDomain,
	GetSubdomain,
	Init,
	GetDomainListForOwner,
}

impl fmt::Display for DatabaseEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::SaveDomainName => write!(f, "save_domain_name"),
			Self::SaveSubdomainName => write!(f, "save_subdomain_name"),
			Self::RemoveDomainName => write!(f, "remove_domain_name"),
			Self::RemoveSubdomainName => write!(f, "remove_subdomain_name"),
			Self::SetDomainOwnership => write!(f, "set_domain_ownership"),
			Self::SetDomainExpiration => write!(f, "set_domain_expiration"),
			Self::SetDomainResolver => write!(f, "set_domain_resolver"),
			Self::SetSubdomainResolver => write!(f, "set_subdomain_resolver"),
			Self::GetDomainList => write!(f, "get_domain_list"),
			Self::GetSubdomainList => write!(f, "get_subdomain_list"),
			Self::GetTotals => write!(f, "get_totals"),
			Self::GetDomain => write!(f, "get_domain"),
			Self::GetSubdomain => write!(f, "get_subdomain"),
			Self::Init => write!(f, "init"),
			Self::GetDomainListForOwner => write!(f, "get_domain_list_for_owner"),
		}
	}
}

/// Registry Contract Keys

#[derive(Debug, Clone, Copy)]
pub enum RegistryEndpoints {
	MapDomainNameToContractHash,
	GetContractHashForDomainName,
}

impl fmt::Display for RegistryEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::MapDomainNameToContractHash =>
				write!(f, "map_domain_name_to_contract_hash"),
			Self::GetContractHashForDomainName =>
				write!(f, "get_contract_hash_for_domain_name"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum RegistryArgs {
	DomainName,
	DatabaseContractHash,
	NftContractHash,
}

impl fmt::Display for RegistryArgs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::DomainName => write!(f, "arg_registry_domain_name"),
			Self::DatabaseContractHash =>
				write!(f, "arg_registry_database_contract_hash"),
			Self::NftContractHash => write!(f, "arg_registry_nft_contract_hash"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum RegistryKeys {
	DomainContractHash = "key_registry_domain_contract_hash",
}

impl fmt::Display for RegistryKeys {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::DomainContractHash =>
				write!(f, "key_registry_domain_contract_hash"),
		}
	}
}

/// Main Contract Keys

#[derive(Debug, Clone, Copy)]
pub enum MainEndpoints {
	RegisterDomain,
	ResolverDomain,
	SetResolverAddressForDomain,
	RegisterSubDomain,
	RemoveSubDomain,
	SetResolverAddressForSubDomain,
	GetSubdomainsForDomain,
	GetDomainList,
	Init,
	Extend,
}

impl fmt::Display for MainEndpoints {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::RegisterDomain => write!(f, "register_domain"),
			Self::ResolverDomain => write!(f, "resolve_domain"),
			Self::SetResolverAddressForDomain =>
				write!(f, "set_resolver_address_for_domain"),
			Self::RegisterSubDomain => write!(f, "register_sub_domain"),
			Self::RemoveSubDomain => write!(f, "remove_subdomain"),
			Self::SetResolverAddressForSubDomain =>
				write!(f, "set_resolver_address_for_subdomain"),
			Self::GetSubdomainsForDomain => write!(f, "get_sudomains_for_domain"),
			Self::GetDomainList => write!(f, "get_domain_list"),
			Self::Init => write!(f, "init"),
			Self::Extend => write!(f, "extend"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum MainArgs {
	Domain,
	Subdomain,
	Duration,
	ResolverAddress,
	DomainPage,
	PriceOracleContractHash,
	RegisterAmount,
	CustomerPurse,
}

impl fmt::Display for MainArgs {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Domain => write!(f, "arg_domain"),
			Self::Subdomain => write!(f, "arg_subdomain"),
			Self::Duration => write!(f, "arg_duration"),
			Self::ResolverAddress => write!(f, "arg_resolver_address"),
			Self::DomainPage => write!(f, "arg_domain_page"),
			Self::PriceOracleContractHash =>
				write!(f, "arg_price_oracle_contract_hash"),
			Self::RegisterAmount => write!(f, "arg_amount"),
			Self::CustomerPurse => write!(f, "arg_main_customer_purse"),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum MainKeys {
	Maintainer,
	DictionaryDomainMetadata,
	DictionarySubdomainMetadata,
	SubdomainMetadata,
	PriceOracleContractHash,
	DatabaseContractHash,
	MaintainerPurse,
	RegistryContractHash,
}

impl fmt::Display for MainKeys {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Maintainer => write!(f, "key_main_maintainer"),
			Self::DictionaryDomainMetadata => write!(f, "key_domain_metadata"),
			Self::DictionarySubdomainMetadata => write!(f, "key_subdomain_metadata"),
			Self::SubdomainMetadata => write!(f, "key_subdomain_metadata"),
			Self::PriceOracleContractHash =>
				write!(f, "key_main_price_oracle_contract_hash"),
			Self::DatabaseContractHash =>
				write!(f, "key_main_database_contract_hash_map"),
			Self::MaintainerPurse => write!(f, "key_main_maintainer_purse"),
			Self::RegistryContractHash =>
				write!(f, "key_main_registry_contract_hash"),
		}
	}
}

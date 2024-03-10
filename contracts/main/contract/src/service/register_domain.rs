use alloc::string::{ String, ToString };
use casper_contract::contract_api::runtime;
use casper_types::account::AccountHash;
use common_lib::{
	constants::common_keys::MainArgs,
	enums::domain_name_actual_state::DomainNameActualState,
	errors::MainContractErrors,
	models::DomainName,
	utils::{
		domain_name::get_end_time_actual_state,
		maintainer::{ is_caller_maintainer, is_maintainer },
	},
};

use crate::{
	names_validator::NamesValidator,
	types::TResult,
	utils::get_allowed_extensions,
};

pub fn register_domain() -> TResult<DomainName> {
	let domain: String = runtime::get_named_arg(&MainArgs::Domain.to_string());
	let duration: u8 = runtime::get_named_arg(&MainArgs::Duration.to_string());
	let resolver_address: AccountHash = runtime::get_named_arg(
		&MainArgs::ResolverAddress.to_string()
	);
	let amount: U512 = runtime::get_named_arg(
		&MainArgs::RegisterAmount.to_string()
	);
	let customer_purse: URef = runtime::get_named_arg(
		&MainArgs::CustomerPurse.to_string()
	);

	let extensions = get_allowed_extensions();
	if extensions.is_empty() {
		return Err(MainContractErrors::AllowedExtensionsNotConfigured);
	}

	// Validation
	let validator = NamesValidator::instance(
		extensions.unwrap(),
		is_caller_maintainer()
	);

	let model = match validator.validate_name(domain.to_string()) {
		Ok(res) => res,
		Err(e) => {
			return Err(e);
		}
	};

	// Duration check
	if duration > 3 {
		return Err(MainContractErrors::InvalidDuration);
	}

	// Checking for existence in db
	let db_contract_hash =
		NameContractHashDb::instance().get_contract_hash_for_domain_name(&domain);

	if let Some(hash) = db_contract_hash {
		let store_domain_optional: Option<DomainName> = call_contract(
			hash,
			ENDPOINT_DATABASE_GET_DOMAIN,
			runtime_args! {
                ARG_DATABASE_DOMAIN_NAME => domain.to_string()
            }
		);

		if let Some(store_domain) = store_domain_optional {
			let actual_state = get_end_time_actual_state(Some(store_domain.end_time));
			match actual_state {
				DomainNameActualState::Busy => {
					return Err(MainContractErrors::DomainNameIsBusy);
				}
				DomainNameActualState::GracePeriod => {
					return Err(MainContractErrors::DomainNameIsInGracePeriod);
				}
				DomainNameActualState::Available => {}
			}
		}
	}

	// Checking with price oracle
	let authorities_hash: Option<ContractHash> = get_stored_value_from_key(
		KEY_MAIN_AUTHORITIES_CONTRACT_HASH
	);
	if authorities_hash.is_none() {
		return response_error(
			MainContractErrors::AuthoritiesContractHashNotConfigured
		);
	}

	let chars_count = model.get_name_len();

	let price_oracle_contract_hash: ContractHash = match
		get_contract_hash_from_authority_contract(
			authorities_hash.unwrap(),
			ContractKind::PriceOracle,
			None
		)
	{
		Ok(res) =>
			match res {
				Some(res) => res,
				None => {
					return response_error(
						CommonError::NoContractHashWasFoundInAuthoritiesContract
					);
				}
			}
		Err(e) => {
			return response_error(e);
		}
	};

	let price: U512 = runtime::call_contract(
		price_oracle_contract_hash,
		ENDPOINT_PO_GET_PRICE,
		runtime_args! {
            ARG_PO_PRICE_TYPE_CHARS_COUNT => chars_count as u64
        }
	);

	// Checking price
	if U512::from(duration) * price != amount {
		runtime::revert(MainContractErrors::PriceDiscrepancy);
	}

	// Payment process
	let purse: URef = get_stored_value_from_key(
		KEY_MAIN_MAINTAINER_PURSE
	).unwrap_or_revert_with(MainContractErrors::MaintainerPurseNotConfigured);

	let balance = get_purse_balance(customer_purse).unwrap_or_revert();

	if balance < amount {
		return response_error(MainContractErrors::InsufficientCustomerBalance);
	}
	transfer_from_purse_to_purse(
		customer_purse,
		purse,
		amount,
		None
	).unwrap_or_revert();

	// Mint NFT
	let nft_contract_hash = match
		get_contract_hash_from_authority_contract(
			authorities_hash.unwrap(),
			ContractKind::NFT,
			None
		)
	{
		Ok(res) =>
			match res {
				Some(res) => res,
				None => {
					return response_error(
						CommonError::NoContractHashWasFoundInAuthoritiesContract
					);
				}
			}
		Err(e) => {
			return response_error(e);
		}
	};

	let token_id = base16::encode_lower(&runtime::blake2b(&domain));

	runtime::call_contract::<()>(
		nft_contract_hash,
		ENDPOINT_NFT_MINT,
		runtime_args! {
            ARG_NFT_TOKEN_OWNER => runtime::get_caller(),
            ARG_NFT_METADATA => token_id,
        }
	);

	// Save to database
	let end_time = calculate_domain_name_end_date(duration);
	let caller = runtime::get_caller();

	let db_contract_hash = match
		get_contract_hash_from_authority_contract(
			authorities_hash.unwrap(),
			ContractKind::Database,
			Some(model.extension)
		)
	{
		Ok(hash) =>
			match hash {
				Some(res) => res,
				None => {
					return response_error(
						MainContractErrors::DatabaseFulfilledOrNotConfigured
					);
				}
			}
		Err(e) => {
			return response_error(e);
		}
	};

	let saving_domain_name = DomainName {
		end_time,
		name: domain.clone(),
		token_id: token_id.to_string(),
		owner: caller,
		resolver: resolver_address,
	};

	runtime::call_contract::<()>(
		db_contract_hash,
		ENDPOINT_DATABASE_SAVE_DOMAIN_NAME,
		runtime_args! {
            ARG_DATABASE_DOMAIN_NAME => saving_domain_name.clone()
        }
	);
	// Save to name_contract_hash_map
	NameContractHashDb::instance().set_contract_hash_for_domain_name(
		&domain,
		db_contract_hash
	);
	Ok(saving_domain_name)
}

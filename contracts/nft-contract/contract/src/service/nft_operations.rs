use alloc::string::{ ToString, String };
use casper_contract::contract_api::runtime;
use casper_types::{ Key, ContractHash, runtime_args, U512, RuntimeArgs };
use common_lib::{
	constants::common_keys::{ NFTContractArgs, NFTCoreContractEndpoints },
	db::dictionary::Dictionary,
	errors::NFTErrors,
};
use crate::db::listing::Listing;
use crate::types::NResult;

pub fn mint() -> NResult<()> {
	let owner = runtime::get_named_arg::<Key>(
		&NFTContractArgs::Owner.to_string()
	);
	let metadata = runtime::get_named_arg::<String>(
		&NFTContractArgs::Metadata.to_string()
	);
	let nft_core_contract_hash = runtime::get_named_arg::<ContractHash>(
		&NFTContractArgs::NftCoreContractHash.to_string()
	);

	Ok(
		runtime::call_contract::<()>(
			nft_core_contract_hash,
			&NFTCoreContractEndpoints::Mint.to_string(),
			runtime_args! {
                    "token_owner" => owner,
                    "token_meta_data" => metadata,
                }
		)
	)
}

pub fn transfer() -> NResult<()> {
	let nft_core_contract_hash = runtime::get_named_arg::<ContractHash>(
		&NFTContractArgs::NftCoreContractHash.to_string()
	);
	let token_id = runtime::get_named_arg::<String>(
		&NFTContractArgs::TokenId.to_string()
	);
	let source_key = runtime::get_named_arg::<Key>(
		&NFTContractArgs::SourceKey.to_string()
	);
	let destination_key = runtime::get_named_arg::<Key>(
		&NFTContractArgs::DestinationKey.to_string()
	);

	Ok(
		runtime::call_contract::<()>(
			nft_core_contract_hash,
			&NFTCoreContractEndpoints::Transfer.to_string(),
			runtime_args! {
                "token_hash" => token_id,
                "source_key" => source_key,
                "target_key" => destination_key,
            }
		)
	)
}

pub fn burn() -> NResult<()> {
	let nft_core_contract_hash = runtime::get_named_arg::<ContractHash>(
		&NFTContractArgs::NftCoreContractHash.to_string()
	);
	let token_id = runtime::get_named_arg::<String>(
		&NFTContractArgs::TokenId.to_string()
	);
	Ok(
		runtime::call_contract::<()>(
			nft_core_contract_hash,
			&NFTCoreContractEndpoints::Burn.to_string(),
			runtime_args! {
                "token_id" => token_id,
            }
		)
	)
}

pub fn list() -> NResult<()> {
	let token_id = runtime::get_named_arg::<String>(
		&NFTContractArgs::TokenId.to_string()
	);
	let token_price = runtime::get_named_arg::<U512>(
		&NFTContractArgs::TokenPrice.to_string()
	);

	let mut instance = Dictionary::listing_instance();
	if !instance.is_listed(token_id.clone()) {
		instance.list(token_id, token_price);
	}

	Ok(())
}

pub fn un_list() -> NResult<()> {
	let token_id = runtime::get_named_arg::<String>(
		&NFTContractArgs::TokenId.to_string()
	);

	let mut instance = Dictionary::listing_instance();
	if instance.is_listed(token_id.clone()) {
		instance.un_list(token_id);
	}

	Ok(())
}

pub fn buy() -> NResult<()> {
	let token_id = runtime::get_named_arg::<String>(
		&NFTContractArgs::TokenId.to_string()
	);
	let source_key = runtime::get_named_arg::<Key>(
		&NFTContractArgs::SourceKey.to_string()
	);
	let instance = Dictionary::listing_instance();
	let caller_key: Key = runtime::get_caller().into();
	if !instance.is_listed(token_id.clone()) {
		return Err(NFTErrors::NFTIsNotListed);
	}

	let nft_core_contract_hash = runtime::get_named_arg::<ContractHash>(
		&NFTContractArgs::NftCoreContractHash.to_string()
	);
	// TODO: payment (2.5% - commission, 97.5% - to the owner)

	runtime::call_contract::<()>(
		nft_core_contract_hash,
		&NFTCoreContractEndpoints::Transfer.to_string(),
		runtime_args! {
            "token_hash" => token_id,
            "source_key" => source_key,
            "target_key" => caller_key,
        }
	);

	Ok(())
}

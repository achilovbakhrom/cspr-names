use casper_types::{ runtime_args, RuntimeArgs, Key, account::AccountHash };
use common_lib::{
	constants::common_keys::{
		AdministrationEndpoints,
		AdministrationArgs,
		AdministractionStoreKeys,
		CommonKeys,
		CommonEndpoints,
	},
	enums::contracts_enum::ContractKind,
	utils::helpers::to_domain_list_limit_key,
	models::registry_pointer::CompoundContract,
};

use crate::utils::{
	UnitTestContext,
	deploy,
	query_uref,
	query_dictionary,
	query,
};

const CONTRACT_NAME: &str = "administration-contract.wasm";

/// Allowed extensions
impl UnitTestContext {
	pub fn set_allowed_extensions(
		&mut self,
		signer: &AccountHash,
		args: RuntimeArgs,
		success: bool
	) {
		deploy(
			&mut self.builder,
			signer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::SetAllowedExtensions.to_string(),
			}),
			args,
			success,
			None
		)
	}

	pub fn set_allowed_extensions_with_maintainer(&mut self, args: RuntimeArgs) {
		let maintainer = self.maintainer;
		self.set_allowed_extensions(&maintainer, args.clone(), true)
	}

	pub fn get_allowed_extensions(&self) -> Vec<String> {
		query::<Vec<String>>(
			&self.builder,
			self.contract_hash.into(),
			&[AdministractionStoreKeys::AllowedExtensions.to_string()]
		)
	}
}

/// Authorities
impl UnitTestContext {
	pub fn add_user_to_authority_with_maintainer(
		&mut self,
		account: AccountHash
	) {
		let args = runtime_args! {
			"authority" => Key::Account(account),
		};
		let maintainer = self.maintainer;
		deploy(
			&mut self.builder,
			&self.maintainer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: CommonEndpoints::AddAuthority.to_string(),
			}),
			args,
			true,
			None
		)
	}

	pub fn add_user_to_authority(
		&mut self,
		signer: AccountHash,
		account: AccountHash,
		success: bool
	) {
		let args = runtime_args! {};
		deploy(
			&mut self.builder,
			&signer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::AddContractAuthority.to_string(),
			}),
			args,
			success,
			None
		)
	}

	pub fn get_authorities(&self) -> Vec<Key> {
		query::<Vec<Key>>(
			&self.builder,
			self.contract_hash.into(),
			&[CommonKeys::Authorities.to_string()]
		)
	}
}

/// Limits
impl UnitTestContext {
	pub fn get_min_chars_count(&self, extension: String) -> u8 {
		let key = format!(
			"{}:{}",
			extension,
			AdministractionStoreKeys::CharsCount.to_string()
		);
		query(&self.builder, self.contract_hash.into(), &[key])
	}

	pub fn set_min_chars_count(&mut self, extension: String, count: u8) {
		let args =
			runtime_args! {
			"chars_count" => count,
			"extension" => Some(extension)
		};

		deploy(
			&mut self.builder,
			&self.maintainer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::SetCharsMinCount.to_string(),
			}),
			args,
			true,
			None
		)
	}

	pub fn get_listing_limit(&self, kind: ContractKind) -> u32 {
		let key = to_domain_list_limit_key(&kind);
		query(&self.builder, self.contract_hash.into(), &[key])
	}

	pub fn set_listing_limit(&mut self, kind: ContractKind, count: u32) {
		deploy(
			&mut self.builder,
			&self.maintainer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::SetListingLimit.to_string(),
			}),
			runtime_args! {
				"chars_count" => count,
				"contract_kind" => kind,
			},
			true,
			None
		)
	}
}

/// Contract List
impl UnitTestContext {
	pub fn add_contract(
		&mut self,
		kind: ContractKind,
		contract_hash: Key,
		extension: Option<String>
	) {
		let args = if let Some(ext) = extension {
			runtime_args! {
				"contract_kind" => kind,
				"key" => contract_hash,
				"extension" => Some(ext)
			}
		} else {
			runtime_args! {
				"contract_kind" => kind,
				"key" => contract_hash,
			}
		};

		deploy(
			&mut self.builder,
			&self.maintainer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::AddContract.to_string(),
			}),
			args,
			true,
			None
		)
	}
	pub fn get_contract(
		&self,
		kind: ContractKind,
		is_compound: bool,
		ext: Option<String>
	) -> (Key, Option<u32>) {
		if is_compound {
			let extension = ext.unwrap();
			let store_key = format!("{}:{}", kind, extension);

			let keys: Vec<CompoundContract> = query(
				&self.builder,
				self.contract_hash.into(),
				&[store_key]
			);
			let limit = 100u32;

			if !keys.is_empty() {
				let filtered = keys
					.iter()
					.filter(|item| (**item).count.unwrap_or(0) < limit)
					.map(|item| *item)
					.collect::<Vec<CompoundContract>>();
				if filtered.is_empty() {
					panic!("Contract is not found");
				}
				let first = filtered.first().unwrap();
				(first.key, first.count)
			} else {
				panic!("Contract is not found");
			}
		} else {
			let contract_key: Key = query(
				&self.builder,
				self.contract_hash.into(),
				&[kind.to_string()]
			);

			(contract_key, None)
		}
	}
	pub fn increment_contract(
		&mut self,
		kind: ContractKind,
		key: Key,
		extension: Option<String>
	) {
		deploy(
			&mut self.builder,
			&self.maintainer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::IncrementContract.to_string(),
			}),
			runtime_args! {
				"contract_kind" => kind,
				"key" => key,
				"extension" => extension,
			},
			true,
			None
		)
	}
	pub fn decrement_contract(
		&mut self,
		kind: ContractKind,
		key: Key,
		extension: Option<String>
	) {
		deploy(
			&mut self.builder,
			&self.maintainer,
			&(crate::utils::DeploySource::ByContractHash {
				hash: self.contract_hash,
				entry_point: AdministrationEndpoints::DecrementContract.to_string(),
			}),
			runtime_args! {
				"contract_kind" => kind,
				"key" => key,
				"extension" => extension,
			},
			true,
			None
		)
	}
}

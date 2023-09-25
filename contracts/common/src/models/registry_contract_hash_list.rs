use alloc::{ string::String, vec::Vec };
use casper_types::{ bytesrepr::{ allocate_buffer, FromBytes, ToBytes }, CLTyped, ContractHash };

use crate::enums::contracts_enum::ContractKind;

#[derive(Clone)]
pub struct RegistryContractHashList {
	pub contract_type: ContractKind,
	pub contract_hash_list: Vec<ContractHash>,
	pub attr_key: Option<String>,
}

impl ToBytes for RegistryContractHashList {
	fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
		let mut result = allocate_buffer(self)?;
		result.extend(self.contract_type.to_bytes()?);
		result.extend(self.contract_hash_list.to_bytes()?);
		result.extend(self.attr_key.to_bytes()?);

		Ok(result)
	}

	fn serialized_length(&self) -> usize {
		self.contract_type.serialized_length() +
			self.contract_hash_list.serialized_length() +
			self.attr_key.serialized_length()
	}
}

impl FromBytes for RegistryContractHashList {
	fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
		let (contract_type, remainder) = ContractKind::from_bytes(bytes)?;
		let (contract_hash_list, remainder) = Vec::<ContractHash>::from_bytes(remainder)?;
		let (attr_key, remainder) = Option::<String>::from_bytes(remainder)?;

		let result = Self {
			contract_type,
			contract_hash_list,
			attr_key,
		};
		Ok((result, remainder))
	}
}

impl CLTyped for RegistryContractHashList {
	fn cl_type() -> casper_types::CLType {
		casper_types::CLType::Any
	}
}

use casper_types::{ bytesrepr::{ allocate_buffer, FromBytes, ToBytes }, CLTyped, ContractHash };

#[derive(Clone)]
pub struct RegistryContractHashPair {
	pub db_contract_hash: ContractHash,
	pub nft_contract_hash: ContractHash,
}

impl ToBytes for RegistryContractHashPair {
	fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
		let mut result = allocate_buffer(self)?;
		result.extend(self.db_contract_hash.to_bytes()?);
		result.extend(self.nft_contract_hash.to_bytes()?);

		Ok(result)
	}

	fn serialized_length(&self) -> usize {
		self.db_contract_hash.serialized_length() + self.nft_contract_hash.serialized_length()
	}
}

impl FromBytes for RegistryContractHashPair {
	fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
		let (db_contract_hash, remainder) = ContractHash::from_bytes(bytes)?;
		let (nft_contract_hash, remainder) = ContractHash::from_bytes(remainder)?;

		let result = Self {
			db_contract_hash,
			nft_contract_hash,
		};
		Ok((result, remainder))
	}
}

impl CLTyped for RegistryContractHashPair {
	fn cl_type() -> casper_types::CLType {
		casper_types::CLType::Any
	}
}

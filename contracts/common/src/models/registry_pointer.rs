use casper_types::{ bytesrepr::{ allocate_buffer, FromBytes, ToBytes }, CLTyped, ContractHash };

#[derive(Clone)]
pub struct RegistryPointer {
	pub contract_hash: ContractHash,
	pub count: Option<i32>,
}

impl ToBytes for RegistryPointer {
	fn to_bytes(&self) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
		let mut result = allocate_buffer(self)?;
		result.extend(self.contract_hash.to_bytes()?);
		result.extend(self.count.to_bytes()?);

		Ok(result)
	}

	fn serialized_length(&self) -> usize {
		self.contract_hash.serialized_length() + self.count.serialized_length()
	}
}

impl FromBytes for RegistryPointer {
	fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
		let (contract_hash, remainder) = ContractHash::from_bytes(bytes)?;
		let (count, remainder) = Option::<i32>::from_bytes(remainder)?;

		let result = Self {
			contract_hash,
			count,
		};
		Ok((result, remainder))
	}
}

impl CLTyped for RegistryPointer {
	fn cl_type() -> casper_types::CLType {
		casper_types::CLType::Any
	}
}

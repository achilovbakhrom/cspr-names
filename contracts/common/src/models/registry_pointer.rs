use casper_types::{
	bytesrepr::{ allocate_buffer, FromBytes, ToBytes },
	CLTyped,
	Key,
};

#[derive(Clone)]
pub struct CompoundContract {
	pub key: Key,
	pub count: Option<i32>,
}

impl ToBytes for CompoundContract {
	fn to_bytes(
		&self
	) -> Result<alloc::vec::Vec<u8>, casper_types::bytesrepr::Error> {
		let mut result = allocate_buffer(self)?;
		result.extend(self.key.to_bytes()?);
		result.extend(self.count.to_bytes()?);

		Ok(result)
	}

	fn serialized_length(&self) -> usize {
		self.key.serialized_length() + self.count.serialized_length()
	}
}

impl FromBytes for CompoundContract {
	fn from_bytes(
		bytes: &[u8]
	) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
		let (contract_hash, remainder) = Key::from_bytes(bytes)?;
		let (count, remainder) = Option::<i32>::from_bytes(remainder)?;

		let result = Self {
			key: contract_hash,
			count,
		};
		Ok((result, remainder))
	}
}

impl CLTyped for CompoundContract {
	fn cl_type() -> casper_types::CLType {
		casper_types::CLType::Any
	}
}

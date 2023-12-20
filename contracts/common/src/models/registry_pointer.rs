use alloc::{ fmt::Display, string::{ ToString, String } };
use casper_types::{
	bytesrepr::{ allocate_buffer, FromBytes, ToBytes },
	CLTyped,
	Key,
};

#[derive(Clone, Copy)]
pub struct CompoundContract {
	pub key: Key,
	pub count: Option<u32>,
}

impl Display for CompoundContract {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let c = self.count.unwrap_or(0).to_string();
		write!(f, "{}:{}", self.key.to_string(), c)
	}
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
		let (count, remainder) = Option::<u32>::from_bytes(remainder)?;

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

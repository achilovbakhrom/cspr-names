use alloc::{ vec::Vec, string::String };
use casper_types::{ bytesrepr::{ allocate_buffer, Error, FromBytes, ToBytes }, CLType, CLTyped };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
	pub min_allowed_chars_count: u8,
	pub max_enitity_count_per_contract: u16,
	pub allowed_extensions: Vec<String>,
	pub domains_per_page: u8,
	pub max_subscription_years: u8,
}

impl ToBytes for Configuration {
	fn to_bytes(&self) -> Result<Vec<u8>, Error> {
		let mut result = allocate_buffer(self)?;
		result.extend(self.min_allowed_chars_count.to_bytes()?);
		result.extend(self.max_enitity_count_per_contract.to_bytes()?);
		result.extend(self.allowed_extensions.to_bytes()?);
		result.extend(self.domains_per_page.to_bytes()?);
		result.extend(self.max_subscription_years.to_bytes()?);

		Ok(result)
	}

	fn serialized_length(&self) -> usize {
		self.min_allowed_chars_count.serialized_length() +
			self.max_enitity_count_per_contract.serialized_length() +
			self.allowed_extensions.serialized_length() +
			self.domains_per_page.serialized_length() +
			self.max_subscription_years.serialized_length()
	}
}

impl FromBytes for Configuration {
	fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
		let (min_allowed_chars_count, remainder) = u8::from_bytes(bytes)?;
		let (max_enitity_count_per_contract, remainder) = u16::from_bytes(remainder)?;
		let (allowed_extensions, remainder) = Vec::<String>::from_bytes(remainder)?;
		let (max_subscription_years, remainder) = u8::from_bytes(remainder)?;
		let (domains_per_page, remainder) = u8::from_bytes(remainder)?;

		let result = Configuration {
			min_allowed_chars_count,
			max_enitity_count_per_contract,
			allowed_extensions,
			domains_per_page,
			max_subscription_years,
		};
		Ok((result, remainder))
	}
}

impl CLTyped for Configuration {
	fn cl_type() -> CLType {
		CLType::Any
	}
}

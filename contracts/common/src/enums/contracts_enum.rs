use core::fmt::Display;

use alloc::{ vec, vec::Vec };
use casper_types::{ bytesrepr::FromBytes, bytesrepr::ToBytes, CLTyped };

/**
 * Single Contract - ContractKind <-> Key
 * Compound Contract - ContractKind + Extension <-> Key (Also need to consider the number of storing items)
 */

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ContractKind {
	Main = 0, // single
	Database = 1, // compound
	NFT = 2, // single
	NFTCore = 3, // compound
	PriceOracle = 4, // single
	Registry = 5, // single
	Administration = 6, // single
}

impl Display for ContractKind {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self)
	}
}

impl FromBytes for ContractKind {
	fn from_bytes(
		bytes: &[u8]
	) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
		let value = u8
			::from_bytes(bytes)
			.expect("Error while unwrapping &[u8] to u8");
		match value.0 {
			0 => Ok((ContractKind::Database, value.1)),
			1 => Ok((ContractKind::Main, value.1)),
			2 => Ok((ContractKind::NFT, value.1)),
			3 => Ok((ContractKind::NFTCore, value.1)),
			4 => Ok((ContractKind::PriceOracle, value.1)),
			5 => Ok((ContractKind::Authorities, value.1)),
			_ => Err(casper_types::bytesrepr::Error::OutOfMemory),
		}
	}
}

impl CLTyped for ContractKind {
	fn cl_type() -> casper_types::CLType {
		u8::cl_type()
	}
}

impl ToBytes for ContractKind {
	fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
		Ok(vec![*self as u8])
	}
	fn serialized_length(&self) -> usize {
		vec![*self as u8].len()
	}
}

impl ContractKind {
	fn all_contracts() -> Vec<Self> {
		vec![
			Self::Database,
			Self::Main,
			Self::NFT,
			Self::NFTCore,
			Self::PriceOracle,
			Self::Authorities
		]
	}
}

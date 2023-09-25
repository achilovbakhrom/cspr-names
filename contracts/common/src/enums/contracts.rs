use alloc::{vec, vec::Vec};
use casper_types::{bytesrepr::FromBytes, bytesrepr::ToBytes, CLTyped};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Contracts {
    Configuration = 0,
    Registry = 1,
    Database = 2,
    NFTCore = 3,
    NFTContract = 4,
    PriceOracle = 5,
}

impl FromBytes for Contracts {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let value = u8::from_bytes(bytes).expect("Error while unwrapping &[u8] to u8");
        match value.0 {
            0 => Ok((Contracts::Configuration, value.1)),
            1 => Ok((Contracts::Registry, value.1)),
            2 => Ok((Contracts::Database, value.1)),
            3 => Ok((Contracts::NFTCore, value.1)),
            4 => Ok((Contracts::NFTContract, value.1)),
            5 => Ok((Contracts::PriceOracle, value.1)),
        }
    }
}

impl CLTyped for Contracts {
    fn cl_type() -> casper_types::CLType {
        u8::cl_type()
    }
}

impl ToBytes for Contracts {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        Ok(vec![*self as u8])
    }
    fn serialized_length(&self) -> usize {
        vec![*self as u8].len()
    }
}

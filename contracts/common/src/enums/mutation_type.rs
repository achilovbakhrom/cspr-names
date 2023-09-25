use alloc::{vec, vec::Vec};
use casper_types::{bytesrepr::FromBytes, bytesrepr::ToBytes, CLTyped};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum MutationType {
    Append = 0,
    Replace = 1,
}

impl FromBytes for MutationType {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let value = u8::from_bytes(bytes).expect("Error while unwrapping &[u8] to u8");
        match value.0 {
            0 => Ok((MutationType::Append, value.1)),
            1 => Ok((MutationType::Replace, value.1)),
            _ => Err(casper_types::bytesrepr::Error::OutOfMemory),
        }
    }
}

impl CLTyped for MutationType {
    fn cl_type() -> casper_types::CLType {
        u8::cl_type()
    }
}

impl ToBytes for MutationType {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        Ok(vec![*self as u8])
    }
    fn serialized_length(&self) -> usize {
        vec![*self as u8].len()
    }
}

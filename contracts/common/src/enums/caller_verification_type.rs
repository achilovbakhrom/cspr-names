use alloc::{vec, vec::Vec};
use casper_types::{bytesrepr::FromBytes, CLTyped};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum CallerVerificationType {
    OnlyAccountHash = 0,
    OnlyContractHash = 1,
    All = 2,
}

impl FromBytes for CallerVerificationType {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), casper_types::bytesrepr::Error> {
        let value = u8::from_bytes(bytes).expect("Error while unwrapping &[u8] to u8");
        match value.0 {
            0 => Ok((CallerVerificationType::OnlyAccountHash, value.1)),
            1 => Ok((CallerVerificationType::OnlyContractHash, value.1)),
            2 => Ok((CallerVerificationType::All, value.1)),
            _ => Err(casper_types::bytesrepr::Error::OutOfMemory),
        }
    }
}

impl CLTyped for CallerVerificationType {
    fn cl_type() -> casper_types::CLType {
        u8::cl_type()
    }
}

impl ToBytes for CallerVerificationType {
    fn to_bytes(&self) -> Result<Vec<u8>, casper_types::bytesrepr::Error> {
        Ok(vec![*self as u8])
    }
    fn serialized_length(&self) -> usize {
        vec![*self as u8].len()
    }
}

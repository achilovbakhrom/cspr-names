use alloc::vec::Vec;
use casper_types::{
    bytesrepr::{allocate_buffer, Error, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash,
};
use core::u16;

#[derive(Clone)]
pub struct ContractHashItem {
    pub contract_hash: ContractHash,
    pub count: u16,
}

impl ToBytes for ContractHashItem {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        result.extend(self.contract_hash.to_bytes()?);
        result.extend(self.count.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.contract_hash.serialized_length() + self.count.serialized_length()
    }
}

impl FromBytes for ContractHashItem {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (contract_hash, remainder) = ContractHash::from_bytes(bytes)?;
        let (count, remainder) = u16::from_bytes(remainder)?;

        let result = ContractHashItem {
            contract_hash,
            count,
        };
        Ok((result, remainder))
    }
}

impl CLTyped for ContractHashItem {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

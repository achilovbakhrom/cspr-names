
use alloc::{ vec::Vec };
use core::u16;
use casper_types::{
    bytesrepr::{
        ToBytes,
        FromBytes,
        allocate_buffer,
        Error
    },
    CLTyped,
    CLType,
    ContractHash
};


#[derive(Clone)]
pub struct ContractHashDatabaseMap {
    pub contract_hash: ContractHash,
    pub count: u16,
}

impl ToBytes for ContractHashDatabaseMap {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        result.extend(self.contract_hash.to_bytes()?);
        result.extend(self.count.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.contract_hash.serialized_length() +
            self.count.serialized_length()
    }
}

impl FromBytes for ContractHashDatabaseMap {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (contract_hash, remainder) = ContractHash::from_bytes(bytes)?;
        let (count, remainder) = u16::from_bytes(remainder)?;


        let result = ContractHashDatabaseMap {
            contract_hash,
            count,
        };
        Ok((result, remainder))
    }
}

impl CLTyped for ContractHashDatabaseMap {
    fn cl_type() -> CLType {
        CLType::Any
    }
}
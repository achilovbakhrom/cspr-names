use casper_types::{
    account::AccountHash,
    bytesrepr::{ ToBytes, FromBytes, allocate_buffer, Error },
    CLTyped, CLType
};
use alloc::{ string::String, vec::Vec };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SubdomainName {
    pub name: String,
    pub resolver: AccountHash,
}

impl ToBytes for SubdomainName {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        
        result.extend(self.name.to_bytes()?);
        result.extend(self.resolver.to_bytes()?);
        
        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.name.serialized_length() + 
        self.resolver.serialized_length()
    }
}

impl FromBytes for SubdomainName {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {        
        let (name, remainder) = String::from_bytes(bytes)?;
        let (resolver, remainder) = AccountHash::from_bytes(remainder)?;

        let result = SubdomainName { 
            name,
            resolver
        };
        Ok((result, remainder))
    }
}

impl CLTyped for SubdomainName {
    fn cl_type() -> CLType {
        CLType::Any
    }
}
use alloc::{string::String, vec::Vec};
use casper_types::{
    account::AccountHash,
    bytesrepr::{allocate_buffer, Error, FromBytes, ToBytes},
    CLType, CLTyped,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DomainName {
    pub end_time: u64,
    pub name: String,
    pub token_id: String,
    pub owner: AccountHash,
    pub resolver: AccountHash,
}

impl ToBytes for DomainName {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        result.extend(self.end_time.to_bytes()?);
        result.extend(self.name.to_bytes()?);
        result.extend(self.token_id.to_bytes()?);
        result.extend(self.owner.to_bytes()?);
        result.extend(self.resolver.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.end_time.serialized_length()
            + self.name.serialized_length()
            + self.token_id.serialized_length()
            + self.owner.serialized_length()
            + self.resolver.serialized_length()
    }
}

impl FromBytes for DomainName {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (end_time, remainder) = u64::from_bytes(bytes)?;
        let (name, remainder) = String::from_bytes(remainder)?;
        let (token_id, remainder) = String::from_bytes(remainder)?;
        let (owner, remainder) = AccountHash::from_bytes(remainder)?;
        let (resolver, remainder) = AccountHash::from_bytes(remainder)?;

        let result = DomainName {
            end_time,
            name,
            token_id,
            owner,
            resolver,
        };
        Ok((result, remainder))
    }
}

impl CLTyped for DomainName {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

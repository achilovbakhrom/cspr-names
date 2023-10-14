use alloc::vec::Vec;
use casper_types::{
    bytesrepr::{allocate_buffer, Error, FromBytes, ToBytes},
    CLType, CLTyped,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct LocalMetadata {
    pub total_count: u64,
    pub page: u32,
}

impl ToBytes for LocalMetadata {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        result.extend(self.total_count.to_bytes()?);
        result.extend(self.page.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.total_count.serialized_length() + self.page.serialized_length()
    }
}

impl FromBytes for LocalMetadata {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (total_count, remainder) = u64::from_bytes(bytes)?;
        let (page, remainder) = u32::from_bytes(remainder)?;

        let result = LocalMetadata { total_count, page };
        Ok((result, remainder))
    }
}

impl CLTyped for LocalMetadata {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

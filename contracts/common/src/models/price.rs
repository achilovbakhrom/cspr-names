use crate::enums::price_oracle_contract::PriceType;
use alloc::vec;
use alloc::vec::Vec;
use casper_types::bytesrepr::{allocate_buffer, Error, FromBytes, ToBytes};
use casper_types::{CLType, CLTyped, U512};

#[derive(Clone)]
pub struct PriceItem {
    pub char_count: u8,
    pub price: U512,
}

impl ToBytes for PriceItem {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        result.extend(self.char_count.to_bytes()?);
        result.extend(self.price.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.char_count.serialized_length() + self.price.serialized_length()
    }
}

impl FromBytes for PriceItem {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (char_count, remainder) = u8::from_bytes(bytes)?;
        let (price, remainder) = U512::from_bytes(remainder)?;

        let result = Self { char_count, price };
        Ok((result, remainder))
    }
}

#[derive(Clone)]
pub struct Price {
    pub price_type: PriceType,
    pub price: U512,
    pub price_by_count: Vec<PriceItem>,
    pub price_more: U512,
}

impl ToBytes for Price {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = allocate_buffer(self)?;
        result.extend(self.price_type.to_bytes()?);
        result.extend(self.price.to_bytes()?);
        result.extend(self.price_by_count.to_bytes()?);
        result.extend(self.price_more.to_bytes()?);

        Ok(result)
    }

    fn serialized_length(&self) -> usize {
        self.price_type.serialized_length()
            + self.price.serialized_length()
            + self.price_by_count.serialized_length()
            + self.price_more.serialized_length()
    }
}

impl FromBytes for Price {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), Error> {
        let (price_type, remainder) = PriceType::from_bytes(bytes)?;
        let (price, remainder) = U512::from_bytes(remainder)?;
        let (price_by_count, remainder) = Vec::<PriceItem>::from_bytes(remainder)?;
        let (price_more, remainder) = U512::from_bytes(remainder)?;

        let result = Self {
            price_type,
            price,
            price_by_count,
            price_more,
        };

        Ok((result, remainder))
    }
}

impl Price {
    pub fn default() -> Self {
        Self {
            price_type: PriceType::Fixed,
            price: U512::from(500_000_000_000u64),
            price_by_count: vec![],
            price_more: U512::from(0u64),
        }
    }
}

impl CLTyped for Price {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

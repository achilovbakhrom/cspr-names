use casper_types::{
    bytesrepr::{ FromBytes, ToBytes },
    CLTyped
};

pub trait Storable {
    fn get<T: CLTyped + FromBytes>(&self, key: &str) -> Option<T>;
    fn set<T: CLTyped + ToBytes>(&self, key: &str, value: T);
    fn remove<T: CLTyped + ToBytes>(&self, key: &str);
}
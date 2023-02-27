use casper_types::{ApiError};

#[repr(u16)]
pub enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
    InvalidDomainName = 2,
    InvalidDurationForRegistration = 3,
    InvalidResolverAddress = 4
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {        
        ApiError::User(error as u16)
    }
}
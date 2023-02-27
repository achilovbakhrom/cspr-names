use casper_types::ApiError;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum CommonError {
    UnexpectedKeyVariant = 1,
    MissingStorageUref = 2,
    InvalidStorageUref = 3,
    NoAuthority = 4
}

impl From<CommonError> for ApiError {
    fn from(e: CommonError) -> Self {
        ApiError::User(e as u16)
    }
}


#[repr(u16)]
#[derive(Clone, Copy)]
pub enum MainContractErrors {
    InvalidDomain = 21,
    DomainAlreadyExists = 22,
    InvalidDuration = 23,
    PriceDiscrepancy = 24,
    NoDictionaryDomainMetadata = 25,
    NoDictionaryDomainList = 26,
    DomainNotExists = 27,
    InvalidSubdomain = 28,
    SubdomainParseError = 29,
    InvalidOwner = 30,
    SubdomainMaxCountExceeded = 31,
    SubdomainNotExists = 32,
    PriceOracleContractHashNotFound = 33,
    DomainNameIsBusy = 34,
    DomainNameIsInGracePeriod = 35,
    UserHasNoAccessToRegister = 36,
    OnlyMaintainerHasAccess = 37,
    CannotAddMaintainer = 38,
    AuthorityHasAlreadyTaken = 39,
    CannotRemoveMaintainer = 40,
    UserHasNoAccess = 41,
    
}

impl From<MainContractErrors> for ApiError {
    fn from(e: MainContractErrors) -> Self {
        ApiError::User(e as u16)
    }
}

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum PriceOracleContractErrors {
    PriceTypeMismatch = 40,
    PriceMidLengthAndMidCharsCountMismatch = 41,
    PriceForCharsCountNotFound = 42,
    PriceTypeIsNotFound = 43,
    PriceIsNotSet = 44,
    PriceMoreIsNotSet = 45,
    PriceMidIsNotSet = 46,
    PriceCannotAddMaintainer = 47,
    PriceCannotRemoveMaintainer = 48,
    PriceAuthorityHasAlreadyTaken = 49,
    PriceOnlyMaintainerHasAccess = 50,
    PriceUserHasNoAccess = 51


}

impl From<PriceOracleContractErrors> for ApiError {
    fn from(e: PriceOracleContractErrors) -> Self {
        ApiError::User(e as u16)
    }
}
use casper_types::ApiError;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum CommonError {
    UnexpectedKeyVariant = 1,
    MissingStorageUref = 2,
    InvalidStorageUref = 3,
    NoAuthority = 4,
    ItemNotFound = 5,
}

impl From<CommonError> for ApiError {
    fn from(e: CommonError) -> Self {
        ApiError::User(e as u16)
    }
}


#[repr(u16)]
#[derive(Clone, Copy)]
pub enum MainContractErrors {
    InvalidName = 21,
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
    InvalidCreator = 42,
    InvalidExtension = 43,
    ExtensionListIsNotSet = 44,
    MaintainerIsNotSet = 45
    
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
    PriceUserHasNoAccess = 51,
    PriceSimpleOperationsIsNotSet = 52,
    PricePriceIsNotSetForExtension = 53

}

impl From<PriceOracleContractErrors> for ApiError {
    fn from(e: PriceOracleContractErrors) -> Self {
        ApiError::User(e as u16)
    }
}

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum DatabaseErrors {
    DatabaseSubdomainMaxCountExceeded = 60,
    DatabaseSubdomainAlreadyExists = 61,
    DatabaseSubdomainDoesntExist = 62,
    DatabaseDomainDoesntExist = 63,
    DatabaseUnexpected = 64,

}

impl From<DatabaseErrors> for ApiError {
    fn from(e: DatabaseErrors) -> Self {
        ApiError::User(e as u16)
    }
}

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum NFTErrors {
    NFTCoreHashIsNotSet = 80,
    NFTIsNotListed = 81,
    NFTContractHashIsNotSet = 82,
}

impl From<NFTErrors> for ApiError {
    fn from(e: NFTErrors) -> Self {
        ApiError::User(e as u16)
    }
}
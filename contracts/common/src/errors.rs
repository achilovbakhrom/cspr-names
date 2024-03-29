use casper_types::ApiError;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum CommonError {
	UnexpectedKeyVariant = 1,
	MissingStorageUref = 2,
	InvalidStorageUref = 3,
	NoAuthority = 4,
	ItemNotFound = 5,
	NoContractHashWasFoundInAuthoritiesContract = 6,
	InvalidMaintainer = 7,
	InvalidCaller = 8,
	InvalidKey = 9,
	UnknowError = 10,
	MissingMaintainer = 11,
	MissingRegistryHash = 12,
	MissingContractHash = 13,
	FailedToConvertToAccountHash = 14,
	NoAdministrationContractHashStored = 15,
	MissingAdministrationContractHash = 16,
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
	AllowedExtensionsNotConfigured = 44,
	MaintainerNotConfigured = 45,
	DatabaseFulfilledOrNotConfigured = 46,
	MaintainerPurseNotConfigured = 47,
	InsufficientCustomerBalance = 48,
	AuthoritiesContractHashNotConfigured = 49,
	RegistryContractHashNotConfigured = 50,
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
	PricePriceIsNotSetForExtension = 53,
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
pub enum AuthorityErrors {
	AuthorityInvalidCaller = 100,
	AuthorityMaintainerIsNotSet = 101,
}

impl From<AuthorityErrors> for ApiError {
	fn from(e: AuthorityErrors) -> Self {
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

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum RegistryErrors {
	OperatorAlreadyExists = 200,
	OperatorDoesntExist = 201,
	MaintainerIsNotSet = 202,
	RegistryObjectNotFound = 203,
	InvalidContractHash = 204,
	ContractHashCountExceeded = 205,
	ContractHashNotFouond = 206,
	InvalidCaller = 207,
}

impl From<RegistryErrors> for ApiError {
	fn from(e: RegistryErrors) -> Self {
		ApiError::User(e as u16)
	}
}

// Administration errors
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum AdministrationErrors {
	ExtensionNotExist = 1000,
	ExtensionAlreadyExist = 1001,
	ContractNotFound = 1002,
	ProvideExtensionArgument = 1003,
	ContractIsFilled = 1004,
	InvalidCaller = 1005,
}

impl From<AdministrationErrors> for ApiError {
	fn from(e: AdministrationErrors) -> Self {
		ApiError::User(e as u16)
	}
}

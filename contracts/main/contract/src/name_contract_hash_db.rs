use casper_types::ContractHash;
use common_lib::constants::common_keys::KEY_MAIN_NAME_CONTRACT_HASH;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;

pub struct NameContractHashDb {
    store: Dictionary,
}

impl NameContractHashDb {
    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_MAIN_NAME_CONTRACT_HASH),
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_MAIN_NAME_CONTRACT_HASH)
    }

    pub fn set_contract_hash_for_domain_name(
        &self,
        domain_name: &str,
        contract_hash: ContractHash,
    ) {
        self.store.set(domain_name, contract_hash)
    }

    pub fn get_contract_hash_for_domain_name(&self, domain_name: &str) -> Option<ContractHash> {
        self.store.get(domain_name)
    }
}

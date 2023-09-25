use casper_types::ContractHash;
use common_lib::constants::{KEY_NFT_CONTRACT_HASH_NV, KEY_NFT_CORE_CONTRACT_HASH};
use common_lib::db::store::Store;
use common_lib::db::traits::Storable;

pub struct StorageDb {
    store: Store
}

impl StorageDb {

    pub fn instance() -> Self {
        Self {
            store: Store::instance()
        }
    }

    pub fn set_nft_core_contract_hash(&mut self, hash: ContractHash) {
        self.store.set(KEY_NFT_CORE_CONTRACT_HASH, hash);
    }

    pub fn get_nft_core_contract_hash(&self) -> Option<ContractHash> {
        self.store.get(KEY_NFT_CORE_CONTRACT_HASH)
    }

    pub fn set_current_contract_hash(&mut self, hash: ContractHash) {
        self.store.set(KEY_NFT_CONTRACT_HASH_NV, hash)
    }

    pub fn get_current_contract_hash(&mut self) -> Option<ContractHash> {
        self.store.get(KEY_NFT_CONTRACT_HASH_NV)
    }

}
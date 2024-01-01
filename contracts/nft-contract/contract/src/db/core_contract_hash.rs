use alloc::string::ToString;
use casper_types::Key;
use common_lib::constants::common_keys::NFTContractKeys;
use common_lib::db::store::Store;
use common_lib::db::traits::Storable;

trait CoreContract {
	fn set_nft_core_contract_hash(&mut self, key: Key) -> ();
	fn get_nft_core_contract_hash(&self) -> Option<Key>;
	fn set_current_contract_hash(&mut self, key: Key) -> ();
	fn get_current_contract_hash(&mut self) -> Option<Key>;
}

impl CoreContract for Store {
	fn set_nft_core_contract_hash(&mut self, key: Key) -> () {
		self.set(&NFTContractKeys::NFTCoreContractHash.to_string(), key)
	}

	fn get_nft_core_contract_hash(&self) -> Option<Key> {
		self.get(&NFTContractKeys::NFTCoreContractHash.to_string())
	}

	fn set_current_contract_hash(&mut self, key: Key) -> () {
		self.set(&NFTContractKeys::NFTCoreContractHashCurrent.to_string(), key)
	}

	fn get_current_contract_hash(&mut self) -> Option<Key> {
		self.get(&NFTContractKeys::NFTCoreContractHashCurrent.to_string())
	}
}

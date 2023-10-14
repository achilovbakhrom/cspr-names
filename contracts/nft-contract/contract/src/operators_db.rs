use alloc::string::ToString;
use casper_types::account::AccountHash;
use common_lib::constants::common_keys::KEY_NFT_OPERATORS;
use common_lib::db::dictionary::Dictionary;
use common_lib::db::traits::Storable;

pub struct OperatorsDb {
    store: Dictionary,
}

impl OperatorsDb {
    pub fn instance() -> Self {
        Self {
            store: Dictionary::instance(KEY_NFT_OPERATORS),
        }
    }

    pub fn initialize() {
        Dictionary::init(KEY_NFT_OPERATORS)
    }

    pub fn register_minter_as_operator(&mut self, account: AccountHash) {
        self.store.set(&account.to_string(), account)
    }

    pub fn is_minter_registered(&self, account: AccountHash) -> bool {
        let res: Option<AccountHash> = self.store.get(&account.to_string());
        res.is_some()
    }
}

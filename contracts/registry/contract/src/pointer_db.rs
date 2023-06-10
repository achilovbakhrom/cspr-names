use alloc::{
    format,
    string::String,
    vec::{self, Vec},
};
use casper_types::{account::AccountHash, ContractHash};
use common_lib::{
    constants::KEY_REGISTRY_OPERATORS,
    db::{store::Store, traits::Storable},
    enums::contracts_enum::ContractKind,
    errors::RegistryErrors,
    models::registry_pointer::RegistryPointer,
    utils::response::response_error,
};

pub(crate) struct PointStore {
    state: Store,
}

impl PointStore {
    pub fn instance() -> Self {
        Self {
            state: Store::instance(),
        }
    }

    pub fn add_contract_list(
        &self,
        kind: ContractKind,
        pointer: RegistryPointer,
        attr_key: Option<String>,
    ) {
        let key = match attr_key {
            Some(res) => &format!("{}:{}", kind, attr_key),
            None => &format!("{}", kind),
        };
        let pointer_list: Vec<RegistryPointer> = match self.state.get(key) {
            Some(res) => res,
            None => vec![],
        };

        match pointer_list
            .iter()
            .position(|x| x.contract_hash == pointer.contract_hash)
        {
            Some(pos) => {}
            None => {
                pointer_list.push(pointer);
                self.state.set(key, pointer_list);
            }
        }
    }

    pub fn remove_contract_list(
        &self,
        kind: ContractKind,
        hash: ContractHash,
        attr_key: Option<String>,
    ) {
        let key = match attr_key {
            Some(res) => &format!("{}:{}", kind, attr_key),
            None => &format!("{}", kind),
        };
        let pointer_list: Vec<RegistryPointer> = match self.state.get(key) {
            Some(res) => res,
            None => vec![],
        };

        match pointer_list
            .iter()
            .position(|x| x.contract_hash == pointer.contract_hash)
        {
            Some(pos) => {
                pointer_list.remove(pos);
                self.state.set(key, pointer_list);
            }
            None => {}
        }
    }

    pub fn get_contract_list(
        &self,
        kind: ContractKind,
        attr_key: Option<String>,
    ) -> Vec<RegistryPointer> {
        let key = match attr_key {
            Some(res) => &format!("{}:{}", kind, attr_key),
            None => &format!("{}", kind),
        };

        match self.state.get(key) {
            Some(res) => res,
            None => {
                let list = vec![];
                self.state.set(key, list);
                list
            }
        }
    }

    pub fn increment_contract_hash_count(
        &self,
        kind: ContractKind,
        attr_key: String,
        contract_hash: ContractHash,
    ) -> Result<(), RegistryErrors> {
        self.increment_contract_hash_count_by(kind, attr_key, contract_hash, 1)
    }

    pub fn decrement_contract_hash_count(
        &self,
        kind: ContractKind,
        attr_key: String,
        contract_hash: ContractHash,
    ) -> Result<(), RegistryErrors> {
        self.increment_contract_hash_count_by(kind, attr_key, contract_hash, -1)
    }

    pub fn increment_contract_hash_count_by(
        &self,
        kind: ContractKind,
        attr_key: String,
        contract_hash: ContractHash,
        by: u64,
    ) -> Result<(), RegistryErrors> {
        match list.iter().position(|x| x.contract_hash == contract_hash) {
            Some(pos) => {
                let count = list.get(pos).unwrap().count.unwrap();
                list.get(pos).unwrap().count = Some(count + by);
                self.state.set(&format!("{}:{}", kind, attr_key), list);
                Ok(())
            }
            None => Err(RegistryErrors::ContractHashNotFouond),
        }
    }
}

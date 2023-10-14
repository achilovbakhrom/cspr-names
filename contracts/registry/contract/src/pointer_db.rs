use alloc::{
    format,
    string::String,
    vec::{self, Vec},
};
use casper_types::ContractHash;
use common_lib::{
    db::{store::Store, traits::Storable},
    enums::contracts_enum::ContractKind,
    errors::RegistryErrors,
    models::registry_pointer::RegistryPointer,
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
            Some(res) => format!("{}:{}", kind, res),
            None => format!("{}", kind),
        };
        let mut pointer_list: Vec<RegistryPointer> = match self.state.get(&key) {
            Some(res) => res,
            None => Vec::<RegistryPointer>::new(),
        };

        match pointer_list
            .iter()
            .position(|x| x.contract_hash == pointer.contract_hash)
        {
            Some(_) => {}
            None => {
                pointer_list.push(pointer);
                self.state.set(&key, pointer_list);
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
            Some(res) => format!("{}:{}", kind, res),
            None => format!("{}", kind),
        };
        let mut pointer_list: Vec<RegistryPointer> = match self.state.get(&key) {
            Some(res) => res,
            None => Vec::<RegistryPointer>::new(),
        };

        match pointer_list.iter().position(|x| x.contract_hash == hash) {
            Some(pos) => {
                pointer_list.remove(pos);
                self.state.set(&key, pointer_list);
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
            Some(res) => format!("{}:{}", kind, res),
            None => format!("{}", kind),
        };

        match self.state.get(&key) {
            Some(res) => res,
            None => {
                let list = Vec::<RegistryPointer>::new();
                self.state.set(&key, list.clone());
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
        by: i32,
    ) -> Result<(), RegistryErrors> {
        let key = &format!("{}:{}", kind, attr_key);

        let mut pointer_list: Vec<RegistryPointer> = match self.state.get(key) {
            Some(res) => res,
            None => Vec::<RegistryPointer>::new(),
        };

        pointer_list.iter_mut().for_each(|x| {
            if x.contract_hash == contract_hash {
                x.count = Some(x.count.unwrap() + by);
            }
        });

        pointer_list.iter_mut().for_each(|x| {
            if x.contract_hash == contract_hash {
                x.count = Some(x.count.unwrap_or(0) + by);
            }
        });

        self.state
            .set(&format!("{}:{}", kind, attr_key), pointer_list);
        Ok(())
    }
}

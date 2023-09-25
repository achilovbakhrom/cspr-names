use alloc::format;
use common_lib::{
    db::{store::Store, traits::Storable},
    enums::contracts_enum::ContractKind,
};

pub(crate) struct MaxValueDb {
    state: Store,
}

impl MaxValueDb {
    pub fn instance() -> Self {
        Self {
            state: Store::instance(),
        }
    }

    pub fn set_max_value(&self, contract_kind: ContractKind, value: u64) {
        self.state.set(&format!("max:{}", contract_kind), value)
    }

    pub fn get_max_value(&self, contract_kind: ContractKind) -> Option<u64> {
        self.state.get(&format!("max:{}", contract_kind))
    }
}

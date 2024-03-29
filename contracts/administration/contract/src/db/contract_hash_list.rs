use alloc::{ vec::Vec, vec, string::ToString, format };
use casper_types::Key;
use common_lib::{
	db::{ store::Store, traits::Storable },
	enums::contracts_enum::ContractKind,
	models::registry_pointer::CompoundContract,
};

pub(crate) trait ContractHashList {
	fn set_simple_contract(&self, contract_kind: ContractKind, key: Key) -> ();
	fn get_simple_contract(&self, contract_kind: ContractKind) -> Option<Key>;

	fn init_compound_contracts(
		&self,
		contract_kind: ContractKind,
		extension: &str,
		keys: Vec<CompoundContract>
	) -> ();

	fn add_compound_contract(
		&self,
		contract_kind: ContractKind,
		extension: &str,
		key: CompoundContract
	) -> ();

	fn change_count_of_compound_contracts(
		&self,
		contract_kind: ContractKind,
		extension: &str,
		key: Key,
		value: i32
	) -> ();

	fn get_compound_contracts(
		&self,
		contract_kind: ContractKind,
		extension: &str
	) -> Vec<CompoundContract>;
}

impl ContractHashList for Store {
	fn set_simple_contract(&self, contract_kind: ContractKind, key: Key) -> () {
		self.set(&contract_kind.to_string(), key);
	}

	fn get_simple_contract(&self, contract_kind: ContractKind) -> Option<Key> {
		self.get::<Key>(&contract_kind.to_string())
	}

	fn init_compound_contracts(
		&self,
		contract_kind: ContractKind,
		extension: &str,
		keys: Vec<CompoundContract>
	) -> () {
		self.set(&format!("{}:{}", contract_kind, extension), keys);
	}

	fn add_compound_contract(
		&self,
		contract_kind: ContractKind,
		extension: &str,
		key: CompoundContract
	) -> () {
		let store_key = &format!("{}:{}", contract_kind, extension);
		let mut keys = self
			.get::<Vec<CompoundContract>>(store_key)
			.unwrap_or(vec![]);
		keys.push(key);
		self.set(store_key, keys);
	}

	fn change_count_of_compound_contracts(
		&self,
		contract_kind: ContractKind,
		extension: &str,
		key: Key,
		value: i32
	) -> () {
		let store_key = &format!("{}:{}", contract_kind, extension);
		let keys = self.get::<Vec<CompoundContract>>(store_key).unwrap_or(vec![]);

		let keys = keys
			.iter()
			.map(|item| {
				let count = (*item).count.unwrap_or(0) as i32;
				if (*item).key == key && count + value >= 0 {
					return CompoundContract {
						count: Some((count + value) as u32),
						..*item
					};
				}
				return *item;
			})
			.collect::<Vec<CompoundContract>>();

		self.set(store_key, keys);
	}

	fn get_compound_contracts(
		&self,
		contract_kind: ContractKind,
		extension: &str
	) -> Vec<CompoundContract> {
		let store_key = &format!("{}:{}", contract_kind, extension);
		self.get(&store_key).unwrap_or(vec![])
	}
}

use alloc::vec::Vec;
use casper_types::account::AccountHash;
use common_lib::{
	constants::KEY_REGISTRY_OPERATORS,
	db::{ store::Store, traits::Storable },
	errors::RegistryErrors,
	utils::response::response_error,
};

pub(crate) struct OperatorsDb {
	state: Store,
}

impl OperatorsDb {
	pub fn instance() -> Self {
		Self {
			state: Store::instance(),
		}
	}

	pub fn save_operator(&self, operator: AccountHash) {
		let mut operators: Vec<AccountHash> = self.state
			.get(KEY_REGISTRY_OPERATORS)
			.unwrap_or(Vec::<AccountHash>::new());

		match operators.iter().position(|item| item == &operator) {
			Some(index) => {
				return response_error(RegistryErrors::OperatorAlreadyExists);
			}
			None => operators.push(operator),
		}
		self.state.set(KEY_REGISTRY_OPERATORS, operators);
	}

	pub fn get_operators(&self) -> Vec<AccountHash> {
		let operators: Vec<AccountHash> = self.state
			.get(KEY_REGISTRY_OPERATORS)
			.unwrap_or(Vec::<AccountHash>::new());

		operators
	}

	pub fn remove_operator(&self, operator: AccountHash) {
		let mut operators: Vec<AccountHash> = self.state
			.get(KEY_REGISTRY_OPERATORS)
			.unwrap_or(Vec::<AccountHash>::new());

		match operators.iter().position(|item| item == &operator) {
			Some(index) => {
				operators.remove(index);
			}
			None => {
				return response_error(RegistryErrors::OperatorDoesntExist);
			}
		}
		self.state.set(KEY_REGISTRY_OPERATORS, operators);
	}
}

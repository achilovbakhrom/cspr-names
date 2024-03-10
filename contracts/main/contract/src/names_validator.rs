use alloc::format;
use alloc::string::{ String, ToString };
use alloc::vec::Vec;
use casper_contract::contract_api::runtime;
use casper_types::account::AccountHash;
use common_lib::errors::MainContractErrors;

pub const PUBLIC_ALLOWED_NAME_COUNT: usize = 3;

pub struct NamesValidator {
	allowed_names: Vec<String>,
	is_maintainer: bool,
}

impl NamesValidator {
	pub fn instance(allowed_names: Vec<String>, is_maintainer: bool) -> Self {
		Self {
			allowed_names,
			is_maintainer,
		}
	}

	pub fn validate_name(
		&self,
		arg: String
	) -> Result<NamesModel, MainContractErrors> {
		let split = arg.split(".").collect::<Vec<&str>>();

		if split.len() < 2 {
			return Err(MainContractErrors::InvalidName);
		}

		let model = NamesModel::from(arg);

		if !self.allowed_names.contains(&model.extension) {
			return Err(MainContractErrors::InvalidName);
		}

		let caller = runtime::get_caller();
		if model.name.len() <= 3 && !self.is_maintainer {
			return Err(MainContractErrors::InvalidCreator);
		}

		Ok(model)
	}
}

pub struct NamesModel {
	pub extension: String,
	pub name: String,
	pub sub_name: Option<String>,
}

impl From<String> for NamesModel {
	fn from(value: String) -> Self {
		let split = value
			.split(".")
			.map(|item| item.to_string())
			.collect::<Vec<String>>();

		let mut sub_name: Option<String> = None;
		let mut name: String = "".to_string();
		let mut extension: String = "".to_string();

		if split.len() == 2 {
			name = split.get(0).unwrap().to_string();
			extension = split.get(1).unwrap().to_string();
		} else if split.len() > 2 {
			extension = split.last().unwrap().to_string();
			name = split
				.get(split.len() - 2)
				.unwrap()
				.to_string();
			let sub_name_vec = split
				.iter()
				.take(split.len() - 2)
				.map(|item| item.to_string())
				.collect::<Vec<String>>();
			sub_name = Some(sub_name_vec.join(".").to_string());
		}

		Self {
			extension,
			name,
			sub_name,
		}
	}
}

impl Into<String> for NamesModel {
	fn into(self) -> String {
		let mut result = format!("{}.{}", self.name, self.extension);
		if self.sub_name.is_some() {
			result = format!("{}.{}", self.sub_name.unwrap(), result);
		}

		result
	}
}

impl NamesModel {
	pub fn get_name_len(&self) -> usize {
		self.name.len()
	}
}

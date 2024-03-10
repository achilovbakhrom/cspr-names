use casper_types::{
	CLType,
	EntryPointAccess,
	EntryPointType,
	Parameter,
	CLTyped,
	ContractHash,
	EntryPoints,
	contracts::NamedKeys,
};
use common_lib::{
	constants::common_keys::AdministrationEndpoints,
	utils::contract::create_entrypoint,
};

pub fn init() {
	let mut entrypoints = EntryPoints::new();
	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetContractAuthorityList.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					ContractHash::cl_type()
				),
				Parameter::new(
					&AdministrationArgs::ContractAuthorities.to_string(),
					CLType::Any
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::AddContractAuthority.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					CLType::Key
				),
				Parameter::new(
					&AdministrationArgs::ContractAuthority.to_string(),
					CLType::Key
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetContractAuthorityList.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					ContractHash::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::RemoveContractAuthority.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractHash.to_string(),
					ContractHash::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(
					&AdministrationArgs::Extension.to_string(),
					Option::<String>::cl_type()
				)
			],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::AddContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::Key.to_string(), CLType::Key)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::IncrementContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::Key.to_string(), CLType::Key)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::DecrementContract.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(&AdministrationArgs::Key.to_string(), CLType::Key)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetAllowedExtensions.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetAllowedExtensions.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::AddExtension.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::RemoveExtension.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetCharsMinCount.to_string(),
			vec![],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetCharsMinCount.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::CharsCount.to_string(),
					u8::cl_type()
				),
				Parameter::new(
					&AdministrationArgs::Extension.to_string(),
					CLType::Option(Box::new(String::cl_type()))
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::GetListingLimit.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&AdministrationEndpoints::SetListingLimit.to_string(),
			vec![
				Parameter::new(
					&AdministrationArgs::ContractKind.to_string(),
					ContractKind::cl_type()
				),
				Parameter::new(
					&AdministrationArgs::CharsCount.to_string(),
					u32::cl_type()
				)
			],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::SetAuthorities.to_string(),
			vec![Parameter::new(&CommonArgs::Authorities.to_string(), CLType::Any)],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::AddAuthority.to_string(),
			vec![Parameter::new(&CommonArgs::Authority.to_string(), CLType::Key)],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::RemoveAuthority.to_string(),
			vec![Parameter::new(&CommonArgs::Authority.to_string(), CLType::Key)],
			CLType::Unit,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	entrypoints.add_entry_point(
		create_entrypoint(
			&CommonEndpoints::GetAuthorities.to_string(),
			vec![],
			CLType::Any,
			EntryPointAccess::Public,
			EntryPointType::Contract
		)
	);

	let named_keys = NamedKeys::new();

	setup_contract_info(entrypoints, named_keys)
}

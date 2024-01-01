use casper_types::EntryPoints;
use common_lib::{
	utils::contract::{ create_entrypoint, create_named_keys, create_contract },
	constants::common_keys::ENDPOINT_DATABASE_SAVE_DOMAIN_NAME,
};

pub fn init_call() {
	let mut entrypoints = EntryPoints::new();
	entrypoints.add_entry_point(
		create_entrypoint(
			ENDPOINT_DATABASE_SAVE_DOMAIN_NAME,
			vec![Parameter::new(ARG_DATABASE_DOMAIN_NAME, DomainName::cl_type())],
			ret,
			access,
			entry_point_type
		)
	)
}

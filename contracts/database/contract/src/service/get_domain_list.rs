use alloc::{ string::{ String, ToString }, vec::Vec };
use casper_contract::contract_api::runtime;
use common_lib::constants::common_keys::DatabaseArgs;

use crate::{ db::domain_list::DomainListStore, types::TResult };

pub fn get_domain_list() -> TResult<Vec<String>> {
	let page: u64 = runtime::get_named_arg(&DatabaseArgs::Page.to_string());
	let domains = DomainListStore::instance().get_domain_list(
		page.to_string().as_ref()
	);
	Ok(domains)
}

use crate::utils::response::controller;
use crate::service::authorities as authorities_service;

#[no_mangle]
pub extern "C" fn set_authorities() {
	controller(authorities_service::set_authorites)
}

#[no_mangle]
pub extern "C" fn add_authority() {
	controller(authorities_service::add_authority)
}

#[no_mangle]
pub extern "C" fn remove_authority() {
	controller(authorities_service::remove_authority)
}

#[no_mangle]
pub extern "C" fn get_authorities() {
	controller(authorities_service::get_authorities)
}

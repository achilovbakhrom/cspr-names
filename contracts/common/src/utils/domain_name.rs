use crate::{
    constants::{EXTENSION, GRACE_PERIOD, YEAR_IN_MILLIS},
    enums::domain_name_actual_state::DomainNameActualState,
};
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use sha3::{Digest, Keccak256};

pub fn namehash_label(namehash: [u8; 32], label: &str) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(label.as_bytes());
    let labelhash = hasher.finalize_reset();
    hasher.update(namehash);
    hasher.update(labelhash);
    hasher.finalize_reset().into()
}

pub fn is_domain_name_valid(domain_name: &str) -> bool {
    domain_name.len() < 256 && domain_name.ends_with(EXTENSION)
}

pub fn is_sub_domain_name_valid(subdomain_name: &str) -> (bool, Option<String>) {
    let split = subdomain_name.split('.').collect::<Vec<&str>>();
    let extension = *split.get(2).unwrap();
    if split.len() != 3 || extension != EXTENSION {
        return (false, None);
    }
    let skipped_iterable_copy = split.iter().skip(1).copied();

    let domain = skipped_iterable_copy.collect::<Vec<&str>>().join(".");
    (true, Some(domain))
}

pub fn calculate_domain_name_end_date(duration: u8) -> u64 {
    let current_time: u64 = runtime::get_blocktime().into();
    current_time + (duration as u64) * YEAR_IN_MILLIS
}

pub fn year_to_millis(year: u8) -> u64 {
    (year as u64) * YEAR_IN_MILLIS
}

pub fn is_extension_duration_correct(actual_end_time: u64, duration: u64) -> bool {
    let current_time: u64 = runtime::get_blocktime().into();

    let diff = actual_end_time + duration - current_time;

    diff < 3 * YEAR_IN_MILLIS
}

pub fn get_end_time_actual_state(end_time: Option<u64>) -> DomainNameActualState {
    if let Some(et) = end_time {
        let current_time: u64 = runtime::get_blocktime().into();
        if et >= current_time {
            return DomainNameActualState::Busy;
        } else if et < current_time && et + GRACE_PERIOD > current_time {
            return DomainNameActualState::GracePeriod;
        }
    }
    DomainNameActualState::Available
}

pub fn get_domain_name_chars_count(domain: &str) -> usize {
    let split = domain.split('.').collect::<Vec<&str>>();
    let main_part = split.first().unwrap();
    main_part.len()
}

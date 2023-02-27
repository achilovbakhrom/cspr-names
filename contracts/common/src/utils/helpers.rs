use alloc::{ vec::Vec };

pub fn is_array_contain<T: core::cmp::PartialEq>(arr: &Vec<T>, item: &T) -> bool {
    let found = arr.iter().find(|i| i == &item);
    found.is_some()
}

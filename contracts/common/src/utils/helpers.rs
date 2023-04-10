use alloc::format;
use alloc::string::{String, ToString};
use serde_json::json;

pub fn is_array_contain<T: PartialEq>(arr: &[T], item: &T) -> bool {
    let found = arr.iter().find(|i| i == &item);
    found.is_some()
}

pub fn get_metadata_schema(name: &str, token_id: &str) -> alloc::string::String {
    let meta_value = json!({
        "name": name,
        "symbol": "symbol",
        "token_id": token_id
    });

    meta_value.to_string()
}

pub fn concat(str1: &str, str2: &str, connector: &str) -> String {
    format!("{}{}{}", str1, connector, str2)
}
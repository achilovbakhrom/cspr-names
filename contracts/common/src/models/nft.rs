use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    name: String,
    symbol: String,
    token_id: String,
}

impl Metadata {
    pub fn new(name: String, token_id: String) -> Self {
        Self {
            name,
            token_id,
            symbol: "casper-names-token".to_string(),
        }
    }
}

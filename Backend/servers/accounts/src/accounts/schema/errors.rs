use serde::{Deserialize, Serialize};

// Login Error Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountError {
    pub is_error: bool,
    pub error_message: Option<String>,
}

impl AccountError {
    pub fn new() -> AccountError {
        AccountError {
            is_error: false,
            error_message: None,
        }
    }
}

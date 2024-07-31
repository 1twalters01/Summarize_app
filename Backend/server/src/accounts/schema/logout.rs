use crate::accounts::schema::errors::AccountError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logout {
    pub auth_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutResponse {
    pub account_error: AccountError,
    pub success: bool,
}

impl LogoutResponse {
    pub fn new() -> LogoutResponse {
        LogoutResponse {
            account_error: AccountError::new(),
            success: false,
        }
    }
}

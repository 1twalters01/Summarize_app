use serde::{Deserialize, Serialize};

use crate::accounts::schema::errors::AccountError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponseSchema {
    pub account_error: AccountError,
    pub success: bool,
}

impl RefreshTokenResponseSchema {
    pub fn new() -> RefreshTokenResponseSchema {
        RefreshTokenResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}

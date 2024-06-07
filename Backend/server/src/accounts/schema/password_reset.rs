use serde::{Serialize, Deserialize};
use crate::accounts::schema::errors::AccountError;

// Password Reset structs
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetRequestSchema {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetResponseSchema {
    pub account_error: AccountError,
    pub success: bool,
    // email a token to them
}

impl PasswordResetResponseSchema {
    pub fn new() -> PasswordResetResponseSchema {
        PasswordResetResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetConfirmRequestSchema {
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetConfirmResponseSchema {
    pub account_error: AccountError,
    pub success: bool,
}

impl PasswordResetConfirmResponseSchema {
    pub fn new() -> PasswordResetConfirmResponseSchema {
        PasswordResetConfirmResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}



use serde::{Deserialize, Serialize};
use crate::accounts::schema::{
    errors::AccountError,
    auth::AuthTokens
};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPasswordResponseSchema {
    pub account_error: AccountError,
    pub is_password_correct: bool,
    pub has_totp: bool,
    pub auth_tokens: Option<AuthTokens>,
    pub login_response_token: Option<String>,
}

impl LoginPasswordResponseSchema {
    pub fn new() -> LoginPasswordResponseSchema {
        LoginPasswordResponseSchema {
            account_error: AccountError::new(),
            is_password_correct: false,
            has_totp: false,
            auth_tokens: None,
            login_response_token: None,
        }
    }
}

// Login Totp Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTotpRequest {
    pub totp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTotpRequestSchema {
    pub login_password_token: String,
    pub totp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTotpResponseSchema {
    pub account_error: AccountError,
    pub is_totp_correct: bool,
    pub auth_tokens: Option<AuthTokens>,
}

impl LoginTotpResponseSchema {
    pub fn new() -> LoginTotpResponseSchema {
        LoginTotpResponseSchema {
            account_error: AccountError::new(),
            is_totp_correct: false,
            auth_tokens: None,
        }
    }
}



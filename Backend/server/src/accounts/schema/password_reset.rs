use serde::{Serialize, Deserialize};
use crate::accounts::schema::errors::AccountError;

// Password Reset structs
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetRequestSchema {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DualVerificationToken {
    pub verification_token: String,
    pub header_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetResponseSchema {
    pub account_error: AccountError,
    pub password_reset_response_token: Option<String>,
    // email a token to them
}

impl PasswordResetResponseSchema {
    pub fn new() -> PasswordResetResponseSchema {
        PasswordResetResponseSchema {
            account_error: AccountError::new(),
            password_reset_response_token: None,
        }
    }
}

// Password Reset Verify Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub verification_token: String, // thing they enter on the site
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequestSchema {
    pub header_token: String,       // opaque token in place of the email
    pub verification_token: String, // thing they enter on the site
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResponseSchema {
    pub account_error: AccountError,
    pub password_reset_response_token: Option<String>,
}

impl VerificationResponseSchema {
    pub fn new() -> VerificationResponseSchema {
        VerificationResponseSchema {
            account_error: AccountError::new(),
            password_reset_response_token: None,
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



use crate::accounts::schema::errors::AccountError;
use serde::{Deserialize, Serialize};

// Register Email Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterEmailRequestSchema {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DualVerificationToken {
    pub verification_token: String,
    pub header_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterEmailResponseSchema {
    pub account_error: AccountError,
    pub register_response_token: Option<String>,
}

impl RegisterEmailResponseSchema {
    pub fn new() -> RegisterEmailResponseSchema {
        RegisterEmailResponseSchema {
            account_error: AccountError::new(),
            register_response_token: None,
        }
    }
}

// Register Verify Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub verification_token: String, // thing they enter on the site
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequestSchema {
    pub header_token: String,      // opaque token in place of the email
    pub verification_code: String, // thing they enter on the site
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResponseSchema {
    pub account_error: AccountError,
    pub register_response_token: Option<String>,
}

impl VerificationResponseSchema {
    pub fn new() -> VerificationResponseSchema {
        VerificationResponseSchema {
            account_error: AccountError::new(),
            register_response_token: None,
        }
    }
}

// Register Details Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDetailsRequest {
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDetailsRequestSchema {
    pub register_verification_token: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDetailsResponseSchema {
    pub account_error: AccountError,
    pub success: bool,
}

impl RegisterDetailsResponseSchema {
    pub fn new() -> RegisterDetailsResponseSchema {
        RegisterDetailsResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}

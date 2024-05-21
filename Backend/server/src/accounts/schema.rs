use serde::{Deserialize, Serialize};

// Login Structs

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

// Login Email Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginEmailRequestSchema {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginEmailResponseSchema {
    pub account_error: AccountError,
    pub is_email_stored: bool,
    pub login_email_response_token: Option<String>,
}

impl LoginEmailResponseSchema {
    pub fn new() -> LoginEmailResponseSchema {
        LoginEmailResponseSchema {
            account_error: AccountError::new(),
            is_email_stored: false,
            login_email_response_token: None,
        }
    }
}

// Login Password Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPasswordRequest {
    pub password: String,
    pub remember_me: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPasswordRequestSchema {
    pub login_email_response_token: String, // Change to a token stored on redis?
    pub password: String,
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPasswordResponseSchema {
    pub account_error: AccountError,
    pub is_password_correct: bool,
    pub has_totp: bool,
    pub auth_token: Option<String>,
    pub login_password_response_token: Option<String>,
}

impl LoginPasswordResponseSchema {
    pub fn new() -> LoginPasswordResponseSchema {
        LoginPasswordResponseSchema {
            account_error: AccountError::new(),
            is_password_correct: false,
            has_totp: false,
            auth_token: None,
            login_password_response_token: None,
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
    pub login_password_response_token: String,
    pub totp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTotpResponseSchema {
    pub account_error: AccountError,
    pub is_totp_correct: bool,
    pub auth_token: Option<String>,
}

impl LoginTotpResponseSchema {
    pub fn new() -> LoginTotpResponseSchema {
        LoginTotpResponseSchema {
            account_error: AccountError::new(),
            is_totp_correct: false,
            auth_token: None,
        }
    }
}

// Register Structs
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
    pub is_email_stored: bool,
    pub register_response_token: Option<String>,
}

impl RegisterEmailResponseSchema {
    pub fn new() -> RegisterEmailResponseSchema {
        RegisterEmailResponseSchema {
            account_error: AccountError::new(),
            is_email_stored: false,
            register_response_token: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub verification_token: String, // thing they enter on the site
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequestSchema {
    pub header_token: String, // opaque token in place of the email
    pub verification_token: String,   // thing they enter on the site
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResponseSchema {
    pub account_error: AccountError,
    pub is_verification_token_correct: bool,
    pub verification_confirmation_token: Option<String>,
}

impl VerifyResponseSchema {
    pub fn new() -> VerifyResponseSchema {
        VerifyResponseSchema {
            account_error: AccountError::new(),
            is_verification_token_correct: false,
            verification_confirmation_token: None,
        }
    }
}

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


use serde::{Serialize, Deserialize};

// Login Structs

// Login Error Struct
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginError {
    pub is_error: bool,
    pub error_message: Option<String>,
}

impl LoginError {
    pub fn new() -> LoginError {
        LoginError {
            is_error: false,
            error_message: None,
        }
    }
}


// Login Email Structs
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginEmail {
    pub email: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginEmailResponse {
    pub login_error: LoginError,
    pub is_email_stored: bool,
}

impl LoginEmailResponse {
    pub fn new() -> LoginEmailResponse {
        LoginEmailResponse {
            login_error: LoginError::new(),
            is_email_stored: false,
        }
    }
}

// Login Password Structs
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginPassword {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginPasswordResponse {
    pub login_error: LoginError,
    pub password_content: PasswordContent,
}

impl LoginPasswordResponse {
    pub fn new() -> LoginPasswordResponse {
        LoginPasswordResponse {
            login_error: LoginError::new(),
            password_content: PasswordContent::new(),
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PasswordContent {
    pub is_email_stored: bool,
    pub is_password_correct: bool,
    pub has_totp: bool,
    pub token: Option<String>,
}

impl PasswordContent {
    pub fn new() -> PasswordContent {
        PasswordContent {
            is_email_stored: false,
            is_password_correct: false,
            has_totp: false,
            token: None,
        }
    }
}

// Login Totp Structs
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginTotp {
    pub email: String,
    pub password: String,
    pub totp: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct LoginTotpResponse {
    pub login_error: LoginError,
    pub totp_content: TotpContent,
}

impl LoginTotpResponse {
    pub fn new() -> LoginTotpResponse {
        LoginTotpResponse {
            login_error: LoginError::new(),
            totp_content: TotpContent::new(),
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TotpContent {
    pub is_email_stored: bool,
    pub is_password_correct: bool,
    pub has_totp: bool,
    pub is_totp_correct: bool,
    pub token: Option<String>,
}

impl TotpContent {
    pub fn new() -> TotpContent {
        TotpContent {
            is_email_stored: false,
            is_password_correct: false,
            has_totp: false,
            is_totp_correct: false,
            token: None,
        }
    }
}

// Register Structs
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RegisterEmail {
    pub email: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RegisterVerify {
    pub email: String,
    pub token: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RegisterDetails {
    pub email: String,
    pub token: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Activate {

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UsernameReset {
    pub email: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UsernameResetConfirm {
    pub username: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
    pub email: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PasswordResetConfirm {
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Logout {

}


use crate::accounts::schema::errors::AccountError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DualVerificationToken {
    pub verification_token: String,
    pub header_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRequestSchema {
    pub header_token: String,
    pub verification_code: String,
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


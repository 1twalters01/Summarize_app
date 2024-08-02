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


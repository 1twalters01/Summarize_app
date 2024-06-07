use serde::{Deserialize, Serialize};

use crate::accounts::schema::errors::AccountError;

#[derive(Serialize)]
pub struct GetCaptchaResponseSchema {
    pub account_error: AccountError,
    pub success: bool,
}

impl GetCaptchaResponseSchema {
    pub fn new() -> GetCaptchaResponseSchema {
        GetCaptchaResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}


#[derive(Deserialize)]
pub struct CaptchaResponse {
    pub token: String,
    pub response: String,
}

#[derive(Serialize)]
pub struct CaptchaResponseSchema {
    pub account_error: AccountError,
    pub success: bool,
}

impl CaptchaResponseSchema {
    pub fn new() -> CaptchaResponseSchema {
        CaptchaResponseSchema {
            account_error: AccountError::new(),
            success: false,
        }
    }
}


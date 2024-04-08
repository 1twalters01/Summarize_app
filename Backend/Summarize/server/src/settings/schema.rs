use serde::{Serialize, Deserialize};

// Settings Structs

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SettingsError {
    pub is_error: bool,
    pub error_message: Option<String>,
}

impl SettingsError {
    pub fn new() -> SettingsError {
        SettingsError {
            is_error: false,
            error_message: None,
        }
    }
}











#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeUsernameRequestStruct {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeUsernameResponseStruct {
    settings_error: SettingsError,
    success: bool,
}







#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeEmailRequestStruct {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeEmailResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangePasswordRequestStruct {
    pub password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangePasswordResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ToggleTotpRequestStruct {
    pub totp: String,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ToggleTotpResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeThemeRequestStruct {
    pub theme: Theme,
    pub password: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeThemeResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum ChangeThemeRequestStrict {
    Dark,
    Light,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ChangeThemeRequestStrictResponseStruct {
    settings_error: SettingsError,
    success: bool,
}

#[derive(Debug)]
#[derive(, Serialize, Deserialize)]
pub struct DeleteAccountRequestStruct {
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct DeleteAccountResponseStruct {
    pub settings_error: SettingsError,
    pub success: bool,
}

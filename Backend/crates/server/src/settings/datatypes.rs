use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ChangeUsernameStruct {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChangeEmailStruct {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChangePasswordStruct {
    pub password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToggleTotpStruct {
    pub totp: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChangeThemeStruct {
    pub theme: Theme,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeleteAccountStruct {
    pub password: String,
    pub password_confirmation: String,
}


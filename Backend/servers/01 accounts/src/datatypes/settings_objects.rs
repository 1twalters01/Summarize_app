use serde::{Deserialize, Serialize};

/// Token for change email settings
#[derive(Serialize, Deserialize)]
pub struct EmailTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) email: String,
}

/// Token for change username settings
#[derive(Serialize, Deserialize)]
pub struct UsernameTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) username: String,
}

/// Token for change password settings
#[derive(Serialize, Deserialize)]
pub struct PasswordTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) password_hash: String,
}

/// Token for change name password settings
#[derive(Serialize, Deserialize)]
pub struct NameTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
}

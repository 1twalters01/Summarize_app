use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UsernameTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) username: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct NameTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
}

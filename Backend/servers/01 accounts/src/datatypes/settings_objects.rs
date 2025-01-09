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

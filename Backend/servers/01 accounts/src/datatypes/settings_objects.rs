use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailTokenObject {
    pub(crate) user_uuid: String,
    pub(crate) email: String,
}

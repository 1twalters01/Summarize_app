use super::users::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRememberMe {
    pub remember_me: bool,
    pub user: User,
}

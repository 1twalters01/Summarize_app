use super::users::User;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct UserRememberMe {
    pub remember_me: bool,
    pub user: User,
}

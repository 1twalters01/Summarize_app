use redis::{Connection, RedisError};

use crate::{models::user::User, queries::redis::general::set_key_value_in_redis};

pub struct CacheService {
    con: Connection,
}

impl CacheService {
    pub fn new(con: Connection) -> Self {
        Self { con }
    }

    pub fn store_token_for_user(
        &mut self,
        token: &str,
        user: &User,
        expiry_in_seconds: Option<i64>,
    ) -> Result<(), RedisError> {
        let user_json = serde_json::to_string(&user).unwrap();
        set_key_value_in_redis(&mut self.con, token, &user_json, expiry_in_seconds)
    }
}

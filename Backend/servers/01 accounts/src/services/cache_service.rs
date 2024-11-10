use redis::{Connection, RedisError, RedisResult};

use crate::{
    models::user::User,
    datatypes::token_object::UserRememberMe
    queries::redis::general::{delete_key_in_redis, get_key_from_value_in_redis, set_key_value_in_redis},
};

pub struct CacheService {
    con: Connection,
}

impl CacheService {
    pub fn new(con: Connection) -> Self {
        Self { con }
    }

    pub fn store_key_value(
        &mut self,
        key: &str,
        value: &str,
        expiry_in_seconds: Option<i64>,
    ) -> Result<(), RedisError> {
        set_key_value_in_redis(&mut self.con, key, value, expiry_in_seconds)
    }

    pub fn delete_key(
        &mut self,
        key: &str,
    ) -> Result<(), RedisError> {
        delete_key_in_redis(&mut self.con, key)
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

    pub fn get_user_from_token(&mut self, token: &str) -> Result<Option<User>, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        match redis_result {
            Ok(user_json) => match serde_json::from_str(&user_json) {
                Ok(user) => return Ok(user),
                Err(err) => return Err(err.to_string()),
            },
            Err(err) => return Err(err.to_string()),
        }
    }

    pub fn get_user_and_remember_me_from_token(&mut self, token: &str) -> Result<Option<UserRememberMe>, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        match redis_result {
            Ok(user_and_remember_me_json) => match serde_json::from_str(&user_and_remember_me_json) {
                Ok(user_and_remember_me) => return Ok(user_and_remember_me),
                Err(err) => return Err(err.to_string()),
            },
            Err(err) => return Err(err.to_string()),
        }
    }
}

use redis::{Connection, RedisError, RedisResult};

use crate::{
    models::user::User,
    queries::redis::general::{
        delete_key_in_redis, get_key_from_value_in_redis, set_key_value_in_redis,
    },
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

    pub fn delete_key(&mut self, key: &str) -> Result<(), RedisError> {
        delete_key_in_redis(&mut self.con, key)
    }
}
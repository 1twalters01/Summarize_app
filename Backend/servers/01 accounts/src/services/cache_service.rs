use redis::{Connection, RedisError, RedisResult};
use uuid::Uuid;

use crate::{
    datatypes::settings_objects::{EmailTokenObject, UsernameTokenObject}, models::user::User, queries::redis::general::{
        delete_key_in_redis, get_key_from_value_in_redis, set_key_value_in_redis,
    }
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

    pub fn store_answer_for_token(
        &mut self,
        answer: &str,
        token: &str,
        expiry_in_seconds: Option<i64>,
    ) -> Result<(), RedisError> {
        set_key_value_in_redis(&mut self.con, token, answer, expiry_in_seconds)
    }

    pub fn get_answer_from_token(&mut self, answer: &str) -> Result<String, RedisError> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, answer);
        match redis_result {
            Ok(answer) => return Ok(answer),
            Err(err) => return Err(err),
        }
    }

    // Flip these name and parameters to store_user_uuid_for_token
    pub fn store_token_for_user_uuid(
        &mut self,
        token: &str,
        user_uuid: &Uuid,
        expiry_in_seconds: Option<i64>,
    ) -> Result<(), RedisError> {
        let user_json = serde_json::to_string(&user_uuid).unwrap();
        set_key_value_in_redis(&mut self.con, token, &user_json, expiry_in_seconds)
    }
    // Flip to store_email_token
    pub fn store_token_for_email(
        &mut self,
        token: &str,
        email: &str,
        expiry_in_seconds: Option<i64>,
    ) -> Result<(), RedisError> {
        set_key_value_in_redis(&mut self.con, token, &email, expiry_in_seconds)
    }

    // store_user_for_token
    pub fn store_token_for_user(
        &mut self,
        token: &str,
        user: &User,
        expiry_in_seconds: Option<i64>,
    ) -> Result<(), RedisError> {
        let user_json = serde_json::to_string(&user).unwrap();
        set_key_value_in_redis(&mut self.con, token, &user_json, expiry_in_seconds)
    }

    pub fn get_user_uuid_from_token(&mut self, token: &str) -> Result<Option<Uuid>, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        match redis_result {
            Ok(user_uuid_json) => match serde_json::from_str(&user_uuid_json) {
                Ok(user_uuid) => return Ok(user_uuid),
                Err(err) => return Err(err.to_string()),
            },
            Err(err) => return Err(err.to_string()),
        }
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

    pub fn get_user_uuid_and_remember_me_from_token(
        &mut self,
        token: &str,
    ) -> Result<Option<(Uuid, bool)>, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        match redis_result {
            Ok(user_uuid_and_remember_me_json) => {
                match serde_json::from_str(&user_uuid_and_remember_me_json) {
                    Ok(user_uuid_and_remember_me) => return Ok(user_uuid_and_remember_me),
                    Err(err) => return Err(err.to_string()),
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    pub fn get_email_from_token_struct_json(
        &mut self,
        token_struct_json: &str,
    ) -> Result<String, String> {
        let redis_result: RedisResult<String> =
            get_key_from_value_in_redis(&mut self.con, token_struct_json);
        match redis_result {
            Ok(email) => return Ok(email),
            Err(err) => return Err(err.to_string()),
        }
    }

    pub fn get_email_from_token(&mut self, token: &str) -> Result<String, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        match redis_result {
            Ok(email) => return Ok(email),
            Err(err) => return Err(err.to_string()),
        }
    }

    pub fn get_email_object_from_token(mut self, token: &str) -> Result<EmailTokenObject, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        let object_json: String = match redis_result {
            Ok(object_json) => object_json,
            Err(err) => return Err(err.to_string()),
        };
        let object: EmailTokenObject = serde_json::from_str(&object_json).unwrap();
        return Ok(object);
    }

    pub fn get_username_object_from_token(mut self, token: &str) -> Result<UsernameTokenObject, String> {
        let redis_result: RedisResult<String> = get_key_from_value_in_redis(&mut self.con, token);
        let object_json: String = match redis_result {
            Ok(object_json) => object_json,
            Err(err) => return Err(err.to_string()),
        };
        let object: UsernameTokenObject = serde_json::from_str(&object_json).unwrap();
        return Ok(object);
    }
}

use crate::accounts::datatypes::{token_object::UserRememberMe, users::User};
use redis::{Commands, Connection, RedisResult};

pub fn get_user_from_token_in_redis(mut con: Connection, token: &str) -> Result<User, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let user_json: String = match redis_result {
        Ok(user_json) => user_json,
        Err(err) => return Err(err.to_string()),
    };
    let user: User = serde_json::from_str(&user_json).unwrap();
    return Ok(user);
}

pub fn get_user_remember_me_from_token_in_redis(
    mut con: Connection,
    token: &str,
) -> Result<UserRememberMe, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let user_remember_me_json: String = match redis_result {
        Ok(user_remember_me_json) => user_remember_me_json,
        Err(err) => return Err(err.to_string()),
    };
    let user_remember_me: UserRememberMe = serde_json::from_str(&user_remember_me_json).unwrap();
    return Ok(user_remember_me);
}

pub fn get_email_from_token_struct_in_redis(
    mut con: Connection,
    token_struct: &str,
) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token_struct);
    let email: String = match redis_result {
        Ok(email) => email,
        Err(err) => return Err(err.to_string()),
    };
    return Ok(email);
}

pub fn get_user_json_from_token_struct_in_redis(
    mut con: Connection,
    token_struct: &str,
) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token_struct);
    match redis_result {
        Ok(user_json) => return Ok(user_json),
        Err(err) => return Err(err.to_string()),
    };
}

pub fn get_code_from_token_in_redis(mut con: Connection, token: &str) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let code: String = match redis_result {
        Ok(code) => code,
        Err(err) => return Err(err.to_string()),
    };
    return Ok(code);
}

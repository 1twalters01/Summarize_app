use crate::models::user::User;
use redis::{Commands, Connection, RedisResult};

pub fn get_user_from_token_in_redis(con: &mut Connection, token: &str) -> Result<User, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let user_json: String = match redis_result {
        Ok(user_json) => user_json,
        Err(err) => return Err(err.to_string()),
    };
    let user: User = serde_json::from_str(&user_json).unwrap();
    return Ok(user);
}

pub fn get_user_json_from_token_struct_in_redis(
    con: &mut Connection,
    token_struct: &str,
) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token_struct);
    match redis_result {
        Ok(user_json) => return Ok(user_json),
        Err(err) => return Err(err.to_string()),
    };
}

pub fn get_code_from_token_in_redis(con: &mut Connection, token: &str) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let code: String = match redis_result {
        Ok(code) => code,
        Err(err) => return Err(err.to_string()),
    };
    return Ok(code);
}

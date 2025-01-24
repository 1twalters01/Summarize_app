use redis::{Commands, Connection, RedisResult};

pub fn set_key_value_in_redis(
    con: &mut Connection,
    key: &str,
    value: &str,
    expiry_in_seconds: Option<i64>,
) -> RedisResult<()> {
    let _: () = con.set(key, value)?;

    if let Some(expiry) = expiry_in_seconds {
        let _: () = con.expire(key, expiry)?;
    }

    Ok(())
}

pub fn get_value_from_key_in_redis(con: &mut Connection, key: &str) -> RedisResult<String> {
    con.get(key)
}

pub fn delete_key_in_redis(con: &mut Connection, key: &str) -> RedisResult<()> {
    con.del(key)
}

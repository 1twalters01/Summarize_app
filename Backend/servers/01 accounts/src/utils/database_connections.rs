use redis::{Commands, Connection, RedisResult};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn create_pg_pool_connection() -> Pool<Postgres> {
    let url: String = env::var("PG_URL").unwrap();
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await
        .unwrap();
    return pool;
}

pub fn create_redis_client_connection() -> Connection {
    let url: String = env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}

pub fn set_key_value_in_redis(
    mut con: Connection,
    key: &str,
    value: &str,
    expiry_in_seconds: &Option<i64>,
) -> RedisResult<()> {
    con.set(key, value)?;

    if let Some(expiry) = expiry_in_seconds {
        con.expire(key, *expiry)?;
    };

    Ok(())
}

pub async fn delete_key_in_redis(mut con: Connection, key: &str) -> RedisResult<()> {
    con.del(key)
}

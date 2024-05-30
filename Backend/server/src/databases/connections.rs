use redis::{Commands, Connection, RedisResult};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub async fn create_pg_pool_connection() -> Pool<Postgres> {
    let username: String = env::var("PG_USERNAME").unwrap();
    let password: String = env::var("PG_PASSWORD").unwrap();
    let port: String = String::from("5432");
    let dbname: String = env::var("PG_DB_NAME").unwrap();

    let url: String = format!("postgresql://{username}:{password}@localhost:{port}/{dbname}");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await
        .unwrap();
    return pool;
}

pub fn create_redis_client_connection() -> Connection {
    let url = format!("redis://localhost:6379");
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}

pub async fn set_key_value_in_redis(
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

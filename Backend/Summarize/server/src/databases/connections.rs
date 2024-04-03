use std::{env, io::Error, result::Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use redis::AsyncCommands;
use futures::prelude::*;

pub async fn create_pg_pool_connection() -> Pool<Postgres> {
    let username: String = env::var("PG_USERNAME").unwrap();
    let password: String = env::var("PG_PASSWORD").unwrap();
    let host: String = 5432;
    let dbname: String = env::var("PG_DB_NAME").unwrap();

    let url: String = format!("postgres://{username}:{password}@localhost:{port}/+{dbname}");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await
        .unwrap();
    return pool;
}

pub async fn create_redis_client_connection() -> Redis {
    let client = redis::Client::open(url)?;
    let mut con = client.get_async_connection().await?;
}


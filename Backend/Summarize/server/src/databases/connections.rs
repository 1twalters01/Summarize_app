use std::env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use redis::Connection;

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
    let url = format!("redis://127.0.0.1/");
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}


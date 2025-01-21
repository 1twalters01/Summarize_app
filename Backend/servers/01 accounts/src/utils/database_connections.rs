use redis::Connection;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

/// Create connection pool to postgres
pub async fn create_pg_pool_connection() -> Pool<Postgres> {
    let url: String = env::var("PG_URL").unwrap();
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await
        .unwrap();
    return pool;
}

/// Create connection pool to redis
pub fn create_redis_client_connection() -> Connection {
    let url: String = env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_create_pg_pool_connection() {
        dotenv().ok();

        let pool = create_pg_pool_connection().await;
        assert!(pool.acquire().await.is_ok());
    }

    #[test]
    fn test_create_redis_client_connection() {
        dotenv().ok();

        let mut con = create_redis_client_connection();
        let _: () = redis::cmd("PING").query(&mut con).unwrap(); // Should return "PONG"
    }
}

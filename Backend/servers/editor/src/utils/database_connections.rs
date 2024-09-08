use redis::{Commands, Connection, RedisResult};
use std::env;

pub fn create_redis_client_connection() -> Connection {
    let url: String = env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}

use UUID::Uuid;
use sqlx::{Pool, Postgres};

pub async fn user_uuid_from_refresh_token(pool: &Pool<Postgres>, refresh_token: &str) -> Result<Option<Uuid>, sqlx::Error> {
    // fix this, need a join between refresh tokens table and the users table
    let get_refresh_token_query =
        sqlx::query("SELECT u.uuid from users u Join refresh_tokens r ON u.id=r.user_id WHERE r.refresh_token=($1)")
            .bind(refresh_token)
            .fetch_one(pool)
            .await;

    match get_refresh_token_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let user_uuid: Option<Uuid> = res[0].get("uuid");
            return user_uuid;
        }
    }
}

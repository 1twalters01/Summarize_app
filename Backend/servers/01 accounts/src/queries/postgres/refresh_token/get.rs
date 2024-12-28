use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub async fn user_uuid_from_refresh_token(
    pool: &Pool<Postgres>,
    refresh_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
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

            let user_uuid: Option<Uuid> = res.get("uuid");
            return Ok(user_uuid);
        }
    }
}

pub async fn created_at_and_expires_at_from_refresh_token(
    pool: &Pool<Postgres>,
    refresh_token: &str,
) -> Result<Option<(DateTime<Utc>, DateTime<Utc>)>, sqlx::Error> {
    // fix this, need a join between refresh tokens table and the users table
    let get_refresh_token_query =
        sqlx::query("SELECT created_at, expires_at from refresh_tokens WHERE r.refresh_token=($1)")
            .bind(refresh_token)
            .fetch_one(pool)
            .await;

    match get_refresh_token_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let created_at: Option<DateTime<Utc>> = res.get("created_at");
            let expires_at: Option<DateTime<Utc>> = res.get("expires_at");
            return Ok(Some((
                created_at.expect("invalid"),
                expires_at.expect("invalid"),
            )));
        }
    }
}

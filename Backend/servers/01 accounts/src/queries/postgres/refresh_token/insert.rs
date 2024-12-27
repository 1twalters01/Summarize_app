use chrono::Duration;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn from_user_uuid_and_refresh_token(
    pool: &Pool<Postgres>,
    user_uuid: &Uuid,
    refresh_token: &str,
    remember_me: bool,
) -> Result<(), sqlx::Error> {
    let duration: Duration;

    if remember_me == true {
        duration = Duration::days(7)
    } else {
        duration = Duration::days(1)
    }
    let save_refresh_token_query = sqlx::query(
        "
        INSERT INTO refresh_tokens (user_id, refresh_token, expires_at)
        SELECT id, $1, $2
        FROM user u
        WHERE uuid=($3)
        ",
    )
    .bind(refresh_token)
    .bind(duration)
    .bind(user_uuid)
    .execute(pool)
    .await;

    if let Err(err) = save_refresh_token_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

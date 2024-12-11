use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn update_login_time(
    pool: &Pool<Postgres>,
    last_login: DateTime<Utc>,
    uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let user_delete_query = sqlx::query("UPDATE users SET last_login=($1) WHERE uuid=($2);")
        .bind(last_login)
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_delete_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

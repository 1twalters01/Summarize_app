use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn from_uuid(pool: &Pool<Postgres>, uuid: &Uuid) -> Result<(), sqlx::Error> {
    let user_delete_query = sqlx::query("Delete FROM users WHERE uuid=($1);")
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_delete_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

pub async fn totp_from_uuid(
    pool: &Pool<Postgres>,
    user_uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let user_select_query = sqlx::query("UPDATE users SET totp_key=NULL WHERE uuid=($1)")
        .bind(user_uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_select_query {
        return Err(err);
    } else {
        return Ok(());
    }
}


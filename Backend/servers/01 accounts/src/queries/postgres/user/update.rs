use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn update_login_time_from_uuid(
    pool: &Pool<Postgres>,
    last_login: DateTime<Utc>,
    uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET last_login=($1) WHERE uuid=($2);")
        .bind(last_login)
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

pub async fn update_email_from_uuid(
    pool: &Pool<Postgres>,
    email: &str,
    uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET email=($1) WHERE uuid=($2);")
        .bind(email)
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

pub async fn update_name_from_uuid(
    pool: &Pool<Postgres>,
    first_name: Option<&String>,
    last_name: Option<&String>,
    uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let user_update_query =
        sqlx::query("UPDATE users SET first_name=($1), last_name=($2) WHERE uuid=($3);")
        .bind(first_name)
        .bind(last_name)
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

pub async fn update_language_from_uuid(
    pool: &Pool<Postgres>,
    language: &str,
    uuid: &Uuid,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET language=($1) WHERE uuid=($2);")
        .bind(language)
        .bind(uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

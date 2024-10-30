use crate::models::user::User;
use sqlx::{Pool, Postgres};

pub async fn from_user_and_refresh_token(
    pool: &Pool<Postgres>,
    user: &User,
    refresh_token: &str,
) -> Result<(), sqlx::Error> {
    let save_refresh_token_query =
        sqlx::query("INSERT INTO auth WHERE refresh_token=($1), username=($2)")
            .bind(refresh_token)
            .bind(user.get_uuid().to_string())
            .execute(pool)
            .await;

    if let Err(err) = save_refresh_token_query {
        return Err(err);
    } else {
        return Ok(());
    }
}


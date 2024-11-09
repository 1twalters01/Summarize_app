use crate::models::user::User;
use sqlx::{Pool, Postgres};

pub async fn from_user(pool: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET password=($1) WHERE uuid=($2);")
        // .bind(user)
        .bind(user.get_password())
        .bind(user.get_uuid())
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        println!("error: {:?}", err);
        return Err(err);
    } else {
        return Ok(());
    }
}

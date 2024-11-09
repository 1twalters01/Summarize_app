use crate::models::user::User;
use sqlx::{Pool, Postgres};

pub async fn from_user(pool: &Pool<Postgres>, user: User) -> Result<(), sqlx::Error> {
    let user_create_query = sqlx::query("INSERT INTO users(uuid, email, username, password, first_name, last_name) VALUES (($1), ($2), ($3), ($4), ($5), ($6));")
        .bind(user.get_uuid())
        .bind(user.get_email())
        .bind(user.get_username())
        .bind(user.get_password())
        .bind(user.get_first_name())
        .bind(user.get_last_name())
        .execute(pool)
        .await;

    if let Err(err) = user_create_query {
        println!("err: {:#?}", err);
        return Err(err);
    } else {
        println!("yooo");
        return Ok(());
    }
}

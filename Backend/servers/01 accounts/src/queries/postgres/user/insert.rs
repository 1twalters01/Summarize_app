use crate::models::user::User;
use sqlx::{Pool, Postgres, Row};

pub async fn from_user(pool: &Pool<Postgres>, user: User) -> Result<(), sqlx::Error> {
    let user_create_query = sqlx::query("INSERT INTO users(uuid, username, email, first_name, last_name) VALUES (($1), ($2), ($3), ($4), ($5));")
        .bind(user.get_uuid())
        .bind(user.get_username())
        .bind(user.get_email())
        .bind(user.get_first_name())
        .bind(user.get_last_name())
        .fetch_one(pool)
        .await;

    match user_create_query {
        Err(err) => {
            println!("err: {:#?}", err);
            return Err(err);
        },
        Ok(res) => {
            let user_id: i32 = res.get("id");
            // join for u.id where u.uuid = user.get_uuid()
            let password_create_query = sqlx::query("INSERT INTO password_history(user_id, password_hash) VALUES (($1), ($2));")
                .bind(user_id)
                .bind(user.get_password())
                .execute(pool)
                .await;
            if let Err(err) = password_create_query {
                println!("err: {:#?}", err);
                return Err(err);
            }
        }
    }

    return Ok(());
}

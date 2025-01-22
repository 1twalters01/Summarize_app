use chrono::Utc;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub async fn new_guest(pool: &Pool<Postgres>) -> Result<Uuid, sqlx::Error> {
    let user_create_query = sqlx::query("INSERT INTO users (last_login) VALUES (($1)) RETURNING uuid;")
        .bind(Utc::now())
        .fetch_one(pool)
        .await;

    match user_create_query {
        Err(err) => return Err(err),
        Ok(res) => {
            let user_uuid: Uuid = res.get("uuid");
            return Ok(user_uuid);
        }
    }
}

pub async fn from_all(
    pool: &Pool<Postgres>,
    username: &str,
    email: &str,
    first_name: Option<&str>,
    last_name: Option<&str>,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    let user_create_query = sqlx::query("INSERT INTO users(username, email, first_name, last_name) VALUES (($1), ($2), ($3), ($4)) RETURNING id;")
        .bind(username)
        .bind(email)
        .bind(first_name)
        .bind(last_name)
        .fetch_one(pool)
        .await;

    match user_create_query {
        Err(err) => {
            println!("err: {:#?}", err);
            return Err(err);
        }
        Ok(res) => {
            let user_id: i32 = res.get("id");
            // join for u.id where u.uuid = user.get_uuid()
            let password_create_query = sqlx::query(
                "INSERT INTO password_history(user_id, password_hash) VALUES (($1), ($2));",
            )
            .bind(user_id)
            .bind(password_hash)
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

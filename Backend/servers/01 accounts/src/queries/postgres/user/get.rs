use crate::models::user::User;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub async fn from_email(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE email=($1)")
        .bind(email)
        .fetch_all(pool)
        .await;

    match user_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let id: Option<Uuid> = res[0].get("uuid");
            let username: String = res[0].get("username");
            let email: String = res[0].get("email");
            let password: String = res[0].get("password");
            let first_name: Option<String> = res[0].get("first_name");
            let last_name: Option<String> = res[0].get("last_name");

            let user = match id {
                None => None,
                Some(id) => Some(
                    User::from_all(id, username, email, password, first_name, last_name).unwrap(),
                ),
            };

            return Ok(user);
        }
    }
}

pub async fn from_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE username=($1)")
        .bind(username)
        .fetch_all(pool)
        .await;

    match user_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let id: Uuid = res[0].get("uuid");
            let username: String = res[0].get("username");
            let email: String = res[0].get("email");
            let password: String = res[0].get("password");
            let first_name: Option<String> = res[0].get("first_name");
            let last_name: Option<String> = res[0].get("last_name");

            let user: User =
                User::from_all(id, username, email, password, first_name, last_name).unwrap();
            println!("user: {:#?}", user);
            return Ok(Some(user));
        }
    }
}

pub async fn from_uuid(
    pool: &Pool<Postgres>,
    uuid: &Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE uuid=($1)")
        .bind(uuid)
        .fetch_all(pool)
        .await;

    match user_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let id: Uuid = res[0].get("uuid");
            let username: String = res[0].get("username");
            let email: String = res[0].get("email");
            let password: String = res[0].get("password");
            let first_name: Option<String> = res[0].get("first_name");
            let last_name: Option<String> = res[0].get("last_name");

            let user: User =
                User::from_all(id, username, email, password, first_name, last_name).unwrap();
            println!("user: {:#?}", user);
            return Ok(Some(user));
        }
    }
}

pub async fn from_refresh_token(
    pool: &Pool<Postgres>,
    refresh_token: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user_select_query = sqlx::query("Select user from refresh_tokens WHERE refresh_token=($1)")
        .bind(refresh_token)
        .fetch_all(pool)
        .await;

    match user_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let id: Uuid = res[0].get("uuid");
            let username: String = res[0].get("username");
            let email: String = res[0].get("email");
            let password: String = res[0].get("password");
            let first_name: Option<String> = res[0].get("first_name");
            let last_name: Option<String> = res[0].get("last_name");

            let user: User =
                User::from_all(id, username, email, password, first_name, last_name).unwrap();
            println!("user: {:#?}", user);
            return Ok(Some(user));
        }
    }
}

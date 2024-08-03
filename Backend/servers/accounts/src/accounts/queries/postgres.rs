use crate::accounts::datatypes::users::User;
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub async fn create_new_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    user: User,
) -> Result<(), sqlx::Error> {
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

pub async fn delete_user_from_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    uuid: &str,
) -> Result<(), sqlx::Error> {
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

pub async fn update_password_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    user: &User,
) -> Result<(), sqlx::Error> {
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

pub async fn get_user_from_email_in_pg_users_table(
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

pub async fn get_user_from_username_in_pg_users_table(
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

pub async fn get_user_from_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    uuid: &str,
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

pub async fn save_refresh_token_user_in_postgres_auth_table(
    pool: &Pool<Postgres>,
    refresh_token: &str,
    user: &User,
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

pub async fn get_user_from_refresh_token_in_postgres_auth_table(
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

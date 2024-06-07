use crate::accounts::datatypes::users::User;
use sqlx::{Pool, Postgres};

pub async fn create_new_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    user: User,
) -> Result<(), sqlx::Error> {
    let user_create_query = sqlx::query("INSERT INTO users WHERE uuid=($1), email=($2), username=($3), password=($4), first_name=($5), last_name=($6)")
        .bind(user.get_uuid().to_string())
        .bind(user.get_email())
        .bind(user.get_username())
        .bind(user.get_password())
        .bind(user.get_first_name())
        .bind(user.get_last_name())
        .execute(pool)
        .await;

    if let Err(err) = user_create_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

pub async fn delete_user_from_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    uuid: &str,
) -> Result<(), sqlx::Error> {
    let user_delete_query = sqlx::query("Delete FROM users WHERE uuid=($1)")
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
    // user: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("")
        // .bind(user)
        .bind(password)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

pub async fn get_user_from_email_in_pg_users_table(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<User, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE email=($1)")
        .bind(email)
        .fetch_all(pool)
        .await;

    if let Err(err) = user_select_query {
        return Err(err);
    }

    let username = "username".to_string();
    let email = "email".to_string();
    let password = "password".to_string();
    let user: User = User::new(username, email, password).unwrap();
    return Ok(user);
}

pub async fn get_user_from_username_in_pg_users_table(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<User, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE username=($1)")
        .bind(username)
        .fetch_all(pool)
        .await;

    if let Err(err) = user_select_query {
        return Err(err);
    }

    let username = "username".to_string();
    let email = "email".to_string();
    let password = "password".to_string();
    let user: User = User::new(username, email, password).unwrap();
    return Ok(user);
}

pub async fn get_user_from_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    uuid: &str,
) -> Result<User, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE uuid=($1)")
        .bind(uuid)
        .fetch_all(pool)
        .await;

    if let Err(err) = user_select_query {
        return Err(err);
    }

    let username = "username".to_string();
    let email = "email".to_string();
    let password = "password".to_string();
    let user: User = User::new(username, email, password).unwrap();
    return Ok(user);
}

pub async fn save_refresh_token_user_in_postgres_auth_table(
    pool: &Pool<Postgres>,
    refresh_token: &str,
    user: &User,
) -> Result<(), sqlx::Error> {
    let save_refresh_token_query =
        sqlx::query("INSERT INTO auth WHERE refresh_token=($1), user=($2)ame=($6)")
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
) -> Result<User, sqlx::Error> {
    let user_select_query = sqlx::query("Select user from refresh_tokens WHERE refresh_token=($1)")
        .bind(refresh_token)
        .fetch_all(pool)
        .await;

    if let Err(err) = user_select_query {
        return Err(err);
    }
    let username = "username".to_string();
    let email = "email".to_string();
    let password = "password".to_string();
    let user: User = User::new(username, email, password).unwrap();
    return Ok(user);
}

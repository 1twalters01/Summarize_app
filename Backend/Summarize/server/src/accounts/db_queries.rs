use sqlx::{Pool, Postgres};
use crate::accounts::datatypes::users::User;

pub fn fake_postgres_check_email(email: &String) -> bool {
    let email_database = vec![
        String::from("test@something.com"),
        String::from("test2@something.com"),
        String::from("test3@something.com")
    ];
    return email_database.contains(email);
}

pub async fn get_user_from_email_in_pg_users_table(pool: &Pool<Postgres>, email: &str) -> Result<User, sqlx::Error> {
    let user_select_query = sqlx::query("Select * from users WHERE email=($1)")
        .bind(email)
        .fetch_all(pool)
        .await;

    if let Err(err) = user_select_query { return Err(err) }

    let username = "username".to_string();
    let email = "email".to_string();
    let password = "password".to_string();
    let user: User = User::new(username, email, password).unwrap();
    return Ok(user)
}

pub fn set_token_user_in_redis(email: &str, user: User) {
}

pub fn get_user_from_token_in_redis(email: &str) -> Result<User, Error> {
}

pub fn delete_email_user_in_redis(email: &str) {
}

fn fake_postgres_check_password(email: &String) -> bool {
    let email_database = vec![
        String::from("test@something.com"),
        String::from("test2@something.com"),
    ];
    return email_database.contains(email);
}

fn fake_postgres_check_totp(email: &String) -> bool {
    let email_database = vec![
        String::from("test@something.com"),
        String::from("test1@something.com"),
    ];
    return email_database.contains(email);
}


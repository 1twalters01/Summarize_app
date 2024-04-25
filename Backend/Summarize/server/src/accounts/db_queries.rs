use redis::{Commands, Connection, ExistenceCheck, RedisResult, SetExpiry, SetOptions};
use sqlx::{Pool, Postgres};
use crate::accounts::datatypes::users::User;

use super::datatypes::token_object::UserRememberMe;


pub async fn create_new_user_in_pg_users_table(pool: &Pool<Postgres>, user: User) -> Result<(), sqlx::Error> {
    let user_create_query = sqlx::query("INSERT INTO users WHERE email=($1), username=($2), password=($3), password_confirmation=($4), first_name=($5), last_name=($6)")
        .bind(user.get_email())
        .execute(pool)
        .await;

    if let Err(err) = user_create_query {
        return Err(err)
    } else {
        return Ok(());
    }
}


pub async fn update_password_for_user_in_pg_users_table(pool: &Pool<Postgres>, user: &User) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("")
        .bind(user.password)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err)
    } else {
        return Ok(());
    }
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


pub async fn set_token_user_in_redis(mut con: Connection, token: &str, user_json: &str, expiry_in_seconds: &Option<i64>) -> RedisResult<Vec<usize>> {
    let expiry: usize = match expiry_in_seconds {
        Some(expiry_in_seconds) => *expiry_in_seconds as usize,
        None => 0
    };

    let opts: SetOptions = SetOptions::default()
        .conditional_set(ExistenceCheck::NX)
        .get(true)
        .with_expiration(SetExpiry::EX(expiry));
    con.set_options(token, user_json, opts)
}

pub async fn set_token_email_in_redis(mut con: Connection, token: &str, email: &str, expiry_in_seconds: &Option<i64>) -> RedisResult<Vec<usize>> {
    let expiry: usize = match expiry_in_seconds {
        Some(expiry_in_seconds) => *expiry_in_seconds as usize,
        None => 0
    };

    let opts: SetOptions = SetOptions::default()
        .conditional_set(ExistenceCheck::NX)
        .get(true)
        .with_expiration(SetExpiry::EX(expiry));
    con.set_options(token, email, opts)
}


pub fn get_user_from_token_in_redis(mut con: Connection, token: &str) -> Result<User, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let user_json: String = match redis_result {
        Ok(user_json) => user_json,
        Err(err) => return Err(err.to_string())
    };
    let user: User = serde_json::from_str(&user_json).unwrap();
    return Ok(user)
}


pub async fn set_token_tokenObject_in_redis(mut con: Connection, token: &str, token_object_json: &str, expiry_in_seconds: &Option<i64>) -> RedisResult<Vec<usize>> {
    let expiry: usize = match expiry_in_seconds {
        Some(expiry_in_seconds) => *expiry_in_seconds as usize,
        None => 0
    };

    let opts: SetOptions = SetOptions::default()
        .conditional_set(ExistenceCheck::NX)
        .get(true)
        .with_expiration(SetExpiry::EX(expiry));
    con.set_options(token, token_object_json, opts)
}


pub async fn delete_token_in_redis(mut con: Connection, token: &str) -> RedisResult<()> {
    con.del(token)
}

pub fn get_user_remember_me_from_token_in_redis(mut con: Connection, token: &str) -> Result<UserRememberMe, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let user_remember_me_json: String = match redis_result {
        Ok(user_remember_me_json) => user_remember_me_json,
        Err(err) => return Err(err.to_string())
    };
    let user_remember_me: UserRememberMe = serde_json::from_str(&user_remember_me_json).unwrap();
    return Ok(user_remember_me)
}

pub fn get_email_from_token_struct_in_redis(mut con: Connection, token_struct: &str) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token_struct);
    let email: String = match redis_result {
        Ok(email) => email,
        Err(err) => return Err(err.to_string())
    };
    return Ok(email)
}


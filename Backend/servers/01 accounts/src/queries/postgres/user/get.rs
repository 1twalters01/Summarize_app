use crate::models::{
    totp::{Totp, TotpFields},
    user::User,
};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

pub async fn uuid_from_username(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let uuid_select_query = sqlx::query("Select uuid from users WHERE username=($1)")
        .bind(username)
        .fetch_all(pool)
        .await;

    match uuid_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let uuid: Option<Uuid> = res[0].get("uuid");
            return Ok(uuid);
        }
    }
}

pub async fn uuid_from_email(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let uuid_select_query = sqlx::query("Select uuid from users WHERE email=($1)")
        .bind(email)
        .fetch_all(pool)
        .await;

    match uuid_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let uuid: Option<Uuid> = res[0].get("uuid");
            return Ok(uuid);
        }
    }
}

pub async fn password_hash_from_uuid(
    pool: &Pool<Postgres>,
    uuid: &Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let uuid_select_query = sqlx::query(
        "
        SELECT ph.password_hash
        FROM password_history ph
        INNER Join users u
        ON ph.user_id = u.id
        WHERE u.uuid=($1)
        ORDER BY ph.created_at DESC
        LIMIT 1
    ",
    )
    .bind(uuid)
    .fetch_one(pool)
    .await;

    match uuid_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let password_hash: Option<String> = res.get("password_hash");
            return Ok(password_hash);
        }
    }
}

pub async fn totp_activation_status_from_uuid(
    pool: &Pool<Postgres>,
    uuid: &Uuid,
) -> Result<bool, sqlx::Error> {
    let uuid_select_query = sqlx::query(
        "
        SELECT ts.is_activated
        FROM totp_secrets ts
        INNER Join users u
        ON ts.user_id = u.id
        WHERE u.uuid=($1)
    ",
    )
    .bind(uuid)
    .fetch_one(pool)
    .await;

    match uuid_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(false);
            }

            let totp_activation_status: bool = res.get("is_activated");
            return Ok(totp_activation_status);
        }
    }
}

pub async fn totp_struct_from_uuid(
    pool: &Pool<Postgres>,
    uuid: &Uuid,
) -> Result<Option<Totp>, sqlx::Error> {
    let uuid_select_query = sqlx::query(
        "
        SELECT ts.encrypted_totp_key, last_updated, ts.is_activated, ts.is_verified, ts.verified_at
        FROM totp_secrets ts
        INNER Join users u
        ON ts.user_id = u.id
        WHERE u.uuid=($1)
    ",
    )
    .bind(uuid)
    .fetch_one(pool)
    .await;

    match uuid_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let totp_fields: Option<TotpFields>;
            let encrypted_totp_key: Option<String> = res.get("encrypted_totp_key");
            let last_updated: Option<DateTime<Utc>> = res.get("last_updated");
            if let Some(encrypted_key) = encrypted_totp_key {
                if let Some(last_updated) = last_updated {
                    totp_fields = Some(TotpFields {
                        key: encrypted_key,
                        last_updated: last_updated,
                    });
                } else {
                    totp_fields = None
                }
            } else {
                totp_fields = None
            }

            let totp: Totp;
            let is_activated: Option<bool> = res.get("is_activated");
            let is_verified: Option<bool> = res.get("is_verified");
            let verified_at: Option<DateTime<Utc>> = res.get("verified_at");
            if let Some(is_activated) = is_activated {
                if let Some(is_verified) = is_verified {
                    totp = Totp::from_all(is_activated, is_verified, verified_at, totp_fields);
                    return Ok(Some(totp));
                }
            }
            return Ok(None);
        }
    }
}

pub async fn from_email(pool: &Pool<Postgres>, email: &str) -> Result<Option<User>, sqlx::Error> {
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

pub async fn from_uuid(pool: &Pool<Postgres>, uuid: &Uuid) -> Result<Option<User>, sqlx::Error> {
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

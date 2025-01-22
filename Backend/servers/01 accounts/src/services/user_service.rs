use crate::{
    generated::protos::settings::profile::theme::request::request::RequestField,
    models::{password::Password, totp::Totp, user::User},
    queries::postgres::{password_hash, theme, user},
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct UserService {
    pool: Pool<Postgres>,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn save_new_guest(&self) -> Result<Uuid, sqlx::Error> {
        user::insert::new_guest(&self.pool).await
    }

    pub async fn save_new_user(
        &self,
        username: &str,
        email: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let password_struct = Password::from_password(password).unwrap();
        let password_hash = password_struct.get_password_hash_str();
        user::insert::from_all(
            &self.pool,
            username,
            email,
            first_name,
            last_name,
            password_hash,
        )
        .await
    }

    pub async fn get_user_uuid_from_email(&self, email: &str) -> Result<Option<Uuid>, sqlx::Error> {
        user::get::uuid_from_email(&self.pool, &email).await
    }

    pub async fn get_user_uuid_from_username(
        &self,
        username: &str,
    ) -> Result<Option<Uuid>, sqlx::Error> {
        user::get::uuid_from_username(&self.pool, &username).await
    }

    pub async fn get_password_from_uuid(
        &self,
        uuid: &Uuid,
    ) -> Result<Option<Password>, sqlx::Error> {
        let password_hash = user::get::password_hash_from_uuid(&self.pool, uuid).await;
        match password_hash {
            Ok(Some(password_hash)) => {
                return Ok(Some(Password::from_hash(password_hash).unwrap()))
            }
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        }
    }

    pub async fn get_totp_key_from_uuid(&self, uuid: &Uuid) -> Result<Option<String>, sqlx::Error> {
        let totp_result = user::get::totp_from_uuid(&self.pool, uuid).await;
        match totp_result {
            Ok(Some(totp)) => match totp.get_url() {
                Some(key) => return Ok(Some(key)),
                None => return Ok(None),
            },
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        }
    }

    pub async fn update_password_hash_for_uuid(
        &self,
        password: &str,
        uuid: &Uuid,
    ) -> Result<(), sqlx::Error> {
        let password_struct = Password::from_password(password).unwrap();
        let password_hash = password_struct.get_password_hash_str();
        password_hash::update::from_user_uuid(&self.pool, uuid, password_hash).await
    }

    pub async fn update_email_for_uuid(&self, email: &str, uuid: &Uuid) -> Result<(), sqlx::Error> {
        user::update::update_email_from_uuid(&self.pool, email, uuid).await
    }

    pub async fn update_name_for_uuid(
        &self,
        first_name: Option<&String>,
        last_name: Option<&String>,
        uuid: &Uuid,
    ) -> Result<(), sqlx::Error> {
        user::update::update_name_from_uuid(&self.pool, first_name, last_name, uuid).await
    }

    pub async fn update_theme_for_uuid(
        &self,
        theme: RequestField,
        uuid: &Uuid,
    ) -> Result<(), sqlx::Error> {
        theme::update::update_theme_from_uuid(&self.pool, theme, uuid).await
    }

    pub async fn update_language_for_uuid(
        &self,
        language: &str,
        uuid: &Uuid,
    ) -> Result<(), sqlx::Error> {
        user::update::update_language_from_uuid(&self.pool, language, uuid).await
    }

    pub async fn get_totp_activation_status_from_uuid(
        &self,
        uuid: &Uuid,
    ) -> Result<bool, sqlx::Error> {
        user::get::totp_activation_status_from_uuid(&self.pool, uuid).await
    }

    pub async fn get_totp_from_uuid(&self, uuid: &Uuid) -> Result<Option<Totp>, sqlx::Error> {
        let totp = user::get::totp_from_uuid(&self.pool, uuid).await;
        return totp;
    }

    pub async fn set_totp_from_uuid(&self, totp: &Totp, uuid: &Uuid) -> Result<(), sqlx::Error> {
        user::update::totp_from_uuid(&self.pool, uuid, totp).await
    }

    pub async fn get_user_from_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        user::get::from_email(&self.pool, &email).await
    }

    pub async fn delete_user_from_uuid(&self, uuid: &Uuid) -> Result<(), sqlx::Error> {
        user::delete::from_uuid(&self.pool, uuid).await
    }

    pub async fn delete_totp_from_uuid(&self, uuid: &Uuid) -> Result<(), sqlx::Error> {
        user::delete::totp_from_uuid(&self.pool, uuid).await
    }
}

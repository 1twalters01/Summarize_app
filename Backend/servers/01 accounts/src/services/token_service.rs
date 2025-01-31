use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{env, net::SocketAddr};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{
    datatypes::claims::{CaptchaClaims, UserClaims},
    queries::postgres::refresh_token,
    utils::database_connections::create_pg_pool_connection,
};

pub struct TokenService<'a> {
    user_uuid: Option<&'a Uuid>,
}

impl<'a> TokenService<'a> {
    pub fn new() -> Self {
        Self { user_uuid: None }
    }

    pub fn from_uuid(user_uuid: &'a Uuid) -> Self {
        Self {
            user_uuid: Some(user_uuid),
        }
    }

    pub fn generate_opaque_token_of_length(&self, length: i64) -> String {
        let mut rng = thread_rng();
        let bytes: Vec<u8> = (0..length).map(|_| rng.sample(Alphanumeric)).collect();
        return String::from_utf8(bytes).unwrap();
    }

    pub fn generate_captcha_token(&self, ip: SocketAddr) -> Result<String, String> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::minutes(30))
            .unwrap()
            .timestamp() as usize;

        let claims = CaptchaClaims {
            ip,
            exp: expiration,
        };
        let secret = env::var("JWT_SECRET").unwrap();

        let captcha_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();
        return Ok(captcha_token);
    }

    pub fn generate_access_token(&self) -> Result<String, String> {
        if self.user_uuid == None {
            return Err(String::from("User UUID is None"));
        }

        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::minutes(30))
            .unwrap()
            .timestamp() as usize;

        let claims = UserClaims {
            sub: self.user_uuid.unwrap().to_string(),
            exp: expiration,
        };
        let secret = env::var("JWT_SECRET").unwrap();

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();
        return Ok(access_token);
    }

    pub fn generate_refresh_token(&self) -> String {
        self.generate_opaque_token_of_length(32)
    }

    pub async fn save_refresh_token_to_postgres(
        &self,
        refresh_token: &str,
        remember_me: bool,
    ) -> Result<(), String> {
        if self.user_uuid == None {
            return Err(String::from("User UUID is None"));
        }

        let pool = create_pg_pool_connection().await;
        let postgres_result = refresh_token::insert::from_user_uuid_and_refresh_token(
            &pool,
            self.user_uuid.unwrap(),
            refresh_token,
            remember_me,
        )
        .await;

        match postgres_result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err.to_string()),
        };
    }

    pub fn get_claims_from_captcha_token(captcha_token: &str) -> Result<CaptchaClaims, String> {
        let secret = env::var("JWT_SECRET").unwrap();
        let validation = Validation::default();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        match decode::<CaptchaClaims>(captcha_token, &decoding_key, &validation) {
            Ok(token_data) => return Ok(token_data.claims),
            Err(err) => return Err(err.to_string()),
        }
    }

    pub fn get_claims_from_access_token(access_token: &str) -> Result<UserClaims, String> {
        let secret = env::var("JWT_SECRET").unwrap();
        let validation = Validation::default();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        if access_token.starts_with("Bearer ") {
            let token = &access_token[7..];
            match decode::<UserClaims>(token, &decoding_key, &validation) {
                Ok(token_data) => return Ok(token_data.claims),
                Err(err) => return Err(err.to_string()),
            }
        }

        return Err("Invalid access token".to_string());
    }

    pub fn get_claims_from_bearer_token(bearer_token: &str) -> Result<UserClaims, String> {
        // remove "bearer " from token
            // If does not have then return error

        // if bearer token starts with SITE_
        // else if bearer token starts with GOOGLE_
        // else if bearer token starts with APPLE_
        // else return Err("Invalid bearer token".to_string)

        // Need to get user_id as a string and token creation time
    }

    pub async fn get_user_uuid_from_refresh_token(
        refresh_token: &str,
    ) -> Result<Option<Uuid>, String> {
        let pool = create_pg_pool_connection().await;
        let user_uuid_result: Result<Option<Uuid>, sqlx::Error> =
            refresh_token::get::user_uuid_from_refresh_token(&pool, &refresh_token).await;

        match user_uuid_result {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use super::*;

    #[test]
    fn test_generate_access_token() {
        dotenv().ok();
        let uuid: Uuid = Uuid::new_v4();
        let token_service = TokenService::from_uuid(&uuid);
        let access_token = token_service.generate_access_token();
        println!("access token: {:#?}", access_token);
        // assert!(1 == 2); # An error allows you to print in a test
    }
}

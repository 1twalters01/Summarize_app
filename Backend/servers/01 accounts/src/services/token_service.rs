use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    queries::postgres::refresh_token,
    utils::{
        database_connections::create_pg_pool_connection, tokens::generate_opaque_token_of_length,
    },
};

pub struct TokenService;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl TokenService {
    pub fn generate_access_token(user_uuid: &Uuid) -> String {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::days(1))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
            sub: user_uuid.to_string(),
            exp: expiration,
        };
        let secret = env::var("JWT_SECRET").unwrap();

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();
        return access_token;
    }

    pub fn generate_refresh_token(remember_me: bool) -> Option<String> {
        match remember_me {
            true => return None,
            false => return Some(generate_opaque_token_of_length(32)),
        }
    }

    pub async fn save_refresh_token_to_postgres(
        user_uuid: &Uuid,
        refresh_token: &str,
    ) -> Result<(), String> {
        let pool = create_pg_pool_connection().await;
        let postgres_result = refresh_token::insert::from_user_uuid_and_refresh_token(
            &pool,
            &user_uuid,
            refresh_token,
        )
        .await;

        match postgres_result {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err.to_string()),
        };
    }

    pub fn get_claims_from_access_token(access_token: &str) -> Result<Claims, String> {
        let secret = env::var("JWT_SECRET").unwrap();
        let validation = Validation::default();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        if access_token.starts_with("Bearer ") {
            let token = &access_token[7..];
            match decode::<Claims>(token, &decoding_key, &validation) {
                Ok(token_data) => return Ok(token_data.claims),
                Err(err) => return Err(err.to_string()),
            }
        }

        return Err("Invalid access token".to_string());
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

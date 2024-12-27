use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::{
    datatypes::claims::Claims, queries::postgres::refresh_token, utils::database_connections::create_pg_pool_connection,
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

    pub fn generate_access_token(&self) -> Result<String, String> {
        if self.user_uuid == None {
            return Err(String::from("User UUID is None"));
        }

        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::days(1))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
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

    pub fn generate_refresh_token(&self, remember_me: bool) -> Option<String> {
        match remember_me {
            true => return None,
            false => return Some(self.generate_opaque_token_of_length(32)),
        }
    }

    pub async fn save_refresh_token_to_postgres(&self, refresh_token: &str) -> Result<(), String> {
        if self.user_uuid == None {
            return Err(String::from("User UUID is None"));
        }

        let pool = create_pg_pool_connection().await;
        let postgres_result = refresh_token::insert::from_user_uuid_and_refresh_token(
            &pool,
            self.user_uuid.unwrap(),
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

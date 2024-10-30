use std::env;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    queries::postgres::refresh_token::insert::from_user_and_refresh_token,
    models::user::User,
    accounts::schema::errors::AccountError,
    utils::{
        database_connections::create_pg_pool_connection, tokens::generate_opaque_token_of_length,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokens {
    pub refresh_token: Option<String>,
    pub access_token: AccessToken,
}

impl AuthTokens {
    pub async fn new(user: User, remember_me: bool) -> Result<AuthTokens, AccountError> {
        let access_token = AccessToken::new(&user);

        let refresh_token;
        if remember_me == true {
            // Generate refresh token
            refresh_token = Some(generate_opaque_token_of_length(32));

            // save refresh token
            let pool = create_pg_pool_connection().await;
            if let Err(err) = from_user_and_refresh_token(
                &pool,
                &user,
                &refresh_token.as_ref().unwrap(),
            )
            .await
            {
                let error: AccountError = AccountError {
                    is_error: true,
                    error_message: Some(err.to_string()),
                };
                return Err(error);
            };
        } else {
            refresh_token = None;
        }

        let tokens = AuthTokens {
            access_token,
            refresh_token,
        };

        return Ok(tokens);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
}

impl AccessToken {
    pub fn new(user: &User) -> Self {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::days(1))
            .unwrap()
            .timestamp() as usize;

        let claims = Claims {
            sub: user.get_uuid().to_string(),
            exp: expiration,
        };
        let secret = env::var("JWT_SECRET").unwrap();

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();
        return AccessToken { access_token };
    }

    pub fn to_string(self) -> String {
        return self.access_token;
    }
}

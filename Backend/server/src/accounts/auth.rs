use std::env;

use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Duration, Utc};
use crate::{accounts::datatypes::users::User, utils::tokens::generate_opaque_token_of_length};

use super::{db_queries::save_refresh_token_user_in_postgres_auth_table, schema::AccountError};
use crate::databases::connections::create_pg_pool_connection;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    exp: usize,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AuthTokens {
    pub refresh_token: Option<String>,
    pub access_token: String,
}

pub async fn generate_auth_tokens(user: User, remember_me: bool) -> Result<AuthTokens, AccountError> {
    let refresh_token;
    if remember_me == true {
        refresh_token = Some(generate_opaque_token_of_length(32));
    } else {
        refresh_token = None;
    }

    let access_token = generate_access_token(&user);
    
    // save refresh token
    let pool = create_pg_pool_connection().await; 
    if let Err(err) = save_refresh_token_user_in_postgres_auth_table(&pool, &refresh_token.clone().unwrap(), &user).await {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(err.to_string()),
        };
        return Err(error);
    };

    let tokens = AuthTokens {
        access_token,
        refresh_token,
    };

    return Ok(tokens);
}

pub fn generate_access_token(user: &User) -> String {
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

    let access_token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    return access_token;
}


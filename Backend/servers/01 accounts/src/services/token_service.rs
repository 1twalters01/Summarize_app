use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde_json::{Serialize, Deserialize};

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

    pub fn generate_refresh_token(remember_me: bool) -> String {
        match remember_me {
            true => return None,
            false => {
                refresh_token = Some(generate_opaque_token_of_length(32));
            }
        }
    }

    pub fn save_refresh_token_to_postgres(user_uuid: &Uuid) -> Result<(), String> {
        let pool = create_pg_pool_connection().await;
        let postgres_result = refresh_token::insert::from_user_uuid_and_refresh_token(
            &pool,
            &user_uuid,
            &refresh_token.as_ref().unwrap()
        ).await;

        match postgres_result {
            Ok() => return Ok(()),
            Err(error) => return Err(err.to_string()),
        };
    }

    pub fn get_claims_from_access_token(access_token: &str) -> Result<Claims, String> {
        let secret = env::var("JWT_SECRET").unwrap();
        let validation = Validation::default();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                    return Ok(token_data.claims);
                }
            }
        }

        return Err(());
    }

    pub fn get_user_uuid_from_refresh_token(refresh_token: &str) -> Result<Option<Uuid>, String>> {
        let pool = create_pg_pool_connection().await;
        let user_uuid_result: Result<Option<Uuid>, sqlx::Error> =
            refresh_token::get::user_uuid_from_refresh_token(
                &pool,
                &refresh_token.as_ref().unwrap()
            ).await;

        match user_uuid_result {
            Ok(res) => res,
            Err(err) => err.to_string(),
        }
    }
}
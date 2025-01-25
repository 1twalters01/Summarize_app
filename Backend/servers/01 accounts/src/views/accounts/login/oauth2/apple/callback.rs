use actix_web::{web, HttpResponse, Responder, Result};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{
    datatypes::oauth_types::ClientProvider;
    services::oauth_service::OAuthService,
};

async fn handle_oauth2_callback(query: web::Query<OAuthCallbackQuery>) -> impl Responder {
    dotenv()::ok();

    let client = OAuthService::new_basic_client(ClientProvider::Apple);

    let code = query.get("code").expect("Missing authorization code");

    // Get PKSE code verifier

    let http_client = reqwest::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let token_response = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request(&http_client);

    let (access_token, refresh_token, id_token, access_token_expiration) = match token_result {
        Ok(token) => {
            // Extract access and refresh tokens
            let access_token = token.access_token().secret();
            let refresh_token = token.refresh_token().map(|rt| rt.secret().to_string());
            let id_token = token.id_token().map(|t| t.secret());
            let access_token_expiration = token.expires_in().unwrap_or_default().as_secs();
            (access_token, refresh_token, id_token, access_token_expiration)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }

    let user_info = decode_apple_id_token(id_token).unwrap();

    // Check if sub (oauth_provider_user_id) is associated to oauth_provider_id for apple
    // if yes then:
        // update refresh token to postgres oauth_refresh_token table
    // if no then:
        // Check if email is associated to anything
        // if yes then:
            // get uuid associated to the email
            // save oauth to the uuid with provider as google to the oauth_table
        // if no then:
            // create new user
            // save first, last name and email
        // save refresh token to postgres oauth_refresh_token table

    // generate opaque token with prefix GOOGLE_
    // save: con.set_ex(format!("session:{}", opaque_token), access_token, expiration as usize)

    // return opaque token to user
    // if wasn't associated then
}

#[derive(Debug, Serialize, Deserialize)]
struct AppleIdTokenClaims {
    email: String,
    email_verified: String,
    sub: String,
    given_name: Option<String>, // Only provided on the first login
    family_name: Option<String>, // Only provided on the first login
}

fn decode_apple_id_token(id_token: &str) -> Result<AppleIdTokenClaims, Box<dyn Error>> {
    let decoding_key = DecodingKey::from_rsa_pem(include_bytes!("path_to_apple_public_key.pem"))?;
    let token = decode::<AppleIdTokenClaims>(
        id_token,
        &decoding_key,
        &Validation::new(Algorithm::RS256),
    )?;

    Ok(token.claims)
}

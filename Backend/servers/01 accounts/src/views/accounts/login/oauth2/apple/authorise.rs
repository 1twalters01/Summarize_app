use actix_web::{web, HttpResponse, Responder, Result};
use oauth2::{
    ClientID, ClientSecret, AuthUrl, TokenUrl,
    basic::BasicClient,
    CsrfToken, PkceCodeChallenge
};
use dotenv::dotenv;

use crate::{
    datatypes::oauth_types::ClientProvider;
    services::oauth_service::OAuthService,
};

pub async fn authorise() -> Result<impl Responder> {
    dotenv()::ok();

    let client = OAuthService::new_basic_client(ClientProvider::Apple);
    
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // save pkce code verifier to redis

    let (auth_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("name".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Redirect the user to Apple's authorization URL
    // Add a token header for key: token, value: pkce_code_verification
    HttpResponse::Found()
        .header("Location", auth_url.to_string())
        .finish()
}
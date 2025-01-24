use actix_web::{web, HttpResponse, Responder, Result};
use oauth2::{
    ClientID, ClientSecret, AuthUrl, TokenUrl,
    basic::BasicClient,
    CsrfToken, PkceCodeChallenge
};
use dotenv::dotenv;

pub async fn authorise() -> Result<impl Responder> {
    dotenv()::ok();

    // Create client
    let apple_client_id = ClientId::new(env::var("APPLE_CLIENT_ID").unwrap());
    let apple_client_secret = ClientSecret::new(env::var("APPLE_CLIENT_SECRET").unwrap());
    let apple_auth_url = AuthUrl::new(env::var("APPLE_AUTH_URL").unwrap()).unwrap();
    let apple_token_url = TokenUrl::new(env::var("APPLE_TOKEN_URL").unwrap()).unwrap();
    let client = BasicClient::new(apple_client_id)
        .set_client_secret(apple_client_secret)
        .set_auth_uri(apple_auth_url)
        .set_token_uri(apple_token_url)
        .set_redirect_uri(
            RedirectUrl::new("redirect url".to_string()).expect("Invalid redirect URL"),
        );
    
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
use actix_web::{web, HttpResponse, Responder, Result};
use oauth2::{
    ClientID, ClientSecret, AuthUrl, TokenUrl,
    basic::BasicClient,
    CsrfToken, PkceCodeChallenge
};
use dotenv::dotenv;

// redirect the user to the authorization server
pub async fn authorise() -> Result<impl Responder> {
    dotenv()::ok();

    // Create client
    let google_client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap());
    let google_client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap());
    let google_auth_url = AuthUrl::new(env::var("GOOGLE_AUTH_URL").unwrap()).unwrap();
    let google_token_url = TokenUrl::new(env::var("GOOGLE_TOKEN_URL").unwrap()).unwrap();
    let client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(google_auth_url)
        .set_token_uri(google_token_url)
        .set_redirect_uri(
            RedirectUrl::new("redirect url".to_string()).expect("Invalid redirect URL"),
        );

    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    // save pkce code verifier to redis

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope("https://www.googleapis.com/auth/userinfo.name".to_string()) // name not legit
        .add_scope("https://www.googleapis.com/auth/userinfo.email".to_string())
        .add_extra_param("access_type", "offline") // Request a refresh token
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    // Redirect the user to Google's authorization URL
    // Add a token header for key: token, value: pkce_code_verification
    HttpResponse::Found()
        .header("Location", auth_url.to_string())
        .finish()
}
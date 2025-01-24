use actix_web::{web, HttpResponse, Responder, Result};
use chrono::Utc;

pub async fn refresh_token(data: web::Json<RefreshTokenQuery>) -> Result<impl Responder> {
    dotenv()::ok();

    // Get refresh token from user_id
        // If no refresh token return error

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
    
    let http_client = reqwest::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request(http_client);

    let (access_token, refresh_token, access_token_expiration) = match token_result {
        Ok(token) => {
            // Extract access and refresh tokens
            let access_token = token.access_token().secret().clone();
            let refresh_token = token.refresh_token().map(|t| t.secret().to_string());
            let access_token_expiration = token.expires_in().unwrap_or_default().as_secs();
            (access_token, refresh_token, access_token_expiration)
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
    
    // save refresh token to postgres oauth_refresh_token table

    // generate opaque token with prefix GOOGLE_
    let now = Utc::now();
    // save: con.set_ex(format!("session:{}", (opaque_token, now)), access_token, expiration as usize)
    
    // return opaque token to user
}

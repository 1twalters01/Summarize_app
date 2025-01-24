use actix_web::{web, HttpResponse, Responder, Result};
use dotenv::dotenv;

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

    let (access_token, refresh_token, access_token_expiration) = match token_result {
        Ok(token) => {
            // Extract access and refresh tokens
            let access_token = token.access_token().secret();
            let refresh_token = token.refresh_token().map(|rt| rt.secret().to_string());
            let access_token_expiration = token.expires_in().unwrap_or_default().as_secs();
            (access_token, refresh_token, access_token_expiration)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }

    // save refresh token to postgres oauth_refresh_token table

    // generate opaque token with prefix APPLE_
    // save: con.set_ex(format!("session:{}", opaque_token), access_token, expiration as usize)

    // return opaque token to user
}

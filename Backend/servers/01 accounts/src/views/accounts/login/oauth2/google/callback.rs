use actix_web::{web, HttpResponse, Responder, Result};

// exchanges the authorization code for tokens, and returns the tokens
pub async fn callback(query: web::Query<CallbackQuery>) -> Result<impl Responder> {
    dotenv()::ok();

    let code = query.get("code").expect("Missing authorization code");

    // Create client
    let google_client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID").unwrap());
    let google_client_secret = ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET").unwrap());
    let auth_url = AuthUrl::new(env::var("AUTH_URL").unwrap()).unwrap();
    let token_url = TokenUrl::new(env::var("TOKEN_URL").unwrap()).unwrap();
    let client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(
            RedirectUrl::new("redirect url".to_string()).expect("Invalid redirect URL"),
        );

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

    let (access_token, refresh_token) = match token_result {
        Ok(token) => {
            // Extract access and refresh tokens
            let access_token = token.access_token().secret();
            let refresh_token = token.refresh_token().map(|t| t.secret().to_string());
            (access_token, refresh_token)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }

    // Save refresh token

    // generate opaque token and save key: opaque, value: access
    // return opaque token to user

}
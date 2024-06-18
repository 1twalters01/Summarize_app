use crate::{
    accounts::schema::oauth2::{CallbackQuery, OAuth2Client, RefreshTokenQuery},
    utils::tokens::generate_opaque_token_of_length,
};
use actix_web::{web, HttpResponse, Responder, Result};

// redirect the user to the authorization server
pub async fn authorise() -> Result<impl Responder> {
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    let scope = "https://www.googleapis.com/auth/userinfo.email";
    let state = generate_opaque_token_of_length(32);
    let authorization_url = oauth_client.get_authorization_url(&state, scope);

    Ok(HttpResponse::Found()
        .append_header(("Location", authorization_url))
        .finish())
}

// exchanges the authorization code for tokens, and returns the tokens
pub async fn callback(query: web::Query<CallbackQuery>) -> Result<impl Responder> {
    // check that state is valid
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    match oauth_client.get_access_tokens_from_code(&query.code).await {
        Ok(token_response) => return Ok(HttpResponse::Ok().json(token_response)),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
}

pub async fn refresh_token(data: web::Json<RefreshTokenQuery>) -> Result<impl Responder> {
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    match oauth_client.refresh_access_token(&data.refresh_token).await {
        Ok(token_response) => return Ok(HttpResponse::Ok().json(token_response)),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
}

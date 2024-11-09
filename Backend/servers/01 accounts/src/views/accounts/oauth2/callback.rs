use crate::{
    accounts::schema::oauth2::{CallbackQuery, OAuth2Client, RefreshTokenQuery},
    utils::tokens::generate_opaque_token_of_length,
};
use actix_web::{web, HttpResponse, Responder, Result};

// exchanges the authorization code for tokens, and returns the tokens
pub async fn callback(query: web::Query<CallbackQuery>) -> Result<impl Responder> {
    // check that state is valid
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    match oauth_client.get_access_tokens_from_code(&query.code).await {
        Ok(token_response) => return Ok(HttpResponse::Ok().json(token_response)),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
}
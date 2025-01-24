use crate::{
    accounts::schema::oauth2::{CallbackQuery, OAuth2Client, RefreshTokenQuery},
    utils::tokens::generate_opaque_token_of_length,
};
use actix_web::{web, HttpResponse, Responder, Result};

pub async fn refresh_token(data: web::Json<RefreshTokenQuery>) -> Result<impl Responder> {
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    match oauth_client.refresh_access_token(&data.refresh_token).await {
        Ok(token_response) => return Ok(HttpResponse::Ok().json(token_response)),
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
}

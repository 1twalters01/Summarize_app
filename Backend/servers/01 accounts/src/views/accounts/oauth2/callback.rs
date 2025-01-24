use crate::{
    utils::tokens::generate_opaque_token_of_length,
};
use actix_web::{web, HttpResponse, Responder, Result};

// exchanges the authorization code for tokens, and returns the tokens
pub async fn callback(query: web::Query<CallbackQuery>) -> Result<impl Responder> {
    // Get PKSE code verifier
}
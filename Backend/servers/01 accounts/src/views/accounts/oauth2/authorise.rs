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
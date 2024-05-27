use actix_web::{post, web, HttpResponse, Responder, Result};
use crate::{
    accounts::oauth2::{CallbackQuery, OAuth2Client, RefreshTokenQuery},
    utils::tokens::generate_opaque_token_of_length
};

// redirect the user to the authorization server
#[post("/authorise")]
async fn authorise() -> Result<impl Responder> {
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    let scope = "https://www.googleapis.com/auth/userinfo.email";
    let state = generate_opaque_token_of_length(32);
    let authorization_url = oauth_client.get_authorization_url(&state, scope);

    Ok(HttpResponse::Found()
        .append_header(("Location", authorization_url))
        .finish()
    )
}

// exchanges the authorization code for tokens, and returns the tokens
#[post("/callback")]
async fn callback(query: web::Query<CallbackQuery>) -> Result<impl Responder> {
    // check that state is valid
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    match oauth_client.get_access_tokens_from_code(&query.code).await {
        Ok(token_response) => {
            return Ok(HttpResponse::Ok().json(token_response))
        },
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
}

#[post("/refresh")]
async fn refresh(data: web::Json<RefreshTokenQuery>) -> Result<impl Responder> {
    let oauth_client: OAuth2Client = OAuth2Client::new_google();
    match oauth_client.refresh_access_token(&data.refresh_token).await {
        Ok(token_response) => {
            return Ok(HttpResponse::Ok().json(token_response))
        },
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
}

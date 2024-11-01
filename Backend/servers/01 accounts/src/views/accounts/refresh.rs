use actix_web::{web::Json, HttpResponse, Responder, Result};
// use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};


use crate::{
    queries::postgres::user::get::from_refresh_token,
    models::user::User,
    accounts::{
        schema::{
            auth::{AccessToken, AuthTokens},
            errors::AccountError,
            refresh_token::RefreshTokenResponseSchema,
        },
    },
    utils::database_connections::create_pg_pool_connection,
};

pub async fn refresh_token(data: Json<AuthTokens>) -> Result<impl Responder> {
    let mut res_body: RefreshTokenResponseSchema = RefreshTokenResponseSchema::new();
    let refresh_token: String = match &data.refresh_token {
        None => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(String::from("Internal server error")),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Some(refresh_token) => refresh_token.to_string(),
    };

    let pool = create_pg_pool_connection().await;
    let user: User =
        match from_refresh_token(&pool, &refresh_token).await {
            Ok(user) => match user {
                Some(user) => user,
                None => {
                    let error: AccountError = AccountError {
                        is_error: true,
                        error_message: Some("invalid refresh token".to_string()),
                    };
                    res_body.account_error = error;
                    return Ok(HttpResponse::UnprocessableEntity()
                        .content_type("application/json; charset=utf-8")
                        .json(res_body));
                }
            },
            Err(err) => {
                let error: AccountError = AccountError {
                    is_error: true,
                    error_message: Some(err.to_string()),
                };
                res_body.account_error = error;
                return Ok(HttpResponse::UnprocessableEntity()
                    .content_type("application/json; charset=utf-8")
                    .json(res_body));
            }
        };

    let access_token = AccessToken::new(&user);

    let auth_tokens: AuthTokens = AuthTokens {
        refresh_token: Some(refresh_token),
        access_token,
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(auth_tokens))
}
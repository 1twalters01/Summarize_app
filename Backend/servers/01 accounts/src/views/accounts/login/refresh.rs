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

pub async fn post_refresh_token(req: HttpRequest) -> Result<impl Responder> {
    // Read refresh token from header if none then error
    let refresh_token: String = match req
        .headers()
        .get("Refresh-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string() {
            Some(refresh_token) => refresh_token,
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::LoginRefresh(Error::NoRefreshToken),
                    StatusCode::NOT_FOUND,
                ));
            },
        };

    // Validate refresh token
    if validate_refresh_token(&refresh_token).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginRefresh(Error::InvalidRefreshToken),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // try to Get user uuid from refresh token else fail

    // Create new access token for user
    let auth_tokens = AccessToken::new(&user);

    // Return token
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginTotp(Response {
            response_field: Some(ResponseField::Token(auth_tokens)),
        }),
        StatusCode::OK,
    ));
}





//     let pool = create_pg_pool_connection().await;
//     let user: User =
//         match from_refresh_token(&pool, &refresh_token).await {
//             Ok(user) => match user {
//                 Some(user) => user,
//                 None => {
//                     let error: AccountError = AccountError {
//                         is_error: true,
//                         error_message: Some("invalid refresh token".to_string()),
//                     };
//                     res_body.account_error = error;
//                     return Ok(HttpResponse::UnprocessableEntity()
//                         .content_type("application/json; charset=utf-8")
//                         .json(res_body));
//                 }
//             },
//             Err(err) => {
//                 let error: AccountError = AccountError {
//                     is_error: true,
//.                    error_message: Some(err.to_string()),
//                 };
//                 res_body.account_error = error;
//                 return Ok(HttpResponse::UnprocessableEntity()
//                     .content_type("application/json; charset=utf-8")
//                     .json(res_body));
//             }
//         };


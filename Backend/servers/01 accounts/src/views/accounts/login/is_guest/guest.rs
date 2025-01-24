use actix_web::{http::StatusCode, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::{
        auth_tokens::AuthTokens,
        login::guest::response::{response::ResponseField, Error, Response},
    },
    services::{
        response_service::ResponseService, token_service::TokenService, user_service::UserService,
    },
    utils::database_connections::create_pg_pool_connection,
};

pub async fn get_guest() -> Result<impl Responder> {
    // Create and save user
    let user_service = UserService::new(create_pg_pool_connection().await);
    let user_result = user_service.save_new_guest().await;
    let user_uuid = match user_result {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginGuest(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    // Create access and refresh tokens
    let token_service = TokenService::from_uuid(&user_uuid);
    let refresh_token = token_service.generate_refresh_token();
    let access_token = token_service.generate_access_token().unwrap();

    // save refresh token in not remembering state
    let save_result = token_service
        .save_refresh_token_to_postgres(&refresh_token, false)
        .await;
    if save_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginGuest(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }


    // generate opaque token with prefix SITE_
    // save: con.set_ex(format!("session:{}", opaque_token), access_token, expiration as usize)
    
    // return opaque token to user
    let auth_tokens = AuthTokens {
        refresh: refresh_token,
        access: access_token,
    };
    println!("auth tokens: {:#?}", auth_tokens);
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginGuest(Response {
            response_field: Some(ResponseField::Tokens(auth_tokens)),
        }),
        StatusCode::OK,
    ));
}

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    models::user::User,
    generated::protos::accounts::{
        auth_tokens::AuthTokens,
        login::sms::{
            request,
            response::{response::ResponseField, Error, Response},
        },
    },
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{database_connections::create_redis_client_connection, validations::validate_totp},
}

pub async fn post_sms(
    data: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let login_password_token: String = req
        .headers()
        .get("Login-Password-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    // Try to get user and remember_me status from redis
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_and_remember_me_from_token(&login_password_token);
    let (mut user, remember_me): (User, bool) = match cache_result {
        Ok(Some((user, remember_me))) => {
            (user, remember_me)
        },
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginTotp(Error::UserNotFound),
                StatusCode::NOT_FOUND,
            ));
        },
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginTotp(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        },
    };

    // Get sms response from request

    // check if the entered sms response is valid

    // check if sms response is correct
    
    // update last login time

    // delete old token
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.delete_key(&login_password_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginSms(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return success
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginSms(Response {
            response_field: Some(ResponseField::Tokens(auth_tokens)),
        }),
        StatusCode::OK,
    ));
}

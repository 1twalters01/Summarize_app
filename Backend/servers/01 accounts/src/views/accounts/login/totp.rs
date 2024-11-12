use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::{
        auth_tokens::AuthTokens,
        login::totp::{
            request,
            response::{response::ResponseField, Error, Response},
        },
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{database_connections::create_redis_client_connection, validations::validate_totp},
};

pub async fn post_totp(
    // data: Json<LoginTotpRequest>,
    data: ProtoBuf<request::Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let login_password_token: String = req
        .headers()
        .get("Login-Password-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let request::Request {
        digit1,
        digit2,
        digit3,
        digit4,
        digit5,
        digit6,
    } = data.0;

    // Try to get user and remember_me from redis
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_and_remember_me_from_token(&login_password_token);
    let (mut user, remember_me): (User, bool) = match cache_result {
        Ok(Some(user_and_remember_me)) => {
            (user_and_remember_me.user, user_and_remember_me.remember_me)
        }
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginTotp(Error::UserNotFound),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginTotp(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    // check if the entered totp is a valid totp
    if validate_totp(digit1, digit2, digit3, digit4, digit5, digit6).is_err()
        || user.is_totp_activated() == false
    {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::InvalidTotp),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // check totp
    if user.check_totp(digit1, digit2, digit3, digit4, digit5, digit6) == false {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::IncorrectTotp),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // update last login time

    // create auth tokens
    let refresh_token = TokenService::generate_refresh_token(remember_me);
    let access_token = TokenService::generate_access_token(&user.get_uuid());

    let auth_tokens = AuthTokens {
        refresh: refresh_token,
        access: access_token,
    };
    println!("auth tokens: {:#?}", auth_tokens);

    // delete old token
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.delete_key(&login_password_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return success
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginTotp(Response {
            response_field: Some(ResponseField::Tokens(auth_tokens)),
        }),
        StatusCode::OK,
    ));
}

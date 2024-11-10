use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};

use crate::{
    datatypes::auth::AuthTokens,
    generated::protos::accounts::{
        auth_tokens,
        login::totp::{
            request,
            response::{Error, Response, response::ResponseField},
        },
    },
    models::user::User,
    queries::redis::{all::get_user_remember_me_from_token_in_redis, general::delete_key_in_redis},
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
        Ok(Some(user_and_remember_me)) => (user_remember_me.user, user_remember_me.remember_me),
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
    if validate_totp(digit1, digit2, digit3, digit4, digit5, digit6).is_err() || user.is_totp_activated() == false {
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
    let auth_tokens: auth_tokens::AuthTokens = match AuthTokens::new(user, remember_me).await {
        Ok(tokens) => auth_tokens::AuthTokens {
            refresh: tokens.refresh_token,
            access: tokens.access_token.to_string(),
        },
        Err(err) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginTotp(Error::ServerError),
                StatusCode::FAILED_DEPENDENCY,
            ));
        }
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

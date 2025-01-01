use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use uuid::Uuid;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::password_reset::password::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_password,
    },
};

pub async fn post_password_reset(
    data: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    // Check if ip has verified captcha
    
    let verification_confirmation_token: String = req
        .headers()
        .get("Password-Reset-Verification-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    println!("verification token: {:?}", verification_confirmation_token);

    // get user from token in redis
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_uuid_from_token(&verification_confirmation_token);
    let user_uuid: Uuid = match cache_result {
        Ok(Some(user_uuid)) => user_uuid,
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetPassword(Error::InvalidCredentials),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetPassword(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    let Request {
        password,
        password_confirmation,
    } = data.0;

    if password != password_confirmation {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetPassword(Error::IncorrectPasswordConfirmation),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if validate_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetPassword(Error::InvalidPassword),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // if change is not allowed then error
    let user_service = UserService::new(create_pg_pool_connection().await);
    let user_result = user_service
        .update_password_for_uuid(&password, &user_uuid)
        .await;
    if user_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetPassword(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // delete used token
    let cache_result = cache_service.delete_key(&verification_confirmation_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetPassword(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::PasswordResetPassword(Response {
            response_field: Some(ResponseField::Success(Success {})),
        }),
        StatusCode::OK,
    ));
}

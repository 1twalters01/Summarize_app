use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::password_reset::email::{
        request,
        response::{response::ResponseField, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService,
        token_service::TokenService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        email::{compose::compose_password_reset_email_message, handler::send_email},
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<request::Request>) -> Result<impl Responder> {
    let request::Request { email } = data.0;

    if validate_email(&email).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetEmail(Error::InvalidEmail),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }
    println!("email: {:?}", email);

    // Get user from database
    let user_service = UserService::new(create_pg_pool_connection().await);
    let user: User = match user_service.get_user_from_email(&email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("No user found");
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetEmail(Error::UnregisteredEmail),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetEmail(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    // create a verify token, and a register email token
    let token_service = TokenService::new();
    let verification_token = token_service.generate_opaque_token_of_length(8);
    let header_token = token_service.generate_opaque_token_of_length(64);

    // try to email the account a message containing the token
    let message = compose_password_reset_email_message(&verification_token, &header_token);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetEmail(Error::EmailFailedToSend),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // add {key: token, value: UUID} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let token_tuple: (&str, &str) = (&header_token, &verification_token);
    let token_tuple_json: String = serde_json::to_string(&token_tuple).unwrap();

    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result =
        cache_service.store_user_for_token(&user, &token_tuple_json, expiry_in_seconds);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetEmail(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::PasswordResetEmail(Response {
            response_field: Some(ResponseField::Token(header_token)),
        }),
        StatusCode::OK,
    ));
}

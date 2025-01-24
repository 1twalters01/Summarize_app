use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

use crate::{
    datatypes::{
        email_types::{MessageType::RegisterEmail, RegisterEmailParams},
        response_types::{AppError, AppResponse},
    },
    generated::protos::accounts::register::email::{
        request::Request,
        response::{response::ResponseField, Error, Response},
    },
    services::{
        cache_service::CacheService, email_service::EmailService,
        response_service::ResponseService, token_service::TokenService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::email::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<Request>) -> Result<impl Responder> {
    let Request { email } = data.0;
    if validate_email(&email).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterEmail(Error::InvalidEmail),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // get uuid
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeEmail(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    // check that email is associated with account else error
    if is_associated(&user_uuid, &email) == false {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeEmail(Error::InvalidCredentials),
            StatusCode::NOT_FOUND,
        ));
    }

    // Check if email is in use for regular account
    let user_service = UserService::new(create_pg_pool_connection().await);
    let _ = match user_service.get_user_from_email(&email).await {
        Ok(Some(_)) => {
            println!("Email is in use");
            return Ok(ResponseService::create_error_response(
                AppError::RegisterEmail(Error::RegisteredEmail),
                StatusCode::CONFLICT,
            ));
        }
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::RegisterEmail(Error::InvalidEmail),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        Ok(None) => (),
    };

    // create a verification token and a register email token
    let token_service = TokenService::new();
    let verification_token = token_service.generate_opaque_token_of_length(8);
    let header_token = token_service.generate_opaque_token_of_length(64);
    let token_tuple: (&str, &str) = (&header_token, &verification_token);

    // try to email the account a message containing the token
    let mut email_service = EmailService::new(&email);
    let message_type = RegisterEmail(RegisterEmailParams {
        verification_token: &verification_token,
        register_email_token: &header_token,
    });
    email_service.compose_preformatted_message(message_type);
    let message_result = email_service.send_email();

    // if unable to email then return an error
    if message_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterEmail(Error::EmailFailedToSend),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let token_tuple_json: String = serde_json::to_string(&token_tuple).unwrap();
    let expiry_in_seconds: Option<i64> = Some(300);

    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.store_key_value(&token_tuple_json, &email, expiry_in_seconds);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterEmail(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::RegisterEmail(Response {
            response_field: Some(ResponseField::Token(header_token)),
        }),
        StatusCode::OK,
    ));
}

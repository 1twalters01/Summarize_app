use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

use crate::{
    datatypes::{
        email_types::{MessageType::RegisterConfirmation, RegisterConfirmationParams},
        response_types::{AppError, AppResponse},
    },
    generated::protos::accounts::register::details::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    services::{
        cache_service::CacheService, email_service::EmailService,
        response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::{
            name::validate_name, password::validate_password, username::validate_username,
        },
    },
};

pub async fn post_details(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let verification_confirmation_token: String = req
        .headers()
        .get("Register-Verification-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // get the email from redis using the token
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_email_from_token(&verification_confirmation_token);
    let email: String = match cache_result {
        // if error return error
        Err(err) => {
            println!("error: {:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::RegisterDetails(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        Ok(email) => email,
    };

    // get the uuid from redis using the token
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_uuid_from_token(&verification_confirmation_token);
    let uuid: Uuid = match cache_result {
        // if error return error
        Err(err) => {
            println!("error: {:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::RegisterDetails(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        Ok(email) => email,
    };

    let Request {
        password,
        password_confirmation,
    } = data.0;

    if password != password_confirmation {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::IncorrectPasswordConfirmation),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    if validate_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::InvalidPassword),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // check that email is associated with account else error
    if is_associated(&user_uuid, &email) == false {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeEmail(Error::InvalidCredentials),
            StatusCode::NOT_FOUND,
        ));
    }

    let user_service = UserService::new(create_pg_pool_connection().await);
    let user_result = user_service
        .new_from_oauth_user(
            &uuid,
            &email,
            &password,
        )
        .await;
    if user_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let mut email_service = EmailService::new(&email);
    let message_type = RegisterConfirmation(RegisterConfirmationParams {});
    email_service.compose_preformatted_message(message_type);
    let message_result = email_service.send_email();

    // if unable to email then return an error
    if message_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::FromOauthRegisterDetails(Error::EmailFailedToSend),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // delete old {key: token, value: email}
    let cache_result = cache_service.delete_key(&verification_confirmation_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::RegisterDetails(Response {
            response_field: Some(ResponseField::Success(Success {})),
        }),
        StatusCode::OK,
    ));
}
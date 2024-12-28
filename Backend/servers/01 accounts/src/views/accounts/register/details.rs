use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::register::details::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::{
            validate_first_name, validate_last_name, validate_password, validate_username,
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

    let Request {
        username,
        password,
        password_confirmation,
        first_name,
        last_name,
    } = data.0;

    if validate_username(&username).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::InvalidUsername),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

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

    if let Some(ref fname) = first_name {
        if validate_first_name(&fname).is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::RegisterDetails(Error::InvalidFirstName),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }

    if last_name.is_some() {
        if validate_last_name(last_name.clone().unwrap()).is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::RegisterDetails(Error::InvalidLastName),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }

    let user_service = UserService::new(create_pg_pool_connection().await);
    let user_result = user_service
        .save_new_user(
            &username,
            &email,
            first_name.as_ref().map(|s| s.as_str()),
            last_name.as_ref().map(|s| s.as_str()),
            &password,
        )
        .await;
    if user_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterDetails(Error::ServerError),
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

    // set created time

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::RegisterDetails(Response {
            response_field: Some(ResponseField::Success(Success {})),
        }),
        StatusCode::OK,
    ));
}

#[cfg(test)]
mod tests {
    // use actix_web::{test, web, App};
    // use dotenv::dotenv;

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_without_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_token_with_header_token_username_password_without_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_without_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_with_header_token_username_password_confirmation_first_without_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_being_authenticated_without_header_token_username_password_confirmation_with_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_without_username_password_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_token_with_header_token_username_password_without_confirmation_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_without_first_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_with_header_token_username_password_confirmation_first_without_last(
    ) {
    }

    #[actix_web::test]
    async fn test_post_details_while_not_being_authenticated_without_header_token_username_password_confirmation_with_first_last(
    ) {
    }
}

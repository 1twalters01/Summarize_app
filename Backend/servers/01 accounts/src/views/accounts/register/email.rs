use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::register::email::{
        request::Request,
        response::{response::ResponseField, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        email::{compose::compose_register_email_message, handler::send_email},
        tokens::generate_opaque_token_of_length,
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<Request>) -> Result<impl Responder> {
    // Get email from posted data
    let request::Request { email } = data.0;

    // Validate email
    if validate_email(&email).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterEmail(Error::InvalidEmail),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Ensure that no user exists for email
    let user_service = UserService::new(create_pg_pool_connection().await);
    let user: User = match user_service.get_user_from_email(&email).await {
        Ok(Some(user)) => {
            println!("No user found");
            return Ok(ResponseService::create_error_response(
                AppError::RegisterEmail(Error::UnregisteredEmail),
                StatusCode::NOT_FOUND,
            ));
        },
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::RegisterEmail(Error::RegisteredEmail),
                StatusCode::CONFLICT,
            ));
        },
        Ok(None) => (),
    };

    // create a verify token, a register email token, and a register_email_token_tuple
    let verification_token = generate_opaque_token_of_length(8);
    let header_token = generate_opaque_token_of_length(64);
    let token_tuple: (&str, &str) = (&header_token, &verification_token);

    // try to email the account a message containing the token
    let message = compose_register_email_message(&verification_token, &header_token);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterEmail(Error::EmailFailedToSend),
            StatusCode::INTERNAL_SERVICE_ERROR,
        ));
    }

    // save {key: token_tuple_json, value: email} to redis cache for 300 seconds
    let token_tuple_json: String = serde_json::to_string(&token_tuple).unwrap();
    let expiry_in_seconds: Option<i64> = Some(300);

    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.store_token_for_user(&token_tuple_json, &email, expiry_in_seconds);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginEmail(Error::ServerError),
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

#[cfg(test)]
mod tests {
    // use actix_web::{test, web, App};
    // use dotenv::dotenv;

    #[actix_web::test]
    async fn test_post_email_while_being_authenticated_without_email() {}
    #[actix_web::test]
    async fn test_post_email_while_being_authenticated_with_email() {}

    #[actix_web::test]
    async fn test_post_email_while_not_being_authenticated_without_email() {}
    #[actix_web::test]
    async fn test_post_email_while_not_being_authenticated_with_email() {}
}

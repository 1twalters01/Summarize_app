use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

use crate::{
    generated::protos::accounts::password_reset::email::{
        request,
        response::{self, response::ResponseField},
    },
    models::user::User,
    queries::{postgres::user::get::from_email, redis::general::set_key_value_in_redis},
    utils::email::{compose::compose_password_reset_email_message, handler::send_email},
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        tokens::generate_opaque_token_of_length,
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

    // Check if email is in postgres database
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> = from_email(&pool, email.as_str()).await;

    // extract user or error
    let user: User = match user_result {
        Err(err) => {
            println!("error: {:?}", err);

            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetEmail(Error::UnregistServerErroreredEmail),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(user_option) => match user_option {
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::PasswordResetEmail(Error::UnregisteredEmail),
                    StatusCode::NOT_FOUND,
                ));
            }
            Some(user) => user,
        },
    };
    println!("user: {:#?}", user);

    // get and serialize user
    let user_json: String = serde_json::to_string(&user).unwrap();

    // create a verify token, and a register email token
    let verification_token = generate_opaque_token_of_length(8);
    let header_token = generate_opaque_token_of_length(64);

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
    let token_tuple: (&str, &str) = (header_token, verification_token);
    let token_tuple_json: String = serde_json::to_string(&token_tuple).unwrap();

    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.store_token_for_user(&token_tuple_json, &user_json, expiry_in_seconds);
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

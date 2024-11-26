use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::password_reset::password::{
        request::Request,
        response::{response::ResponseField, Error, Response, Success},
    },
    models::user::User,
    queries::{
        postgres::password_hash::update::from_user,
        redis::{all::get_user_from_token_in_redis, general::delete_key_in_redis},
    },
    services::response_service::ResponseService,
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_password,
    },
};

pub async fn post_password_reset(
    data: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let verification_confirmation_token: String = req
        .headers()
        .get("Password-Reset-Verification-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    println!("verification token: {:?}", verification_confirmation_token);

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

    // get user from token in redis
    let mut con = create_redis_client_connection();
    let mut user: User =
        match get_user_from_token_in_redis(&mut con, &verification_confirmation_token) {
            // if error return error
            Err(err) => {
                println!("error: {:#?}", err);
                return Ok(ResponseService::create_error_response(
                    AppError::PasswordResetPassword(Error::InvalidCredentials),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
            Ok(email) => email,
        };

    // if change is not allowed then error
    let set_password_result = user.set_password(password);
    if set_password_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetPassword(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // save change in postgres
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> = from_user(&pool, &user).await;

    // if sql update error then return an error
    if update_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetPassword(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(&mut con, &verification_confirmation_token);

    // if redis fails then return an error
    if delete_redis_result.is_err() {
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

use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::confirmation::{
        request::Request as Request,
        response::{response, Error, Response, Success},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_password,
    },
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_confirmation(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request { password } = req_body.0;
    let login_email_token: String = req
        .headers()
        .get("Change-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::Confirmation(Error::InvalidCredentials),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid_str: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid_str).await;
    let user: User = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        },
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // authenticate password
    if user.check_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::Confirmation(Error::IncorrectPassword),
            StatusCode::UNAUTHORIZED,
        ));
    };

    // Get email from redis
    let cache_service = CacheService::new(create_redis_client_connection());
    let token_object_result = cache_service.get_email_object_from_token(&login_email_token);
    let email: String = match token_object_result {
        // if error return error
        Err(err) => {
            println!("err: {:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::Confirmation(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        },
        Ok(object) => match object.user_uuid == user_uuid_str {
            true => object.email,
            false => {
                return Ok(ResponseService::create_error_response(
                    AppError::Confirmation(Error::ServerError),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ));
            }
        },
    };

    // change email
    let user_service = UserService::new(create_pg_pool_connection().await);
    let update_result: Result<(), sqlx::Error> = user_service.update_email_for_uuid(&email, &user.get_uuid()).await;

    // if sql update error then return an error
    if update_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::Confirmation(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::Confirmation(Response {
            response_field: Some(response::ResponseField::Success(Success{})),
        }),
        StatusCode::OK,
    ));
}

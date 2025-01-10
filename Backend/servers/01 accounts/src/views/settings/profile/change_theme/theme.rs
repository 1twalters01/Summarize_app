use std::str::FromStr;

use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::theme::{
        request::Request,
        response::{response, Error, Response, Success},
    },
    models::user::User,
    services::{response_service::ResponseService, user_service::UserService},
    utils::{database_connections::create_pg_pool_connection, validations::validate_theme},
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};
use uuid::Uuid;

pub async fn post_language(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request {
        request_field: theme,
    } = req_body.0;

    // validate theme
    let validated_theme = validate_theme(theme.clone());
    if validated_theme.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeTheme(Error::InvalidTheme),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid_str: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeTheme(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid_str).await;
    _ = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeTheme(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(user) => match user {
            Some(_) => (),
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::ChangeTheme(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };
    let user_uuid: Uuid = Uuid::from_str(&user_uuid_str).unwrap();

    // change theme
    let user_service = UserService::new(create_pg_pool_connection().await);
    let update_result: Result<(), sqlx::Error> = user_service
        .update_theme_for_uuid(theme.expect("invalid theme"), &user_uuid)
        .await;

    // if sql update error then return an error
    if update_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeTheme(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangeTheme(Response {
            response_field: Some(response::ResponseField::Success(Success {})),
        }),
        StatusCode::OK,
    ));
}

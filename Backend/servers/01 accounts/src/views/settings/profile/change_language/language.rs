use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
    },
    generated::protos::settings::profile::language::{
        request::{Language, Request},
        response::{response, Error, Response, Success},
    },
    models::user::User,
    services::{
        response_service::ResponseService, user_service::UserService,
    },
    utils::{database_connections::create_pg_pool_connection, validations::validate_language},
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_language(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request { language } = req_body.0;

    // validate language
    let validated_language = validate_language(language);
    if validated_language.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeLanguage(Error::InvalidCredentials),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeLanguage(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    let user: User = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeLanguage(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        },
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::ChangeLanguage(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // change language
    let language_str: &str = Language::try_from(language).ok().expect("invalid number").as_str_name();
    let user_service = UserService::new(create_pg_pool_connection().await);
    let update_result: Result<(), sqlx::Error> = user_service.update_language_for_uuid(language_str, &user.get_uuid()).await;

    // if sql update error then return an error
    if update_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeLanguage(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangeLanguage(Response {
            response_field: Some(response::ResponseField::Success(Success {})),
        }),
        StatusCode::OK,
    ));
}


use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
        settings_objects::NameTokenObject,
    },
    generated::protos::settings::profile::name::{
        request::{request::RequestField, BothNames, Request},
        response::{response, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{database_connections::create_redis_client_connection, validations::validate_name},
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_name(req_body: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    // Get first and last names
    let Request { request_field } = req_body.0;
    let (first_name, last_name): (Option<String>, Option<String>) = match request_field.unwrap() {
        RequestField::FirstName(first_name) => (Some(first_name), None),
        RequestField::LastName(last_name) => (None, Some(last_name)),
        RequestField::BothNames(BothNames {
            first_name,
            last_name,
        }) => (Some(first_name), Some(last_name)),
    };

    // validate firstname
    if let Some(ref name) = first_name {
        let validated_firstname = validate_name(&name);
        if validated_firstname.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeName(Error::InvalidName),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }

    // validate lastname
    if let Some(ref name) = last_name {
        let validated_lastname = validate_name(&name);
        if validated_lastname.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeName(Error::InvalidName),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeName(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    _ = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeName(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(user) => match user {
            Some(_) => (),
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::ChangeName(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let token_object: NameTokenObject = NameTokenObject {
        user_uuid,
        first_name,
        last_name,
    };
    let token_object_json = serde_json::to_string(&token_object).unwrap();

    // Save key: token, value: {jwt, email} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let set_redis_result =
        cache_service.store_key_value(&token, &token_object_json, expiry_in_seconds);

    // err handling
    if set_redis_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeName(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return token
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangeName(Response {
            response_field: Some(response::ResponseField::RequiresPassword(true)),
        }),
        StatusCode::OK,
    ));
}

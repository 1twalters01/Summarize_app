use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
        settings_objects::UsernameTokenObject,
    },
    generated::protos::settings::profile::username::{
        request::Request,
        response::{response, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService, user_service::UserService
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_username,
    },
};

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_username(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request { username } = req_body.0;

    // validate username
    let validated_username = validate_username(&username);
    if validated_username.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeUsername(Error::InvalidCredentials),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeUsername(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    _ = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeUsername(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        },
        Ok(user) => match user {
            Some(_) => (),
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::ChangeUsername(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // error if username is already taken
    let user_service = UserService::new(create_pg_pool_connection().await);
    match user_service.get_user_uuid_from_username(&username).await {
        Ok(Some(_)) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangeUsername(Error::RegisteredUsername),
                StatusCode::CONFLICT,
            ));
        }
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::ChangeUsername(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        _ => (),
    };

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let token_object: UsernameTokenObject = UsernameTokenObject { user_uuid, username };
    let token_object_json = serde_json::to_string(&token_object).unwrap();

    // Save key: token, value: {jwt, username} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let set_redis_result = cache_service.store_key_value(&token, &token_object_json, expiry_in_seconds);

    // err handling
    if set_redis_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangeUsername(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return token
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangeUsername(Response {
            response_field: Some(response::ResponseField::RequiresPassword(true)),
        }),
        StatusCode::OK,
    ));
}


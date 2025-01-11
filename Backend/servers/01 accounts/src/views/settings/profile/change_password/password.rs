use crate::{
    datatypes::{
        claims::UserClaims,
        response_types::{AppError, AppResponse},
        settings_objects::PasswordTokenObject,
    },
    generated::protos::settings::profile::password::{
        request::Request,
        response::{response, Error, Response},
    },
    models::{password::Password, user::User},
    queries::postgres::password_hash::get::all_previous_from_user,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_password,
    },
};
use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpMessage, HttpRequest, Responder, Result};

pub async fn post_password(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let Request {
        password,
        password_confirmation,
    } = req_body.0;

    // error if password != password_confirmation
    if password != password_confirmation {
        return Ok(ResponseService::create_error_response(
            AppError::ChangePassword(Error::PasswordsDoNotMatch),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::ChangePassword(Error::InvalidPassword),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangePassword(Error::InvalidCredentials),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    let user = match user_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::ChangePassword(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Ok(ResponseService::create_error_response(
                    AppError::ChangePassword(Error::InvalidCredentials),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        },
    };

    // error if password has already been used
    let pool = create_pg_pool_connection().await;
    let hash_vec_result: Result<Vec<String>, sqlx::Error> =
        all_previous_from_user(&pool, &user).await;
    let hash_vec: Vec<String> = match hash_vec_result {
        Err(err) => {
            println!("Error: {}", err);
            return Ok(ResponseService::create_error_response(
                AppError::ChangePassword(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
        Ok(ref hash_vec) => hash_vec.to_vec(),
    };
    for hash in hash_vec {
        let password_struct = Password::from_hash(hash).unwrap();
        if password_struct.check_password(&password).is_ok() {
            return Ok(ResponseService::create_error_response(
                AppError::ChangePassword(Error::PreviouslyUsedPassword),
                StatusCode::CONFLICT,
            ));
        }
    }

    // hash password
    let password_hash = Password::from_password(&password)
        .unwrap()
        .get_password_hash_string();

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let token_object: PasswordTokenObject = PasswordTokenObject {
        user_uuid,
        password_hash,
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
            AppError::ChangePassword(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return that you require a password
    return Ok(ResponseService::create_success_response(
        AppResponse::ChangePassword(Response {
            response_field: Some(response::ResponseField::Token(token)),
        }),
        StatusCode::OK,
    ));
}

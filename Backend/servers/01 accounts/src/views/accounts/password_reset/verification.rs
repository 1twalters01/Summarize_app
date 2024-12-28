use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::password_reset::verification::{
        request::Request,
        response::{response::ResponseField, Error, Response},
    },
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::database_connections::create_redis_client_connection,
};

#[derive(Debug, Deserialize)]
pub struct VerificationRequestSchema {
    pub header_token: String,
    pub verification_code: String,
}

pub async fn post_verify(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let Request { verification_code } = data.0;
    let password_reset_email_token: String = req
        .headers()
        .get("Password-Reset-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    password_reset_verification_functionality(password_reset_email_token, verification_code).await
}

pub async fn link_verify(
    path: actix_web::web::Path<VerificationRequestSchema>,
) -> Result<impl Responder> {
    let VerificationRequestSchema {
        header_token,
        verification_code,
    } = path.into_inner();

    password_reset_verification_functionality(header_token, verification_code).await
}

async fn password_reset_verification_functionality(
    header_token: String,
    verification_token: String,
) -> Result<impl Responder> {
    // Get email from token using redis
    let token_tuple: (&str, &str) = (&header_token, &verification_token);
    let token_tuple_json = serde_json::to_string(&token_tuple).unwrap();
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_uuid_from_token(&token_tuple_json);
    let user_uuid: Uuid = match cache_result {
        Ok(Some(uuid)) => uuid,
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetVerification(Error::ServerError),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::PasswordResetVerification(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    // create a new token
    let token_service = TokenService::new();
    let password_reset_verification_token = token_service.generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    let expiry_in_seconds: Option<i64> = Some(1800);
    let mut cache_result = cache_service.store_token_for_user_uuid(
        &password_reset_verification_token,
        &user_uuid,
        expiry_in_seconds,
    );
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetVerification(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // delete old {key: token, value: email}
    cache_result = cache_service.delete_key(&token_tuple_json);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::PasswordResetVerification(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::PasswordResetVerification(Response {
            response_field: Some(ResponseField::Token(password_reset_verification_token)),
        }),
        StatusCode::OK,
    ));
}

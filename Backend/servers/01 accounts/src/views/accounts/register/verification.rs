use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, web::Path, HttpRequest, Responder, Result};
use serde::Deserialize;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::register::verification::{
        request,
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

pub async fn post_verify(
    data: ProtoBuf<request::Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let request::Request { verification_code } = data.0;
    let register_email_token: String = req
        .headers()
        .get("Register-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    register_verification_functionality(register_email_token, verification_code).await
}

pub async fn link_verify(path: Path<VerificationRequestSchema>) -> Result<impl Responder> {
    let VerificationRequestSchema {
        header_token,
        verification_code,
    } = path.into_inner();

    register_verification_functionality(header_token, verification_code).await
}

async fn register_verification_functionality(
    header_token: String,
    verification_token: String,
) -> Result<impl Responder> {
    // Form RegisterToken struct
    let token_tuple: (&str, &str) = (&header_token, &verification_token);
    let token_tuple_json = serde_json::to_string(&token_tuple).unwrap();
    println!("schema: {:#?}", token_tuple_json);

    // Get email from token using redis
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_email_from_token_struct_json(&token_tuple_json);
    let email: String = match cache_result {
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::RegisterVerification(Error::IncorrectVerificationCode),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        Ok(email) => email,
    };

    let token_service = TokenService::new();
    let register_verification_token = token_service.generate_opaque_token_of_length(64);
    let expiry_in_seconds: Option<i64> = Some(1800);
    let mut cache_result = cache_service.store_token_for_email(
        &register_verification_token,
        &email,
        expiry_in_seconds,
    );
    if cache_result.is_err() {
        panic!("redis error, panic debug")
    }

    // delete old key
    cache_result = cache_service.delete_key(&token_tuple_json);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::RegisterVerification(Error::IncorrectVerificationCode),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // return ok
    return Ok(ResponseService::create_success_response(
        AppResponse::RegisterVerification(Response {
            response_field: Some(ResponseField::Token(register_verification_token)),
        }),
        StatusCode::OK,
    ));
}

#[cfg(test)]
mod tests {
    // use actix_web::{test, web, App};
    // use dotenv::dotenv;

    #[actix_web::test]
    async fn test_post_verification_while_being_authenticated_without_verification_token_without_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_being_authenticated_without_verification_token_with_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_being_authenticated_with_verification_token_without_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_being_authenticated_with_verification_token_with_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_not_being_authenticated_without_verification_token_without_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_not_being_authenticated_without_verification_token_with_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_not_being_authenticated_with_verification_token_without_header_token(
    ) {
    }

    #[actix_web::test]
    async fn test_post_verification_while_not_being_authenticated_with_verification_token_with_header_token(
    ) {
    }
}

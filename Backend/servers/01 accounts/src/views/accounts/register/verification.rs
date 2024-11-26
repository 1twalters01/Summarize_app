use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, web::Path, HttpRequest, Responder, Result};
use serde::Deserialize;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::register::verification::{
        request,
        response::{response::ResponseField, Error, Response},
    },
    queries::redis::{
        all::get_email_from_token_struct_in_redis,
        general::{delete_key_in_redis, set_key_value_in_redis},
    },
    services::{
        cache_service::CacheService, response_service::ResponseService,
        token_service::TokenService, user_service::UserService,
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
    let token_struct: (&str, &str) = (&header_token, &verification_token);
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();
    println!("schema: {:#?}", token_struct_json);

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(&mut con, &token_struct_json) {
        // if error return error
        Err(_) => {
            return Ok(ResponseService::create_error_response(
                AppError::RegisterVerification(Error::IncorrectVerificationCode),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
        Ok(email) => email,
    };

    // create a new token
    let token_service = TokenService::new();
    let register_verification_token = token_service.generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        &mut con,
        &register_verification_token,
        &email,
        expiry_in_seconds,
    );
    if set_redis_result.is_err() {
        panic!("redis error, panic debug")
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(&mut con, &token_struct_json);

    // if redis fails then return an error
    if delete_redis_result.is_err() {
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

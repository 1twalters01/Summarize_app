use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{web::Path, HttpRequest, HttpResponse, Responder, Result};
use serde::Deserialize;

use crate::{
    queries::redis::{
        all::get_email_from_token_struct_in_redis,
        general::{
            set_key_value_in_redis,
            delete_key_in_redis,
        },
    },
    generated::protos::accounts::register::verification::{
        request,
        response::{self, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_redis_client_connection,
        },
        tokens::generate_opaque_token_of_length,
    },
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
    let token_struct: (String, String) = (header_token.clone(), verification_token.clone());
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();
    println!("schema: {:#?}", token_struct_json);

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(con, &token_struct_json) {
        // if error return error
        Err(err) => {
            println!("error: {:#?}", err);
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(
                    response::Error::IncorrectVerificationCode as i32,
                )),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(email) => email,
    };

    // create a new token
    let register_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result =
        set_key_value_in_redis(con, &register_verification_token, &email, expiry_in_seconds);
    if set_redis_result.is_err() {
        panic!("redis error, panic debug")
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &token_struct_json);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::IncorrectVerificationCode as i32,
            )),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Token(register_verification_token)),
    };
    println!("response: {:#?}", response);
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
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

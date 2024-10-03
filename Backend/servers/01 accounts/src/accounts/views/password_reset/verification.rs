use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};
use serde::Deserialize;

use crate::{
    accounts::queries::redis::get_user_json_from_token_struct_in_redis,
    generated::protos::accounts::password_reset::verification::{
        request::Request,
        response::{self, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_redis_client_connection, delete_key_in_redis, set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
    },
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
    let token_struct: (String, String) = (header_token, verification_token);
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();
    let mut con = create_redis_client_connection();
    let user_json: String = match get_user_json_from_token_struct_in_redis(con, &token_struct_json)
    {
        // if error return error
        Err(err) => {
            println!("error: {:#?}", err);
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(
                    response::Error::InvalidCredentials as i32,
                )),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user_json) => user_json,
    };

    // create a new token
    let password_reset_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        con,
        &password_reset_verification_token,
        &user_json,
        expiry_in_seconds,
    );
    if set_redis_result.is_err() {
        panic!("redis error, panic debug")
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &token_struct_json);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Token(password_reset_verification_token)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

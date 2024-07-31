use actix_web::{HttpRequest, HttpResponse, Responder, Result, web::Path};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};

use crate::{
    generated::protos::accounts::register::verification::{
        response::{self, response::ResponseField},
        request,
    },
    accounts::{
        queries::redis::get_email_from_token_struct_in_redis,
        schema::register::{
            DualVerificationToken,
            VerificationRequestSchema,
        },
    },
    utils::{
        database_connections::{
            create_redis_client_connection, delete_key_in_redis,
            set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
    },
};


pub async fn post_verify(data: ProtoBuf<request::Request>, req: HttpRequest) -> Result<impl Responder> {
    let request::Request { verification_code } = data.0;
    let register_email_token: String = req
        .headers()
        .get("register_email_token")
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
    let token_struct: DualVerificationToken = DualVerificationToken {
        verification_token,
        header_token,
    };
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();
    println!("schema: {:#?}", token_struct_json);

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(con, &token_struct_json) {
        // if error return error
        Err(err) => {
            println!("error: {:#?}", err);
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::IncorrectVerificationCode as i32)),
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
    let set_redis_result = set_key_value_in_redis(
        con,
        &register_verification_token,
        &email,
        &expiry_in_seconds,
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
            response_field: Some(ResponseField::Error(response::Error::IncorrectVerificationCode as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Token(register_verification_token)),
    };
    return Ok(HttpResponse::InternalServerError()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}


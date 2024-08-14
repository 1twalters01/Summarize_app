use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::users::User, emails::compose_password_reset_email_message,
        queries::postgres::get_user_from_email_in_pg_users_table,
    },
    generated::protos::accounts::password_reset::email::{
        request,
        response::{self, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, set_key_value_in_redis,
        },
        email_handler::send_email,
        tokens::generate_opaque_token_of_length,
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<request::Request>) -> Result<impl Responder> {
    let request::Request { email } = data.0;

    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::InvalidEmail as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    println!("email: {:?}", email);

    // Check if email is in postgres database
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;

    // extract user or error
    let user: User = match user_result {
        Err(err) => {
            println!("error: {:?}", err);

            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user_option) => match user_option {
            None => {
                let response: response::Response = response::Response {
                    response_field: Some(ResponseField::Error(
                        response::Error::UnregisteredEmail as i32,
                    )),
                };
                return Ok(HttpResponse::NotFound()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
            Some(user) => user,
        },
    };
    println!("user: {:#?}", user);

    // get and serialize user
    let user_json: String = serde_json::to_string(&user).unwrap();

    // create a verify token, a register email token, and a register_email_token_struct
    let verification_token = generate_opaque_token_of_length(8);
    let header_token = generate_opaque_token_of_length(64);
    let token_struct: (String, String) = (header_token.clone(), verification_token.clone());
    let token_struct_json: String = serde_json::to_string(&token_struct).unwrap();

    // try to email the account a message containing the token
    let message = compose_password_reset_email_message(&verification_token, &header_token);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::EmailFailedToSend as i32,
            )),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // add {key: token, value: UUID} to redis
    let con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(300);

    let set_redis_result =
        set_key_value_in_redis(con, &token_struct_json, &user_json, &expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    println!("complete");

    // return ok
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Token(header_token)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

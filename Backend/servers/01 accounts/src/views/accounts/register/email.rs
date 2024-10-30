use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

use crate::{
    queries::{
        postgres::user::get::from_email,
        redis::general::set_key_value_in_redis,
    },
    models::user::User,
    utils::email::{
        compose::compose_register_email_message,
        handler::send_email,
    },
    generated::protos::accounts::register::email::{
        request,
        response::{self, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection,
        },
        tokens::generate_opaque_token_of_length,
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<request::Request>) -> Result<impl Responder> {
    let request::Request { email } = data.0;

    // Validate the email from the request body
    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::InvalidEmail as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // try to get the user from postgres using the email
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        from_email(&pool, &email).await;

    match user_result {
        Err(err) => {
            println!("error: {:?}", err);

            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user_option) if user_option.is_some() == true => {
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(
                    response::Error::RegisteredEmail as i32,
                )),
            };
            return Ok(HttpResponse::Conflict()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        _ => {}
    };

    // create a verify token, a register email token, and a register_email_token_struct
    let verification_token = generate_opaque_token_of_length(8);
    let header_token = generate_opaque_token_of_length(64);
    let token_struct: (String, String) = (header_token.clone(), verification_token.clone());
    let token_struct_json: String = serde_json::to_string(&token_struct).unwrap();

    // try to email the account a message containing the token
    let message = compose_register_email_message(&verification_token, &header_token);
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

    // save {key: token, value: email} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result =
        set_key_value_in_redis(con, &token_struct_json, &email, expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Token(header_token)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

#[cfg(test)]
mod tests {
    // use actix_web::{test, web, App};
    // use dotenv::dotenv;

    #[actix_web::test]
    async fn test_post_email_while_being_authenticated_without_email() {}
    #[actix_web::test]
    async fn test_post_email_while_being_authenticated_with_email() {}

    #[actix_web::test]
    async fn test_post_email_while_not_being_authenticated_without_email() {}
    #[actix_web::test]
    async fn test_post_email_while_not_being_authenticated_with_email() {}
}

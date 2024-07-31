use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

use crate::{
    accounts::{datatypes::users::User, queries::postgres::get_user_from_email_in_pg_users_table},
    generated::protos::accounts::login::email::{
        request,
        response::{self, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<request::Request>) -> Result<impl Responder> {
    // get request variable
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
        get_user_from_email_in_pg_users_table(&pool, &email).await;

    // if user does not exist or is none then return an error
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

    // create a token
    let token: String = generate_opaque_token_of_length(25);

    // serialize the user
    let user_json = serde_json::to_string(&user).unwrap();

    // save {key: token, value: user} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token, &user_json, &expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Token(token)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

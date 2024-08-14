use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::{token_object::UserRememberMe, users::User},
        queries::redis::get_user_from_token_in_redis,
        schema::auth::AuthTokens,
    },
    generated::protos::accounts::{
        auth_tokens,
        login::password::{
            request,
            response::{self, response::ResponseField},
        },
    },
    utils::{
        database_connections::{
            create_redis_client_connection, delete_key_in_redis, set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
        validations::validate_password,
    },
};

pub async fn post_password(
    data: ProtoBuf<request::Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let login_email_token: String = req
        .headers()
        .get("Login-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let request::Request {
        password,
        remember_me,
    } = data.0;

    // try to get user from token in redis
    let mut con = create_redis_client_connection();
    let user: User = match get_user_from_token_in_redis(con, &login_email_token) {
        // if error return error
        Err(err) => {
            println!("Error, {:?}", err);

            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(
                    response::Error::InvalidCredentials as i32,
                )),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => user,
    };

    // check if the entered password is a valid password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::InvalidPassword as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    println!("password: {:#?}", password);

    // check if password is correct for the given user
    let check_password: Result<(), std::io::Error> = user.check_password(&password);
    if check_password.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::IncorrectPassword as i32,
            )),
        };
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // see if account has a totp
    if user.is_totp_activated() == true {
        // create a token and a serialized UserRememberMe{ remember_me: bool, token: String }
        let token: String = generate_opaque_token_of_length(25);
        let token_object: UserRememberMe = UserRememberMe { remember_me, user };
        let token_object_json = serde_json::to_string(&token_object).unwrap();

        // save {key: token, value: UserRememberMe} to redis
        let expiry_in_seconds: Option<i64> = Some(300);
        let mut con = create_redis_client_connection();
        let set_redis_result =
            set_key_value_in_redis(con, &token, &token_object_json, &expiry_in_seconds);

        // if redis fails then return an error
        if set_redis_result.is_err() {
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
            };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }

        // delete old token
        con = create_redis_client_connection();
        let delete_redis_result = delete_key_in_redis(con, &login_email_token);

        // if redis fails then return an error
        if delete_redis_result.await.is_err() {
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
            };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }

        // return success
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Success(response::Success {
                token: Some(response::Token {
                    token_field: Some(response::token::TokenField::Response(token)),
                }),
                requires_totp: true,
            })),
        };
        return Ok(HttpResponse::Ok()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    println!("inactive totp");

    // update last login time
    // generate tokens
    let auth_tokens: auth_tokens::AuthTokens = match AuthTokens::new(user, remember_me).await {
        Ok(tokens) => auth_tokens::AuthTokens {
            refresh: tokens.refresh_token,
            access: tokens.access_token.to_string(),
        },
        Err(err) => {
            println!("error: {:#?}", err);
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    println!("auth tokens: {:#?}", auth_tokens);

    // delete old token
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &login_email_token);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return success
    let response: response::Response = response::Response {
        response_field: Some(ResponseField::Success(response::Success {
            token: Some(response::Token {
                token_field: Some(response::token::TokenField::Tokens(auth_tokens)),
            }),
            requires_totp: false,
        })),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

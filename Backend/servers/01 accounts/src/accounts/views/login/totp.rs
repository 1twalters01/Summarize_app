use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::users::User, queries::redis::get_user_remember_me_from_token_in_redis,
        schema::auth::AuthTokens,
    },
    generated::protos::accounts::{
        auth_tokens,
        login::totp::{
            request,
            response::{self, response::ResponseField},
        },
    },
    utils::{
        database_connections::{create_redis_client_connection, delete_key_in_redis},
        validations::validate_totp,
    },
};

pub async fn post_totp(
    // data: Json<LoginTotpRequest>,
    data: ProtoBuf<request::Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let login_password_token: String = req
        .headers()
        .get("Login-Password-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let request::Request {
        digit1,
        digit2,
        digit3,
        digit4,
        digit5,
        digit6,
    } = data.0;

    // Try to get TokenObject from redis
    let mut con = create_redis_client_connection();
    let (mut user, remember_me): (User, bool) =
        match get_user_remember_me_from_token_in_redis(con, &login_password_token) {
            // if error return error
            Err(err) => {
                println!("err: {:#?}", err);
                let response: response::Response = response::Response {
                    response_field: Some(ResponseField::Error(
                        response::Error::InvalidCredentials as i32,
                    )),
                };

                return Ok(HttpResponse::UnprocessableEntity()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
            Ok(user_remember_me) => (user_remember_me.user, user_remember_me.remember_me),
        };

    // see if account has a totp
    let has_totp = user.is_totp_activated();
    if has_totp == false {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::InvalidTotp as i32)),
        };

        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // check if the entered totp is a valid totp
    let validated_totp = validate_totp(digit1, digit2, digit3, digit4, digit5, digit6);
    if validated_totp.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::InvalidTotp as i32)),
        };

        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // check totp
    if user.check_totp(digit1, digit2, digit3, digit4, digit5, digit6) == false {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::IncorrectTotp as i32)),
        };

        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // delete old token from redis
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &login_password_token);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };

        return Ok(HttpResponse::FailedDependency()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // update last login time
    // create auth tokens
    let auth_tokens: auth_tokens::AuthTokens = match AuthTokens::new(user, remember_me).await {
        Ok(tokens) => auth_tokens::AuthTokens {
            refresh: tokens.refresh_token,
            access: tokens.access_token.to_string(),
        },
        Err(err) => {
            println!("err: {:#?}", err);
            let response: response::Response = response::Response {
                response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
            };

            return Ok(HttpResponse::FailedDependency()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    println!("auth tokens: {:#?}", auth_tokens);

    // delete old token
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &login_password_token);

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
        response_field: Some(ResponseField::Tokens(auth_tokens)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

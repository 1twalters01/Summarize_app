use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::users::User,
        queries::{
            postgres::update_password_for_user_in_pg_users_table,
            redis::get_user_from_token_in_redis,
        },
    },
    generated::protos::accounts::password_reset::password::{
        request,
        response::{self, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
        },
        validations::validate_password,
    },
};

pub async fn post_password_reset(
    data: ProtoBuf<request::Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let verification_confirmation_token: String = req
        .headers()
        .get("Password-Reset-Verification-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    println!("verification token: {:?}", verification_confirmation_token);

    let request::Request {
        password,
        password_confirmation,
    } = data.0;

    if password != password_confirmation {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(
                response::Error::IncorrectPasswordConfirmation as i32,
            )),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

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

    // get user from token in redis
    let mut con = create_redis_client_connection();
    let mut user: User = match get_user_from_token_in_redis(con, &verification_confirmation_token) {
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
        Ok(email) => email,
    };

    // if change is not allowed then error
    let set_password_result = user.set_password(password);
    if set_password_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };

        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // save change in postgres
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_password_for_user_in_pg_users_table(&pool, &user).await;

    // if sql update error then return an error
    if update_result.is_err() {
        let response: response::Response = response::Response {
            response_field: Some(ResponseField::Error(response::Error::ServerError as i32)),
        };

        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &verification_confirmation_token);

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
        response_field: Some(ResponseField::Success(response::Success {})),
    };

    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

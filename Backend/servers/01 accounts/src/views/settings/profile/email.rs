use crate::{
    datatypes::claims::Claims,
    generated::protos::settings::profile::{
        confirmation::{
            response as confirmation_response, Error as PasswordError, Request as PasswordRequest,
            Response as PasswordResponse, Success as PasswordSuccess,
        },
        email::{
            request::Request as MainRequest,
            response::{response, Error as MainError, Response as MainResponse},
        },
    },
    models::user::User,
    queries::{postgres::user::get::from_email, redis::general::set_key_value_in_redis},
    services::token_service::TokenService,
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::{validate_email, validate_password},
    },
};

use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
struct EmailTokenObject {
    user_uuid: String,
    email: String,
}

pub async fn post_email(
    req_body: ProtoBuf<MainRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let MainRequest { email } = req_body.0;

    // validate email
    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(
                MainError::InvalidCredentials as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            let response: MainResponse = MainResponse {
                response_field: Some(response::ResponseField::Error(
                    MainError::InvalidCredentials as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    _ = match user_result {
        Err(_) => {
            let response: MainResponse = MainResponse {
                response_field: Some(response::ResponseField::Error(
                    MainError::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let response: MainResponse = MainResponse {
                    response_field: Some(response::ResponseField::Error(
                        MainError::InvalidCredentials as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        },
    };

    // error if email is already taken
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> = from_email(&pool, &email).await;

    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == true {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(
                MainError::RegisteredEmail as i32,
            )),
        };

        return Ok(HttpResponse::Conflict()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let token_object: EmailTokenObject = EmailTokenObject { user_uuid, email };
    let token_object_json = serde_json::to_string(&token_object).unwrap();

    // Save key: token, value: {jwt, email} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut con = create_redis_client_connection();
    let set_redis_result =
        set_key_value_in_redis(&mut con, &token, &token_object_json, expiry_in_seconds);

    // err handling
    if set_redis_result.is_err() {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(
                MainError::ServerError as i32,
            )),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return token
    let response: MainResponse = MainResponse {
        response_field: Some(response::ResponseField::RequiresPassword(true)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

pub async fn post_confirmation(
    req_body: ProtoBuf<PasswordRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let PasswordRequest { password } = req_body.0;
    let login_email_token: String = req
        .headers()
        .get("Change-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(confirmation_response::ResponseField::Error(
                PasswordError::InvalidCredentials as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(confirmation_response::ResponseField::Error(
                    PasswordError::InvalidCredentials as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    let user: User = match user_result {
        Err(_) => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(confirmation_response::ResponseField::Error(
                    PasswordError::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let response: PasswordResponse = PasswordResponse {
                    response_field: Some(confirmation_response::ResponseField::Error(
                        PasswordError::InvalidCredentials as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        },
    };

    // authenticate password
    if user.check_password(&password).is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(confirmation_response::ResponseField::Error(
                PasswordError::IncorrectPassword as i32,
            )),
        };
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    };

    // Get email from redis
    let con = create_redis_client_connection();
    let email: String = match get_object_from_token_in_redis(con, &login_email_token) {
        // if error return error
        Err(err) => {
            println!("err: {:#?}", err);
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(confirmation_response::ResponseField::Error(
                    PasswordError::InvalidCredentials as i32,
                )),
            };

            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(object) => match object.user_uuid == user_uuid {
            true => object.email,
            false => {
                let response: PasswordResponse = PasswordResponse {
                    response_field: Some(confirmation_response::ResponseField::Error(
                        PasswordError::ServerError as i32,
                    )),
                };

                return Ok(HttpResponse::UnprocessableEntity()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        },
    };

    // change email
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_email_for_user_in_pg_users_table(&pool, &user_uuid, &email).await;

    // if sql update error then return an error
    if update_result.is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(confirmation_response::ResponseField::Error(
                PasswordError::ServerError as i32,
            )),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: PasswordResponse = PasswordResponse {
        response_field: Some(confirmation_response::ResponseField::Success(
            PasswordSuccess {},
        )),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

use redis::{Commands, Connection, RedisResult};
fn get_object_from_token_in_redis(
    mut con: Connection,
    token: &str,
) -> Result<EmailTokenObject, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let object_json: String = match redis_result {
        Ok(object_json) => object_json,
        Err(err) => return Err(err.to_string()),
    };
    let object: EmailTokenObject = serde_json::from_str(&object_json).unwrap();
    return Ok(object);
}

pub async fn update_email_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    user_uuid: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET email=($1) WHERE uuid=($2);")
        .bind(email)
        .bind(user_uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

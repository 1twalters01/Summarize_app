use crate::{
    generated::protos::settings::profile::{
        confirmation::{
            response as confirmation_response, Error as PasswordError, Request as PasswordRequest,
            Response as PasswordResponse, Success as PasswordSuccess,
        },
        password::{
            request::Request as MainRequest,
            response::{response, Error as MainError, Response as MainResponse},
        },
    },
    models::{password::Password, user::User},
    queries::{
        postgres::password_hash::get::all_previous_from_user,
        redis::general::set_key_value_in_redis,
    },
    services::token_service::{Claims, TokenService},
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_password,
    },
};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
struct PasswordTokenObject {
    user_uuid: String,
    password_hash: String,
}

pub async fn post_password(
    req_body: ProtoBuf<MainRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let MainRequest {
        password,
        password_confirmation,
    } = req_body.0;

    // error if password != password_confirmation
    if password != password_confirmation {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(
                MainError::PasswordsDoNotMatch as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
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
    let user: User = match user_result {
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

    // error if password has already been used
    let pool = create_pg_pool_connection().await;
    let hash_vec_result: Result<Vec<String>, sqlx::Error> =
        all_previous_from_user(&pool, &user).await;
    let hash_vec: Vec<String> = match hash_vec_result {
        Err(err) => {
            println!("Error: {}", err);
            let response: MainResponse = MainResponse {
                response_field: Some(response::ResponseField::Error(
                    MainError::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(ref hash_vec) => hash_vec.to_vec(),
    };
    for hash in hash_vec {
        let password_struct = Password::from_hash(hash).unwrap();
        if password_struct.check_password(&password).is_ok() {
            let response: MainResponse = MainResponse {
                response_field: Some(response::ResponseField::Error(
                    MainError::PreviouslyUsedPassword as i32,
                )),
            };
            return Ok(HttpResponse::Conflict()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    }

    // hash password
    let password_hash = Password::from_password(password)
        .unwrap()
        .get_password_string();

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let token_object: PasswordTokenObject = PasswordTokenObject {
        user_uuid,
        password_hash,
    };
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
    let login_password_token: String = req
        .headers()
        .get("Change-Password-Token")
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

    // Get password hash from redis
    let con = create_redis_client_connection();
    let password_hash: String = match get_object_from_token_in_redis(con, &login_password_token) {
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
            true => object.password_hash,
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

    // change password
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_password_for_user_in_pg_users_table(&pool, &user_uuid, &password_hash).await;

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
) -> Result<PasswordTokenObject, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let object_json: String = match redis_result {
        Ok(object_json) => object_json,
        Err(err) => return Err(err.to_string()),
    };
    let object: PasswordTokenObject = serde_json::from_str(&object_json).unwrap();
    return Ok(object);
}

pub async fn update_password_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    user_uuid: &str,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET password=($1) WHERE uuid=($2);")
        .bind(password_hash)
        .bind(user_uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

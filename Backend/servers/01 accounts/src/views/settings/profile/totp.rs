use crate::{
    datatypes::claims::UserClaims,
    generated::protos::settings::profile::{
        confirmation::{
            response as password_response, Error as PasswordError, Request as PasswordRequest,
            Response as PasswordResponse, Success as PasswordSuccess,
        },
        totp::{
            request::Request as TotpRequest,
            response::{
                response as totp_response, Error as TotpError, Response as TotpResponse,
                Success as TotpSuccess,
            },
        },
    },
    models::{totp::Totp, user::User},
    queries::redis::general::set_key_value_in_redis,
    services::token_service::TokenService,
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::{validate_password, validate_totp},
    },
};
use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Result};

pub async fn post_totp(
    req_body: ProtoBuf<PasswordRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let PasswordRequest { password } = req_body.0;

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(password_response::ResponseField::Error(
                PasswordError::InvalidCredentials as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(password_response::ResponseField::Error(
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
                response_field: Some(password_response::ResponseField::Error(
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
                    response_field: Some(password_response::ResponseField::Error(
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
            response_field: Some(password_response::ResponseField::Error(
                PasswordError::IncorrectPassword as i32,
            )),
        };
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    };

    // Generate token
    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let token_object: String = user_uuid;

    // Save key: token, value: {token, uuid/jwt} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut con = create_redis_client_connection();
    let set_redis_result =
        set_key_value_in_redis(&mut con, &token, &token_object, expiry_in_seconds);

    // err handling
    if set_redis_result.is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(password_response::ResponseField::Error(
                PasswordError::ServerError as i32,
            )),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: PasswordResponse = PasswordResponse {
        response_field: Some(password_response::ResponseField::Success(
            PasswordSuccess {},
        )),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

pub async fn post_confirmation(
    req_body: ProtoBuf<TotpRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let TotpRequest {
        digit1,
        digit2,
        digit3,
        digit4,
        digit5,
        digit6,
    } = req_body.0;
    let login_email_token: String = req
        .headers()
        .get("Change-Totp-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // validate totp
    let validated_totp = validate_totp(digit1, digit2, digit3, digit4, digit5, digit6);
    if validated_totp.is_err() {
        let response: TotpResponse = TotpResponse {
            response_field: Some(totp_response::ResponseField::Error(
                TotpError::InvalidCredentials as i32,
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // Validate user
    let user_uuid: String = match req.extensions().get::<UserClaims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            let response: TotpResponse = TotpResponse {
                response_field: Some(totp_response::ResponseField::Error(
                    TotpError::InvalidCredentials as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    match user_result {
        Err(_) => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(password_response::ResponseField::Error(
                    PasswordError::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(_) => {}
            None => {
                let response: PasswordResponse = PasswordResponse {
                    response_field: Some(password_response::ResponseField::Error(
                        PasswordError::InvalidCredentials as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        },
    };

    // get uuid from redis - make the jwt the token instead of uuid for safety?
    let con = create_redis_client_connection();
    let saved_uuid: String = match get_object_from_token_in_redis(con, &login_email_token) {
        // if error return error
        Err(err) => {
            println!("err: {:#?}", err);
            let response: TotpResponse = TotpResponse {
                response_field: Some(totp_response::ResponseField::Error(
                    PasswordError::InvalidCredentials as i32,
                )),
            };

            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(uuid) => uuid,
    };

    // if saved_uuid != uuid then error
    if user_uuid != saved_uuid {
        let response: TotpResponse = TotpResponse {
            response_field: Some(totp_response::ResponseField::Error(
                TotpError::InvalidCredentials as i32,
            )),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // get current totp status
    let pool = create_pg_pool_connection().await;
    let totp_key: Option<String> =
        match get_totp_key_from_uuid_in_pg_users_table(&pool, &user_uuid).await {
            Ok(result) => result,
            Err(_) => {
                let response: TotpResponse = TotpResponse {
                    response_field: Some(totp_response::ResponseField::Error(
                        TotpError::ServerError as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        };

    // if totp = there then delete
    if let Some(totp_key) = totp_key {
        let mut totp: Totp = Totp::from_key(totp_key);

        if totp
            .verify(digit1, digit2, digit3, digit4, digit5, digit6)
            .is_err()
        {
            let response: TotpResponse = TotpResponse {
                response_field: Some(totp_response::ResponseField::Error(
                    TotpError::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        };

        match delete_totp_from_uuid_in_pg_users_table(&pool, &user_uuid).await {
            Ok(_) => {
                let response: TotpResponse = TotpResponse {
                    response_field: Some(totp_response::ResponseField::Success(TotpSuccess {})),
                };
                return Ok(HttpResponse::Ok()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
            Err(_) => {
                let response: TotpResponse = TotpResponse {
                    response_field: Some(totp_response::ResponseField::Error(
                        TotpError::ServerError as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        }
    } else {
        let totp: Totp = Totp::new();
        match set_totp_for_uuid_in_pg_users_table(&pool, &user_uuid, &totp).await {
            Ok(_) => {
                let response: TotpResponse = TotpResponse {
                    response_field: Some(totp_response::ResponseField::Success(TotpSuccess {})),
                };
                return Ok(HttpResponse::Ok()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
            Err(_) => {
                let response: TotpResponse = TotpResponse {
                    response_field: Some(totp_response::ResponseField::Error(
                        TotpError::ServerError as i32,
                    )),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
        }
    }
}

use sqlx::{Pool, Postgres, Row};
async fn get_totp_key_from_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    user_uuid: &str,
) -> Result<Option<String>, sqlx::Error> {
    let user_select_query = sqlx::query("Select totp_key from users WHERE uuid=($1")
        .bind(user_uuid)
        .fetch_all(pool)
        .await;

    match user_select_query {
        Err(err) => return Err(err),
        Ok(res) => {
            if res.len() == 0 {
                return Ok(None);
            }

            let key: Option<String> = res[0].get("totp_key");
            return Ok(key);
        }
    }
}

async fn delete_totp_from_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    user_uuid: &str,
) -> Result<(), sqlx::Error> {
    let user_select_query = sqlx::query("UPDATE users SET totp_key=NULL WHERE uuid=($1)")
        .bind(user_uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_select_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

async fn set_totp_for_uuid_in_pg_users_table(
    pool: &Pool<Postgres>,
    user_uuid: &str,
    totp: &Totp,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET totp_key=($1) WHERE uuid=($2)")
        .bind(totp.get_url())
        .bind(user_uuid)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

use redis::{Commands, Connection, RedisResult};
fn get_object_from_token_in_redis(mut con: Connection, token: &str) -> Result<String, String> {
    let redis_result: RedisResult<String> = con.get(token);
    let object_json: String = match redis_result {
        Ok(object_json) => object_json,
        Err(err) => return Err(err.to_string()),
    };
    let object: String = serde_json::from_str(&object_json).unwrap();
    return Ok(object);
}

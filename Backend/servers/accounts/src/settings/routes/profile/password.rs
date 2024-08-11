use crate::{
    generated::protos::settings::profile::{
        confirmation::{
            response as confirmation_response,
            Request as PasswordRequest,
            Response as PasswordResponse,
            Error as PasswordError,
            Success as PasswordSuccess,
        },
        password::{
            request::Request as MainRequest,
            response::{
                response,
                Response as MainResponse,
                Error as MainError
            },
        },
    },
    accounts::{
        datatypes::users::User,
        queries::postgres::get_user_from_email_in_pg_users_table,
        schema::auth::Claims,
    },
    utils::{
        database_connections::{
            create_pg_pool_connection,
            create_redis_client_connection,
            set_key_value_in_redis
        },
        tokens::generate_opaque_token_of_length,
        validations::{validate_email, validate_password}
    },
};
use actix_web::HttpMessage;
use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
struct EmailTokenObject {
    user_uuid: String,
    password: String
}

#[post("change-password")]
async fn change_password(
    req_body: ProtoBuf<MainRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let MainRequest { new_password, new_password_confirmation} = req_body.0;

    // error if new_password != new_password_confirmation
    if new_password != new_password_confirmation {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(MainError::PasswordsDoNotMatch as i32)),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(MainError::InvalidCredentials as i32)),
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
                response_field: Some(response::ResponseField::Error(MainError::InvalidCredentials as i32)),
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
                response_field: Some(response::ResponseField::Error(MainError::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let response: MainResponse = MainResponse {
                    response_field: Some(response::ResponseField::Error(MainError::InvalidCredentials as i32)),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            },
        },
    };

    // error if password has already been used
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, &email).await;

    let is_password_stored = (&user_result).as_ref().ok().is_some();
    if is_password_stored == true {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(MainError::PreviouslyUsedPassword as i32)),
        };

        return Ok(HttpResponse::Conflict()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // hash password
        
    // Generate token
    let token: String = generate_opaque_token_of_length(25);
    let token_object: PasswordTokenObject = PasswordTokenObject{user_uuid, password};
    let token_object_json = serde_json::to_string(&token_object).unwrap();

    // Save key: token, value: {jwt, email} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token, &token_object_json, &expiry_in_seconds);

    // err handling
    if set_redis_result.is_err() {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(MainError::ServerError as i32)),
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

#[post("change-email")]
async fn post_confirmation(
    req_body: ProtoBuf<PasswordRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let PasswordRequest { password } = req_body.0;
    let login_email_token: String = req
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
            response_field: Some(confirmation_response::ResponseField::Error(PasswordError::InvalidCredentials as i32)),
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
                response_field: Some(confirmation_response::ResponseField::Error(PasswordError::InvalidCredentials as i32)),
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
                response_field: Some(confirmation_response::ResponseField::Error(PasswordError::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
            let response: PasswordResponse = PasswordResponse {
                response_field: Some(confirmation_response::ResponseField::Error(PasswordError::InvalidCredentials as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
            },
        },
    };
    
    // authenticate password
    if user.check_password(&password).is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(confirmation_response::ResponseField::Error(PasswordError::IncorrectPassword as i32)),
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
            },
            Ok(object) => match object.user_uuid == user_uuid {
                true => {object.email},
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
        update_password_for_user_in_pg_users_table(&pool, &email).await;

    // if sql update error then return an error
    if update_result.is_err() {
        let response: PasswordResponse = PasswordResponse {
            response_field: Some(confirmation_response::ResponseField::Error(PasswordError::ServerError as i32)),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: PasswordResponse = PasswordResponse {
        response_field: Some(confirmation_response::ResponseField::Success(PasswordSuccess {  })),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));

}

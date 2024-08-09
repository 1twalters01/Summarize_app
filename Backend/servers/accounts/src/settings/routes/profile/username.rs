use crate::{
    generated::protos::settings::profile::{
        confirmation::{
            response as confirmation_response,
            Request as PasswordRequest,
            Response as PasswordResponse,
            Error as PasswordError,
            Success as PasswordSuccess,
        },
        username::{
            request::Request as MainRequest,
            response::{
                response,
                Response as MainResponse,
                Error as MainError
            }
        },
    },
    accounts::{
        schema::auth::Claims,
        datatypes::users::User,
        queries::{
            postgres::{
                delete_user_from_uuid_in_pg_users_table,
                get_user_from_email_in_pg_users_table,
                get_user_from_username_in_pg_users_table,
                update_password_for_user_in_pg_users_table,
            },
            redis::get_code_from_token_in_redis,
        },
    },
    settings::schema::{
        ChangeUsernameRequestStruct, ChangeUsernameResponseStruct,
        SettingsError,
    },
    utils::{
        database_connections::{
            create_redis_client_connection,
            set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
        database_connections::create_pg_pool_connection,
        validations::{
            validate_email, validate_name, validate_password, validate_username,
        },
    },
};

use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder}; 
use actix_web::{post, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
struct UsernameTokenObject {
    user_uuid: String,
    username: String,
}

#[post("change-username")]
async fn post_username(
    req_body: ProtoBuf<MainRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let MainRequest { username } = req_body.0;

    // validate username
    let validated_username = validate_username(&username);
    if validated_username.is_err() {
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

    // error if username is already taken
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_username_in_pg_users_table(&pool, &username).await;

    let is_username_stored = (&user_result).as_ref().ok().is_some();
    if is_username_stored == true {
        let response: MainResponse = MainResponse {
            response_field: Some(response::ResponseField::Error(MainError::RegisteredUsername as i32)),
        };
        
        return Ok(HttpResponse::Conflict()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(res_body));
    }

    // Generate token
    let token: String = generate_opaque_token_of_length(25);
    let token_object: EmailTokenObject = UsernameTokenObject{user_uuid, username};
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

pub async fn update_username_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    username: &str,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("").bind(username).execute(pool).await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}


use crate::{
    accounts::{
        datatypes::users::User, queries::postgres::get_user_from_email_in_pg_users_table,
        schema::auth::Claims,
    },
    generated::protos::settings::profile::language::{
          request::Request,
          response::{response, Error, Response,
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
        validations::validate_language,
    },
};

use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use sqlx::{Pool, Postgres};


pub async fn post_language(
    req_body: ProtoBuf<Request>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let MainRequest { language } = req_body.0;

    // validate language
    let validated_language = validate_language(&language);
    if validated_language.is_err() {
        let response: Response = Response {
            response_field: Some(response::ResponseField::Error(
                Error::InvalidCredentials as i32,
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

    // change language
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_language_for_user_in_pg_users_table(&pool, &user, &language).await;

    // if sql update error then return an error
    if update_result.is_err() {
        let response: Response = PasswordResponse {
            response_field: Some(response::ResponseField::Error(
                Error::ServerError as i32,
            )),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return ok
    let response: Response = Response {
        response_field: Some(response::ResponseField::Success(
            Success {},
        )),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

pub async fn update_language_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    user: &User
    language: &Language,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("").bind(email).execute(pool).await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }

use crate::{
    accounts::{
        datatypes::users::User,
        schema::auth::Claims,
    },
    generated::protos::settings::profile::language::{
        request::{Language, Request},
        response::{response, Error, Response, Success},
    },
    utils::{
        database_connections::create_pg_pool_connection,
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
    let Request { language } = req_body.0;

    // validate language
    let validated_language = validate_language(language);
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
            let response: Response = Response {
                response_field: Some(response::ResponseField::Error(
                    Error::InvalidCredentials as i32,
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
            let response: Response = Response {
                response_field: Some(response::ResponseField::Error(
                    Error::ServerError as i32,
                )),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let response: Response = Response {
                    response_field: Some(response::ResponseField::Error(
                        Error::InvalidCredentials as i32,
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
        update_language_for_user_in_pg_users_table(&pool, &user, language).await;

    // if sql update error then return an error
    if update_result.is_err() {
        let response: Response = Response {
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
    user: &User,
    language: i32,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("UPDATE users SET language=l.language FROM languages AS l WHERE uuid=($1), l.language = ($2);")
        .bind(user.get_uuid())
        .bind(Language::try_from(language).unwrap().as_str_name())
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}

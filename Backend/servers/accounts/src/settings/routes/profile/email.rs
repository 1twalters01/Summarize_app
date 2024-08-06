use crate::{
    accounts::{
        schema::auth::Claims,
        datatypes::users::User,
        queries::postgres::get_user_from_email_in_pg_users_table
    },
    settings::schema::{
        ChangeEmailRequestStruct, ChangeEmailResponseStruct,
    SettingsError,
    },
    utils::{
        database_connections::create_pg_pool_connection,
        validations::{
            validate_email, validate_password,
        },
    },
};

use actix_web::HttpMessage;
use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};
use sqlx::{Pool, Postgres};

#[post("change-email")]
async fn change_email(
    req_body: Json<ChangeEmailRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeEmailRequestStruct { email, password } = req_body.into_inner();
    let mut res_body: ChangeEmailResponseStruct = ChangeEmailResponseStruct::new();

    // validate password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // validate email
    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_email.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // error if email is already taken
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, &email).await;

    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == true {
        res_body.success = false;
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("username already exists")),
        };
        return Ok(HttpResponse::Conflict()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // authenticate password
    let user_uuid: String = match req.extensions().get::<Claims>() {
        Some(claims) => claims.sub.clone(),
        None => {
            res_body.settings_error = SettingsError {
                is_error: true,
                error_message: Some(String::from("error")),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
    };

    let user_result: Result<Option<User>, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
    let user: User = match user_result {
        Err(_) => {
            res_body.settings_error = SettingsError {
                is_error: true,
                error_message: Some(String::from("error")),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Ok(user) => match user {
            Some(user) => user,
            None => {
                res_body.settings_error = SettingsError {
                    is_error: true,
                    error_message: Some(String::from("error")),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/json; charset=utf-8")
                    .json(res_body));
            },
        },
    };

    if user.check_password(&password).is_err() {
        res_body.success = false;
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("incorrect password")),
        };
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // change email
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_email_for_user_in_pg_users_table(&pool, &email).await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

pub async fn update_email_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("").bind(email).execute(pool).await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}


use crate::accounts::schema::auth::Claims;
use crate::accounts::datatypes::users::User;
use crate::accounts::queries::{
    postgres::{
        delete_user_from_uuid_in_pg_users_table,
        get_user_from_email_in_pg_users_table, get_user_from_username_in_pg_users_table, update_password_for_user_in_pg_users_table,
    },
    redis::get_code_from_token_in_redis,
};
use crate::settings::schema::{
    ChangeEmailRequestStruct, ChangeEmailResponseStruct,
    ChangeNameRequestStruct, ChangeNameResponseStruct,
    ChangePasswordRequestStruct, ChangePasswordResponseStruct,
    ChangeUsernameRequestStruct, ChangeUsernameResponseStruct,
    DeleteAccountConfirmationRequestStruct, DeleteAccountConfirmationResponseStruct,
    DeleteAccountRequestStruct, DeleteAccountResponseStruct,
    SettingsError,
};
use crate::utils::database_connections::{create_redis_client_connection, set_key_value_in_redis};
use crate::utils::tokens::generate_opaque_token_of_length;
use crate::utils::{
    database_connections::create_pg_pool_connection,
    validations::{
        validate_email, validate_name, validate_password, validate_username,
    },
};
use actix_web::HttpMessage;
use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};
use sqlx::{Pool, Postgres};

#[post("change-username")]
async fn change_username(
    req_body: Json<ChangeUsernameRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeUsernameRequestStruct { username, password } = req_body.into_inner();
    let mut res_body: ChangeUsernameResponseStruct = ChangeUsernameResponseStruct::new();

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

    // validate username
    let validated_username = validate_username(&username);
    if validated_username.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(validated_username.err().unwrap()),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // error if username is already taken
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_username_in_pg_users_table(&pool, &username).await;

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

    // if user_result is error then error
    if user_result.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("error")),
        };
        return Ok(HttpResponse::InternalServerError()
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

    // change username
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_username_for_user_in_pg_users_table(&pool, &username).await;

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


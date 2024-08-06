use crate::{
    accounts::{
        schema::auth::Claims,
        datatypes::users::User,
    },
    settings::schema::{
        ChangeNameRequestStruct, ChangeNameResponseStruct,
        SettingsError,
    },
    utils::{
        
            database_connections::create_pg_pool_connection,
            validations::{
                validate_name, validate_password,
            },
       
    },
};
use actix_web::HttpMessage;
use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};
use sqlx::{Pool, Postgres};

#[post("change-name")]
async fn change_name(
    req_body: Json<ChangeNameRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeNameRequestStruct {
        first_name,
        last_name,
        password,
    } = req_body.into_inner();
    let mut res_body: ChangeNameResponseStruct = ChangeNameResponseStruct::new();

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

    // validate firstname
    let validated_firstname = validate_name(&first_name);
    if validated_firstname.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_firstname.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // validate lastname
    let validated_lastname = validate_name(&last_name);
    if validated_lastname.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_lastname.err().unwrap()),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
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
                    error_message: Some(String::from("Invalid user")),
                };
                return Ok(HttpResponse::InternalServerError()
                    .content_type("application/json; charset=utf-8")
                    .json(res_body));
            }
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

    // change name
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_first_name_and_last_name_for_user_in_pg_users_table(&pool, &first_name, &last_name)
            .await;

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

pub async fn update_first_name_and_last_name_for_user_in_pg_users_table(
    pool: &Pool<Postgres>,
    first_name: &str,
    last_name: &str,
) -> Result<(), sqlx::Error> {
    let user_update_query = sqlx::query("")
        .bind(first_name)
        .bind(last_name)
        .execute(pool)
        .await;

    if let Err(err) = user_update_query {
        return Err(err);
    } else {
        return Ok(());
    }
}


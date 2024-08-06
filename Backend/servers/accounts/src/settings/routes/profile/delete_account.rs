use crate::{
    accounts::{
        schema::auth::Claims,
        datatypes::users::User,
        queries::{
            postgres::delete_user_from_uuid_in_pg_users_table,
            redis::get_code_from_token_in_redis,
        }
    },
    settings::schema::{
        DeleteAccountConfirmationRequestStruct, DeleteAccountConfirmationResponseStruct,
        DeleteAccountRequestStruct, DeleteAccountResponseStruct,
        SettingsError,
    },
    utils::{
        database_connections::{create_redis_client_connection, set_key_value_in_redis},
        tokens::generate_opaque_token_of_length,
        database_connections::create_pg_pool_connection,
        validations::validate_password,
    }
};
use actix_web::{post, web::Json, HttpRequest, HttpResponse, HttpMessage, Responder, Result};
use sqlx::{Pool, Postgres};

#[post("delete-account")]
async fn delete_account(
    req_body: Json<DeleteAccountRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let DeleteAccountRequestStruct { password } = req_body.into_inner();
    let mut res_body: DeleteAccountResponseStruct = DeleteAccountResponseStruct::new();

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

    // save {key: token, value: code} to redis
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let token = generate_opaque_token_of_length(64);
    let code = generate_opaque_token_of_length(6);
    let set_redis_result = set_key_value_in_redis(con, &token, &code, &expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return the token
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("delete-account/confirmation")]
async fn delete_account_confirmation(
    req_body: Json<DeleteAccountConfirmationRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let DeleteAccountConfirmationRequestStruct {
        confirmation_code,
        token,
    } = req_body.into_inner();
    let mut res_body: DeleteAccountConfirmationResponseStruct =
        DeleteAccountConfirmationResponseStruct::new();

    // validate the confirmation code
    
    // get stored code from token else error
    let con = create_redis_client_connection();
    let stored_code: String = match get_code_from_token_in_redis(con, &token) {
        // if error return error
        Err(err) => {
            let error: SettingsError = SettingsError {
                is_error: true,
                error_message: Some(err),
            };
            res_body.settings_error = error;
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Ok(code) => code,
    };
    
    // check if code == stored code else error
    if stored_code != confirmation_code {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("invalid code")),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // Get user uuid
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
    
    // delete user else error
    let pool = create_pg_pool_connection().await;
    let user_result: Result<(), sqlx::Error> =
        delete_user_from_uuid_in_pg_users_table(&pool, &user_uuid).await;

    if user_result.is_err() {
        res_body.settings_error = SettingsError {
            is_error: true,
            error_message: Some(String::from("Error deleting user, try again later")),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return success
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}


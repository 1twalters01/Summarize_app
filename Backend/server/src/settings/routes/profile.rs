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

    let user_result: Result<User, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
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
        Ok(user) => user,
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
    let user_result: Result<User, sqlx::Error> =
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

    let user_result: Result<User, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
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
        Ok(user) => user,
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
    let user_result: Result<User, sqlx::Error> =
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

    let user_result: Result<User, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
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
        Ok(user) => user,
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

#[post("change-password")]
async fn change_password(
    req_body: Json<ChangePasswordRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangePasswordRequestStruct {
        new_password,
        new_password_confirmation,
        password,
    } = req_body.into_inner();
    let mut res_body: ChangePasswordResponseStruct = ChangePasswordResponseStruct::new();

    // error if new_password != new_password_confirmation
    if new_password != new_password_confirmation {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(String::from(
                "confirmation is not the same as the new password",
            )),
        };
        res_body.settings_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

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

    // validate new password
    let validated_new_password = validate_password(&new_password);
    if validated_new_password.is_err() {
        let error: SettingsError = SettingsError {
            is_error: true,
            error_message: Some(validated_new_password.err().unwrap()),
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

    let user_result: Result<User, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
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
        Ok(user) => user,
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

    // change password
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_password_for_user_in_pg_users_table(&pool, &password).await;

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

    let user_result: Result<User, sqlx::Error> = User::from_uuid_str(&user_uuid).await;
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
        Ok(user) => user,
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
    let set_redis_result = set_key_value_in_redis(con, &token, &code, &expiry_in_seconds).await;

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


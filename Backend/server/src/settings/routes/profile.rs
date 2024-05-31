use crate::accounts::auth::Claims;
use crate::accounts::datatypes::users::User;
use crate::accounts::db_queries::{
    get_user_from_email_in_pg_users_table, get_user_from_username_in_pg_users_table,
    update_password_for_user_in_pg_users_table,
};
use crate::settings::schema::{
    ChangeEmailRequestStruct, ChangeEmailResponseStruct, ChangeLanguageRequestStruct,
    ChangeLanguageResponseStruct, ChangeNameRequestStruct, ChangeNameResponseStruct,
    ChangePasswordRequestStruct, ChangePasswordResponseStruct, ChangeThemeRequestStruct,
    ChangeThemeResponseStruct, ChangeUsernameRequestStruct, ChangeUsernameResponseStruct,
    DeleteAccountRequestStruct, DeleteAccountResponseStruct, GetThemeResponseStruct, SettingsError,
    ToggleTotpRequestStruct, ToggleTotpResponseStruct,
};
use crate::utils::{
    database_connections::create_pg_pool_connection,
    validations::{
        validate_email, validate_name, validate_password, validate_totp, validate_username,
    },
};
use actix_web::HttpMessage;
use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder, Result};
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
    let DeleteAccountRequestStruct {
        password,
        password_confirmation,
    } = req_body.into_inner();
    let res_body: DeleteAccountResponseStruct = DeleteAccountResponseStruct::new();

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("toggle-totp")]
async fn toggle_totp(
    req_body: Json<ToggleTotpRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ToggleTotpRequestStruct { password, totp } = req_body.into_inner();
    let mut res_body: ToggleTotpResponseStruct = ToggleTotpResponseStruct::new();

    // Authenticate, is this done outside of this function?

    // check if user has totp enabled
    // if no then:
    // if totp is not none then return error
    // generate a totp string
    // set totp
    // if error on setting totp then return error

    // get user's totp string
    // get totp code from string

    // validate the entered totp code
    let validated_email = validate_totp(&totp);
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

    // if the entered totp doesn't match the generated code then error

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

    // remove totp
    // if error on removal then return error

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[get("get-theme")]
async fn get_theme(req: HttpRequest) -> Result<impl Responder> {
    let res_body: GetThemeResponseStruct = GetThemeResponseStruct::new();
    // get user's device - from header? - this means no req_body
    // get user's theme for the device
    // if error when getting the user's theme then return error

    // return ok

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[post("change-theme")]
async fn change_theme(
    req_body: Json<ChangeThemeRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeThemeRequestStruct { theme } = req_body.into_inner();
    let res_body: ChangeThemeResponseStruct = ChangeThemeResponseStruct::new();

    // get user's device (linux app, windows app, mac app, android app, ios app, desktop website, mobile website)
    // set user's theme for the current device to what was entered
    // if setting error then return error

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[post("change-language")]
async fn change_language(
    req_body: Json<ChangeLanguageRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeLanguageRequestStruct { language } = req_body.into_inner();
    let res_body: ChangeLanguageResponseStruct = ChangeLanguageResponseStruct::new();

    // validate the language
    // update the user's language to the new one
    // if error when updating then error
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

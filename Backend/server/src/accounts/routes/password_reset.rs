use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::users::User,
        queries::{
            postgres::{
                get_user_from_email_in_pg_users_table,
                update_password_for_user_in_pg_users_table,
            },
            redis::{
                get_user_json_from_token_struct_in_redis,
                get_user_from_token_in_redis,
            }
        },
        emails::compose_password_reset_email_message,
        schema::{
            errors::AccountError,
            password_reset::{
                PasswordResetConfirmRequestSchema,
                PasswordResetConfirmResponseSchema, PasswordResetRequestSchema,
                PasswordResetResponseSchema,
                DualVerificationToken,
                VerificationRequest, VerificationRequestSchema, VerificationResponseSchema,
            },
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
            set_key_value_in_redis,
        },
        email_handler::send_email,
        tokens::generate_opaque_token_of_length,
        validations::{validate_email, validate_password},
    },
};


pub async fn post_email(req_body: Json<PasswordResetRequestSchema>) -> Result<impl Responder> {
    let PasswordResetRequestSchema { email } = req_body.into_inner();
    let mut res_body: PasswordResetResponseSchema = PasswordResetResponseSchema::new();

    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()));
    }
    println!("email: {:?}", email);

    // Check if email is in postgres database
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;


    // extract user or error
    let user: User = match user_result {
        Err(err) => {
            res_body.account_error = AccountError {
                is_error: true,
                error_message: Some(err.to_string()),
            };
            return Ok(HttpResponse::NotFound()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        },
        Ok(user_option) => match user_option {
            None => {
                res_body.account_error = AccountError {
                    is_error: true,
                    error_message: Some(String::from("Email not found"))
                };
                return Ok(HttpResponse::NotFound()
                    .content_type("application/json; charset=utf-8")
                    .json(res_body));
            },
            Some(user) => user,
        },
    };
    println!("user: {:#?}", user);

    // get and serialize user
    let user_json: String = serde_json::to_string(&user).unwrap();

    // create a verify token, a register email token, and a register_email_token_struct
    let verification_token = generate_opaque_token_of_length(8);
    let header_token = generate_opaque_token_of_length(64);
    let token_struct: DualVerificationToken = DualVerificationToken {
        header_token: header_token.clone(),
        verification_token: verification_token.clone(),
    };
    let token_struct_json: String = serde_json::to_string(&token_struct).unwrap();

    // try to email the account a message containing the token
    let message = compose_password_reset_email_message(&verification_token, &header_token);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(String::from("unable to send email")),
        };
        res_body.account_error = error;
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // add {key: token, value: UUID} to redis
    let con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(300);

    let set_redis_result = set_key_value_in_redis(
        con,
        &token_struct_json,
        &user_json,
        &expiry_in_seconds,
    );

    // if redis fails then return an error
    if set_redis_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    println!("complete");

    // return ok
    res_body.password_reset_response_token = Some(header_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

pub async fn post_verify(req_body: Json<VerificationRequest>, req: HttpRequest) -> Result<impl Responder> {
    let VerificationRequest { verification_token } = req_body.into_inner();
    let password_reset_email_token: String = req
        .headers()
        .get("password_reset_email_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    password_reset_verification_functionality(password_reset_email_token, verification_token).await
}

pub async fn link_verify(path: actix_web::web::Path<VerificationRequestSchema>) -> Result<impl Responder> {
    let VerificationRequestSchema {
        header_token,
        verification_token,
    } = path.into_inner();

    password_reset_verification_functionality(header_token, verification_token).await
}

async fn password_reset_verification_functionality(
    header_token: String,
    verification_token: String,
) -> Result<impl Responder> {
    let mut res_body: VerificationResponseSchema = VerificationResponseSchema::new();

    // Validate tokens

    // Form RegisterToken struct
    let token_struct: DualVerificationToken = DualVerificationToken {
        verification_token,
        header_token,
    };
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let user_json: String = match get_user_json_from_token_struct_in_redis(con, &token_struct_json) {
        // if error return error
        Err(err) => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(err),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Ok(user_json) => user_json,
    };

    // create a new token
    let password_reset_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        con,
        &password_reset_verification_token,
        &user_json,
        &expiry_in_seconds,
    );
    if set_redis_result.is_err() {
        panic!("redis error, panic debug")
    }

    // delete old {key: token, value: email}
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &token_struct_json);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return ok
    res_body.password_reset_response_token = Some(password_reset_verification_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

pub async fn post_password_reset(
    req_body: Json<PasswordResetConfirmRequestSchema>,
    req: HttpRequest,
) -> Result<impl Responder> {
    println!("hi");
    let verification_confirmation_token: String = req
        .headers()
        .get("password_reset_verification_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    println!("verification token: {:?}", verification_confirmation_token);

    let PasswordResetConfirmRequestSchema {
        password,
        password_confirmation,
    } = req_body.into_inner();
    let mut res_body: PasswordResetConfirmResponseSchema =
        PasswordResetConfirmResponseSchema::new();

    if password != password_confirmation {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(false));
    }

    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()));
    }

    // get user from token in redis
    let con = create_redis_client_connection();
    let mut user: User = match get_user_from_token_in_redis(con, &verification_confirmation_token) {
        // if error return error
        Err(err) => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(err),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Ok(email) => email,
    };

    // if change is not allowed then error
    let set_password_result = user.set_password(password);
    if set_password_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("unable to set password")),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // save change in postgres
    let pool = create_pg_pool_connection().await;
    let update_result: Result<(), sqlx::Error> =
        update_password_for_user_in_pg_users_table(&pool, &user).await;

    // if sql update error then return an error
    if update_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    println!("complete");

    // return success
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(true));
}

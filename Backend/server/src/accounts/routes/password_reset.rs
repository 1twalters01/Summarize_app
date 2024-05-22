use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::users::User,
        db_queries::{
            get_email_from_token_struct_in_redis, get_user_from_email_in_pg_users_table, get_user_from_token_in_redis, update_password_for_user_in_pg_users_table
        },
        emails::{compose_password_reset_email_message, send_email},
        schema::{
            AccountError, DualVerificationToken, PasswordResetConfirmRequestSchema, PasswordResetConfirmResponseSchema, PasswordResetRequestSchema, PasswordResetResponseSchema, VerifyRequest, VerifyRequestSchema, VerifyResponseSchema
        },
    },
    databases::connections::{
        create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis, set_key_value_in_redis
    },
    utils::{
        tokens::generate_opaque_token_of_length,
        validations::{
            validate_email,validate_password,
        },
    }
};


#[post("password-reset")]
async fn post_email(req_body: Json<PasswordResetRequestSchema>) -> Result<impl Responder> {
    let PasswordResetRequestSchema { email } = req_body.into_inner();
    let mut res_body: PasswordResetResponseSchema = PasswordResetResponseSchema::new();

    let validated_email = validate_email(email.clone());
    if validated_email.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_email.err().unwrap()));
    }

    // Check if email is in postgres database
    let pool = create_pg_pool_connection().await;
    let user_result: Result<User, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;

    // if not in database then return some not found error
    if user_result.is_ok() == false {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(String::from("email not found")),
        };
        res_body.account_error = error;
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // get and serialize user
    let user: User = user_result.unwrap();
    let user_json: String = serde_json::to_string(&user).unwrap();

    // create a token
    let password_reset_response_token: String = generate_opaque_token_of_length(25);

    // try to email the account a message containing the token
    let message = compose_password_reset_email_message(&password_reset_response_token, &user);
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
        &password_reset_response_token,
        &user_json,
        &expiry_in_seconds,
    );

    // if redis fails then return an error
    if set_redis_result.await.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return ok
    res_body.success = true;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("/verify")]
async fn post_verify(
    req_body: Json<VerifyRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let VerifyRequest { verification_token } = req_body.into_inner();
    let register_email_token: String = req
        .headers()
        .get("header_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    password_reset_verification_functionality(register_email_token, verification_token).await
}

#[post("{uidb64}/{token}")]
async fn link_verify(
    path: actix_web::web::Path<VerifyRequestSchema>,
) -> Result<impl Responder> {
    let VerifyRequestSchema {
        header_token,
        verification_token,
    } = path.into_inner();

    password_reset_verification_functionality(header_token, verification_token).await
}

async fn password_reset_verification_functionality(
    header_token: String,
    verification_token: String,
) -> Result<impl Responder> {
    let mut res_body: VerifyResponseSchema = VerifyResponseSchema::new();

    // Validate tokens

    // Form RegisterToken struct
   let token_struct: DualVerificationToken = DualVerificationToken {
        verification_token,
        header_token,
    };
    let token_struct_json = serde_json::to_string(&token_struct).unwrap();

    // Get email from token using redis
    let mut con = create_redis_client_connection();
    let email: String = match get_email_from_token_struct_in_redis(con, &token_struct_json) {
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

    // create a new token
    let password_reset_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        con,
        &password_reset_verification_token,
        &email,
        &expiry_in_seconds,
    );
    if set_redis_result.await.is_err() {
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
    res_body.is_verification_token_correct = true;
    res_body.verification_confirmation_token = Some(password_reset_verification_token);
    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(true));
}


#[post("confirmation")]
async fn post_password_reset(
    req_body: Json<PasswordResetConfirmRequestSchema>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let verification_confirmation_token: String = req
        .headers()
        .get("verification_confirmation_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

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

    let validated_password = validate_password(password.clone());
    if validated_password.is_err() {
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(validated_password.err().unwrap()));
    }

    // get user from token in redis
    let con = create_redis_client_connection();
    let mut user: User =
        match get_user_from_token_in_redis(con, &verification_confirmation_token) {
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

    // return success
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(true));
}


use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::users::User,
        queries::{
            postgres::{
                create_new_user_in_pg_users_table,
                get_user_from_email_in_pg_users_table,
            },
            redis::get_email_from_token_struct_in_redis,
        },
        emails::compose_register_email_message,
        schema::{
            register::{
                DualVerificationToken, RegisterDetailsRequest,
                RegisterDetailsResponseSchema, RegisterEmailRequestSchema, RegisterEmailResponseSchema,
                VerifyRequest, VerifyRequestSchema, VerifyResponseSchema,
            },
            errors::AccountError,
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
            set_key_value_in_redis,
        },
        email_handler::send_email,
        tokens::generate_opaque_token_of_length,
        validations::{
            validate_email, validate_first_name, validate_last_name, validate_password,
            validate_username,
        },
    },
};

#[post("/email")]
async fn post_email(req_body: Json<RegisterEmailRequestSchema>) -> Result<impl Responder> {
    let RegisterEmailRequestSchema { email } = req_body.into_inner();
    let mut res_body: RegisterEmailResponseSchema = RegisterEmailResponseSchema::new();

    // Validate the email from the request body
    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(validated_email.err().unwrap()),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // try to get the user from postgres using the email
    let pool = create_pg_pool_connection().await;
    let user_result: Result<User, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, &email).await;

    // if email exists then return an error
    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == true {
        res_body.is_email_stored = true;
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("user already exists")),
        };
        return Ok(HttpResponse::Conflict()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // if user_result is any other error then error
    if user_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("error")),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // create a verify token, a register email token, and a register_email_token_struct
    let verification_token = generate_opaque_token_of_length(8);
    let header_token = generate_opaque_token_of_length(64);
    let token_struct: DualVerificationToken = DualVerificationToken {
        header_token: header_token.clone(),
        verification_token: verification_token.clone(),
    };
    let token_struct_json: String = serde_json::to_string(&token_struct).unwrap();

    // try to email the account a message containing the token
    let message = compose_register_email_message(&verification_token, &header_token);
    let message_result = send_email(message, &email);

    // if unable to email then return an error
    if message_result.is_err() {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(String::from("unable to send an email to this address")),
        };
        res_body.account_error = error;
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // save {key: token, value: email} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result =
        set_key_value_in_redis(con, &token_struct_json, &email, &expiry_in_seconds).await;

    // if redis fails then return an error
    if set_redis_result.is_err() {
        println!("{:?}", set_redis_result);
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Server error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return ok
    res_body.register_response_token = Some(header_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[post("/verify")]
async fn post_verify(req_body: Json<VerifyRequest>, req: HttpRequest) -> Result<impl Responder> {
    let VerifyRequest { verification_token } = req_body.into_inner();
    let register_email_token: String = req
        .headers()
        .get("header_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    register_verification_functionality(register_email_token, verification_token).await
}

#[post("/verify/{register_email_token}/{verification_token}")]
async fn link_verify(path: actix_web::web::Path<VerifyRequestSchema>) -> Result<impl Responder> {
    let VerifyRequestSchema {
        header_token,
        verification_token,
    } = path.into_inner();

    register_verification_functionality(header_token, verification_token).await
}

async fn register_verification_functionality(
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
    let register_verification_token = generate_opaque_token_of_length(64);

    // add {key: token, value: email} to redis
    con = create_redis_client_connection();
    let expiry_in_seconds: Option<i64> = Some(1800);
    let set_redis_result = set_key_value_in_redis(
        con,
        &register_verification_token,
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
    res_body.verification_confirmation_token = Some(register_verification_token);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[post("/details")]
async fn post_details(
    req_body: Json<RegisterDetailsRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let RegisterDetailsRequest {
        username,
        password,
        password_confirmation,
        first_name,
        last_name,
    } = req_body.into_inner();

    let verification_confirmation_token: String = req
        .headers()
        .get("verification_confirmation_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut res_body: RegisterDetailsResponseSchema = RegisterDetailsResponseSchema::new();

    // get the email from redis using the token
    let con = create_redis_client_connection();
    let email: String =
        match get_email_from_token_struct_in_redis(con, &verification_confirmation_token) {
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

    // check if the username is already found in the database. If it is then return error
    let validated_username = validate_username(&username);
    if validated_username.is_err() {
        res_body.account_error = AccountError {
            is_error: false,
            error_message: Some(String::from("invalid username")),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    if password != password_confirmation {
        res_body.account_error = AccountError {
            is_error: false,
            error_message: Some(String::from(
                "password does not match confirmation password",
            )),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        res_body.account_error = AccountError {
            is_error: false,
            error_message: Some(String::from("invalid password")),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    println!("password: {:#?}", password);

    if first_name.is_some() {
        let validated_first_name = validate_first_name(first_name.clone().unwrap());
        if validated_first_name.is_err() {
            res_body.account_error = AccountError {
                is_error: false,
                error_message: Some(String::from("invalid first name")),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
    }

    if last_name.is_some() {
        let validated_last_name = validate_last_name(last_name.clone().unwrap());
        if validated_last_name.is_err() {
            res_body.account_error = AccountError {
                is_error: false,
                error_message: Some(String::from("invalid last name")),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
    }

    let create_user: Result<User, std::io::Error> = User::new(username, email, password);
    if create_user.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    let user: User = create_user.unwrap();

    // save details to the user to postgres
    let pool = create_pg_pool_connection().await;
    let save_user_result: Result<(), sqlx::Error> =
        create_new_user_in_pg_users_table(&pool, user).await;

    // if error then return error
    if save_user_result.is_err() {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("internal error")),
        };
        return Ok(HttpResponse::FailedDependency()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // return Ok
    // create an auth token with remember me set to false and send it over as well?
    res_body.success = true;
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

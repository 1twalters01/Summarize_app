use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::{token_object::UserRememberMe, users::User},
        queries::{
            postgres::{
                get_user_from_email_in_pg_users_table,
                get_user_from_refresh_token_in_postgres_auth_table,
            },
            redis::{
                get_user_from_token_in_redis,
                get_user_remember_me_from_token_in_redis,
            },
        },
        schema::{
            auth::{AccessToken, AuthTokens},
            errors::AccountError,
            login::{
                LoginEmailRequestSchema, LoginEmailResponseSchema, LoginPasswordRequest,
                LoginPasswordRequestSchema, LoginPasswordResponseSchema, LoginTotpRequest,
                LoginTotpRequestSchema, LoginTotpResponseSchema,
            },
            refresh_token::RefreshTokenResponseSchema,
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
            set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
        validations::{validate_email, validate_password, validate_totp},
    },
};

#[post("/email")]
async fn post_email(data: Json<LoginEmailRequestSchema>) -> Result<impl Responder> {
    let LoginEmailRequestSchema { email } = data.into_inner();
    let mut res_body: LoginEmailResponseSchema = LoginEmailResponseSchema::new();

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
    println!("email: {}", &email);
    let pool = create_pg_pool_connection().await;
    let user_result: Result<User, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, email.as_str()).await;

    // if user does not exist then return an error
    let is_email_stored = (&user_result).as_ref().ok().is_some();
    if is_email_stored == false {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("user does not exist")),
        };
        return Ok(HttpResponse::NotFound()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // set is_email_stored field to true
    res_body.is_email_stored = true;

    // create a token
    let token: String = generate_opaque_token_of_length(25);

    // serialize the user
    let user: User = user_result.ok().unwrap();
    let user_json = serde_json::to_string(&user).unwrap();

    // save {key: token, value: user} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token, &user_json, &expiry_in_seconds);

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

    // return success
    res_body.is_email_stored = true;
    res_body.login_email_response_token = Some(token);
    return Ok(HttpResponse::NotFound()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[post("/password")]
async fn post_password(
    data: Json<LoginPasswordRequest>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let login_email_response_token: String = req
        .headers()
        .get("login_email_response_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let LoginPasswordRequest {
        password,
        remember_me,
    } = data.into_inner();
    let LoginPasswordRequestSchema {
        login_email_response_token,
        password,
        remember_me,
    } = LoginPasswordRequestSchema {
        login_email_response_token,
        password,
        remember_me,
    };
    let mut res_body: LoginPasswordResponseSchema = LoginPasswordResponseSchema::new();

    // try to get user from token in redis
    let con = create_redis_client_connection();
    let user: User = match get_user_from_token_in_redis(con, &login_email_response_token) {
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
        Ok(user) => user,
    };

    // check if the entered password is a valid password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(validated_password.err().unwrap()),
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    println!("password: {:#?}", password);

    // check if password is correct for the given user
    let check_password: Result<(), std::io::Error> = user.check_password(&password);
    if check_password.is_err() {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(String::from("Incorrect password")),
        };

        res_body.account_error = error;
        res_body.is_password_correct = false;

        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    res_body.is_password_correct = true;

    // see if account has a totp
    if user.is_totp_activated() == true {
        // create a token and a serialized UserRememberMe{ remember_me: bool, token: String }
        let token: String = generate_opaque_token_of_length(25);
        let token_object: UserRememberMe = UserRememberMe { remember_me, user };
        let token_object_json = serde_json::to_string(&token_object).unwrap();

        // save {key: token, value: UserRememberMe} to redis
        let expiry_in_seconds: Option<i64> = Some(300);
        let mut con = create_redis_client_connection();
        let set_redis_result =
            set_key_value_in_redis(con, &token, &token_object_json, &expiry_in_seconds);

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

        // delete old token
        con = create_redis_client_connection();
        let delete_redis_result = delete_key_in_redis(con, &login_email_response_token);

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

        // return success
        res_body.has_totp = true;
        res_body.login_password_response_token = Some(token);
        return Ok(HttpResponse::Ok()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // generate tokens
    let auth_tokens: AuthTokens = match AuthTokens::new(user, remember_me).await {
        Ok(tokens) => tokens,
        Err(error) => {
            res_body.account_error = error;
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
    };

    // return success
    res_body.has_totp = false;
    res_body.auth_tokens = Some(auth_tokens);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("/totp")]
async fn post_totp(data: Json<LoginTotpRequest>, req: HttpRequest) -> Result<impl Responder> {
    let login_password_response_token: String = req
        .headers()
        .get("login_password_response_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let LoginTotpRequest { totp } = data.into_inner();
    let LoginTotpRequestSchema {
        totp,
        login_password_response_token,
    } = LoginTotpRequestSchema {
        totp,
        login_password_response_token,
    };
    let mut res_body: LoginTotpResponseSchema = LoginTotpResponseSchema::new();

    // Try to get TokenObject from redis
    let mut con = create_redis_client_connection();
    let (mut user, remember_me): (User, bool) =
        match get_user_remember_me_from_token_in_redis(con, &login_password_response_token) {
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
            Ok(user_remember_me) => (user_remember_me.user, user_remember_me.remember_me),
        };

    // check if the entered totp is a valid totp
    let validated_totp = validate_totp(&totp);
    if validated_totp.is_err() {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(validated_totp.err().unwrap()),
        };
        res_body.account_error = error;

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
    println!("totp: {:#?}", totp);

    // see if account has a totp
    let has_totp = user.is_totp_activated();
    if has_totp == false {
        let error: AccountError = AccountError {
            is_error: true,
            error_message: Some(String::from("User does not have totp activated")),
        };
        res_body.account_error = error;
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // check totp
    let is_totp_correct = user.check_totp(totp);
    if is_totp_correct == false {
        res_body.account_error = AccountError {
            is_error: true,
            error_message: Some(String::from("Incorrect totp")),
        };

        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }

    // delete old token from redis
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &login_password_response_token);

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

    // create auth tokens
    let auth_tokens: AuthTokens = match AuthTokens::new(user, remember_me).await {
        Ok(tokens) => tokens,
        Err(error) => {
            res_body.account_error = error;
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
    };

    // return success
    res_body.is_totp_correct = true;
    res_body.auth_tokens = Some(auth_tokens);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

#[post("/refresh-token")]
async fn refresh_token(data: Json<AuthTokens>) -> Result<impl Responder> {
    let mut res_body: RefreshTokenResponseSchema = RefreshTokenResponseSchema::new();
    let refresh_token: String = match &data.refresh_token {
        None => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(String::from("Internal server error")),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::Unauthorized()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Some(refresh_token) => refresh_token.to_string(),
    };

    let pool = create_pg_pool_connection().await;
    let user: User =
        match get_user_from_refresh_token_in_postgres_auth_table(&pool, &refresh_token).await {
            Ok(user) => user,
            Err(err) => {
                let error: AccountError = AccountError {
                    is_error: true,
                    error_message: Some(err.to_string()),
                };
                res_body.account_error = error;
                return Ok(HttpResponse::UnprocessableEntity()
                    .content_type("application/json; charset=utf-8")
                    .json(res_body));
            }
        };

    let access_token = AccessToken::new(&user);

    let auth_tokens: AuthTokens = AuthTokens {
        refresh_token: Some(refresh_token),
        access_token,
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(auth_tokens))
}

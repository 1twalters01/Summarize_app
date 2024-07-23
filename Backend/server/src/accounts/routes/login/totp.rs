use actix_web::{web::Json, HttpRequest, HttpResponse, Responder, Result};
// use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};

// mod request {include!(concat!(env!("OUT_DIR"), "/accounts/login/email/request.rs"));}
// mod response {include!(concat!(env!("OUT_DIR"), "/accounts/login/email/response.rs"));}

use crate::{
    accounts::{
        datatypes::users::User,
        queries::{
            postgres::get_user_from_refresh_token_in_postgres_auth_table,
            redis::get_user_remember_me_from_token_in_redis,
        },
        schema::{
            auth::{AccessToken, AuthTokens},
            errors::AccountError,
            login::{
                LoginTotpRequest,
                LoginTotpRequestSchema,
                LoginTotpResponseSchema,
            },
            refresh_token::RefreshTokenResponseSchema,
        },
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, delete_key_in_redis,
        },
        validations::validate_totp,
    },
};

pub async fn post_totp(data: Json<LoginTotpRequest>, req: HttpRequest) -> Result<impl Responder> {
    let login_password_token: String = req
        .headers()
        .get("login_password_token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let LoginTotpRequest { totp } = data.into_inner();
    let LoginTotpRequestSchema {
        totp,
        login_password_token,
    } = LoginTotpRequestSchema {
        totp,
        login_password_token,
    };
    let mut res_body: LoginTotpResponseSchema = LoginTotpResponseSchema::new();

    // Try to get TokenObject from redis
    let mut con = create_redis_client_connection();
    let (mut user, remember_me): (User, bool) =
        match get_user_remember_me_from_token_in_redis(con, &login_password_token) {
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
    let delete_redis_result = delete_key_in_redis(con, &login_password_token);

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

    // update last login time
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

    // delete old token
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &login_password_token);

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
    res_body.is_totp_correct = true;
    res_body.auth_tokens = Some(auth_tokens);
    Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body))
}

pub async fn refresh_token(data: Json<AuthTokens>) -> Result<impl Responder> {
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
            Ok(user) => match user {
                Some(user) => user,
                None => {
                    let error: AccountError = AccountError {
                        is_error: true,
                        error_message: Some("invalid refresh token".to_string()),
                    };
                    res_body.account_error = error;
                    return Ok(HttpResponse::UnprocessableEntity()
                        .content_type("application/json; charset=utf-8")
                        .json(res_body));
                }

            },
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


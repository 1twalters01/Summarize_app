use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use uuid::Uuid;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::{
        auth_tokens::AuthTokens,
        login::totp::{
            request::Request,
            response::{response::ResponseField, Error, Response},
        },
    },
    models::totp::Totp,
    queries::postgres::user::update::update_login_time_from_uuid,
    services::{
        cache_service::CacheService, response_service::ResponseService,
        token_service::TokenService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::totp::validate_totp,
    },
};

pub async fn post_totp(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let login_password_token: String = req
        .headers()
        .get("Login-Password-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Try to get user and remember_me status from redis
    let user_uuid: Uuid;
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result =
        cache_service.get_user_uuid_and_remember_me_from_token(&login_password_token);
    let (mut totp_struct, totp_activation_status, remember_me): (Totp, bool, bool) = match cache_result {
        Ok(Some((uuid, remember_me))) => {
            user_uuid = uuid;
            let user_service = UserService::new(create_pg_pool_connection().await);
            let totp_activation_status = match user_service
                .get_totp_activation_status_from_uuid(&user_uuid)
                .await {
                    Ok(totp_activation_status) =>  totp_activation_status,
                    Err(_) => {
                        return Ok(ResponseService::create_error_response(
                            AppError::LoginSms(Error::ServerError),
                            StatusCode::NOT_FOUND,
                        ));
                    },
                };
            let totp_struct_option = user_service
                .get_totp_status_from_uuid(&user_uuid)
                .await {
                    Ok(totp_struct_option) =>  totp_struct_option,
                    Err(_) => {
                        return Ok(ResponseService::create_error_response(
                            AppError::LoginSms(Error::ServerError),
                            StatusCode::NOT_FOUND,
                        ));
                    },
                };
            match totp_struct_option {
                Some(totp_struct) => (totp_struct, totp_activation_status, remember_me),
                None => {
                    return Ok(ResponseService::create_error_response(
                        AppError::LoginSms(Error::ServerError),
                        StatusCode::NOT_FOUND,
                    ));
                }
            }
        }
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginSms(Error::UserNotFound),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginSms(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    // Get totp from request
    let Request {
        digit1,
        digit2,
        digit3,
        digit4,
        digit5,
        digit6,
    } = data.0;

    // check if the entered totp is a valid totp
    let digits = &[digit1, digit2, digit3, digit4, digit5, digit6];
    if validate_totp(digits).is_err() || totp_activation_status == false {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::InvalidTotp),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // check if totp is correct
    if totp_struct
        .verify(digit1, digit2, digit3, digit4, digit5, digit6)
        .is_err()
    {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::IncorrectTotp),
            StatusCode::UNAUTHORIZED,
        ));
    }

    // create auth tokens
    let token_service = TokenService::from_uuid(&user_uuid);
    let refresh_token = token_service.generate_refresh_token();
    let access_token = token_service.generate_access_token().unwrap();

    let save_result = token_service
        .save_refresh_token_to_postgres(&refresh_token, remember_me)
        .await;
    if save_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // update last login time
    let pool = create_pg_pool_connection().await;
    if update_login_time_from_uuid(&pool, chrono::Utc::now(), &user_uuid)
        .await
        .is_err()
    {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    };


    // generate opaque token with prefix SITE_
    // save: con.set_ex(format!("session:{}", opaque_token), access_token, expiration as usize)

    // delete old token
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.delete_key(&login_password_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginTotp(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return opaque token to user
    let auth_tokens = AuthTokens {
        refresh: refresh_token,
        access: access_token,
    };
    println!("auth tokens: {:#?}", auth_tokens);
    return Ok(ResponseService::create_success_response(
        AppResponse::LoginTotp(Response {
            response_field: Some(ResponseField::Tokens(auth_tokens)),
        }),
        StatusCode::OK,
    ));
}

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};
use uuid::Uuid;

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::{
        auth_tokens::AuthTokens,
        login::password::{
            request::Request,
            response::{
                response::ResponseField, token::TokenField, Error, Response, Success, Token,
            },
        },
    },
    models::password::Password,
    queries::postgres::user::update::update_login_time_from_uuid,
    services::{
        cache_service::CacheService, response_service::ResponseService,
        token_service::TokenService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::password::validate_password,
    },
};

pub async fn post_password(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    // Check if ip has verified captcha

    let login_email_token: String = req
        .headers()
        .get("Login-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let user_uuid: Uuid;
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_uuid_from_token(&login_email_token);
    let (stored_password, totp_activation_status, sms_activation_status): (Password, bool, bool) = match cache_result {
        Ok(Some(uuid)) => {
            user_uuid = uuid;
            let user_service = UserService::new(create_pg_pool_connection().await);
            let totp_activation_status = match user_service
                .get_totp_activation_status_from_uuid(&user_uuid)
                .await {
                    Ok(totp_activation_status) =>  totp_activation_status,
                    Err(_) => {
                        return Ok(ResponseService::create_error_response(
                            AppError::LoginPassword(Error::ServerError),
                            StatusCode::NOT_FOUND,
                        ));
                    },
                };
            let sms_activation_status = match user_service
                    .get_totp_activation_status_from_uuid(&user_uuid)
                    .await {
                        Ok(sms_activation_status) =>  sms_activation_status,
                        Err(_) => {
                            return Ok(ResponseService::create_error_response(
                                AppError::LoginSms(Error::ServerError),
                                StatusCode::NOT_FOUND,
                            ));
                        },
                    };
                
            let password_option = user_service
                .get_password_from_uuid(&user_uuid)
                .await {
                    Ok(password_option) =>  password_option,
                    Err(_) => {
                        return Ok(ResponseService::create_error_response(
                            AppError::LoginPassword(Error::ServerError),
                            StatusCode::NOT_FOUND,
                        ));
                    },
                };
            match password_option {
                Some(password) => (password, totp_activation_status),
                None => {
                    return Ok(ResponseService::create_error_response(
                        AppError::LoginPassword(Error::ServerError),
                        StatusCode::NOT_FOUND,
                    ));
                }
            }
        }
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginPassword(Error::UserNotFound),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("Error, {:?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginPassword(Error::InvalidCredentials),
                StatusCode::UNPROCESSABLE_ENTITY,
            ));
        }
    };

    // if both activation statuses are true get the default mfa and set the other to false else set totp to false
    if totp_activation_status == true && sms_activation_status == true {
        let default_mfa_method: Option<String> = user_service.get_default_mfa_method();
        if let Some(default_mfa_method) == totp {
            sms_activation_status = false
        } else {
            totp_activation_status = false
        }
    }

    let Request {
        password,
        remember_me,
    } = data.0;

    if validate_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginPassword(Error::InvalidPassword),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    } else if stored_password.check_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginPassword(Error::IncorrectPassword),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let token_service = TokenService::from_uuid(&user_uuid);
    let app_response: AppResponse;

    if totp_activation_status == true || sms_activation_status == true {
        let token: String = token_service.generate_opaque_token_of_length(25);
        let user_uuid_and_remember_me_json =
            serde_json::to_string(&(user_uuid, remember_me)).unwrap();
        let expiry_in_seconds: Option<i64> = Some(300);
        let cache_result = cache_service.store_key_value(
            &token, // change to user_state:{}, token
            &user_uuid_and_remember_me_json,
            expiry_in_seconds,
        );
        if cache_result.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::LoginPassword(Error::ServerError),
                StatusCode::FAILED_DEPENDENCY,
            ));
        }

        if sms_activation_status == true {
            // generate otp
            // send text containing otp
            // save otp with same token under key sms:{}, token
        }

        app_response = AppResponse::LoginPassword(Response {
            response_field: Some(ResponseField::Success(Success {
                token: Some(Token {
                    token_field: Some(TokenField::Response(token)),
                }),
                requires_totp: true,
            })),
        });
    } else {
        let refresh_token = token_service.generate_refresh_token();
        let access_token = token_service.generate_access_token().unwrap();

        // If remember_me then save the refresh token for 7 days else save for 1 day
        let save_result = token_service
            .save_refresh_token_to_postgres(&refresh_token, remember_me)
            .await;
        if save_result.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::LoginPassword(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }

        // generate opaque token with prefix SITE_
        // save: con.set_ex(format!("session:{}", opaque_token), access_token, expiration as usize)
        let auth_tokens = AuthTokens {
            refresh: refresh_token,
            access: access_token,
        };

        // create app response containg opaque token instead of auth tokens
        app_response = AppResponse::LoginPassword(Response {
            response_field: Some(ResponseField::Success(Success {
                token: Some(Token {
                    token_field: Some(TokenField::Tokens(auth_tokens)),
                }),
                requires_totp: false,
            })),
        });

        // update last login time
        let pool = create_pg_pool_connection().await;
        if update_login_time_from_uuid(&pool, chrono::Utc::now(), &user_uuid)
            .await
            .is_err()
        {
            return Ok(ResponseService::create_error_response(
                AppError::LoginPassword(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        };
    }

    // delete old token
    let cache_result = cache_service.delete_key(&login_email_token);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginPassword(Error::ServerError),
            StatusCode::FAILED_DEPENDENCY,
        ));
    }

    // return success
    return Ok(ResponseService::create_success_response(
        app_response,
        StatusCode::OK,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use chrono::{DateTime, TimeDelta, Utc};
    use dotenv::dotenv;
    use prost::Message;
    use std::env;

    use crate::{
        datatypes::claims::UserClaims,
        generated::protos::accounts::{
            auth_tokens::AuthTokens,
            login::email::{
                request::Request as EmailRequest,
                response::{
                    response::ResponseField as EmailResponseField, Response as EmailResponse,
                },
            },
        },
        middleware::authentication::{AuthenticationMiddlewareFactory, NotAuthenticated},
        models::user::User,
        queries::postgres::refresh_token,
        views::accounts::login::email::post_email,
    };

    #[actix_web::test]
    async fn test_post_correct_password_no_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = EmailRequest { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = env::var("TEST_PASSWORD").unwrap();
        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Success(Success {
                token,
                requires_totp,
            }) = response_field
            {
                if let Some(Token { token_field }) = token {
                    match token_field.unwrap() {
                        TokenField::Response(token) => {
                            println!("requires totp: {}", requires_totp);
                            println!("response token: {}", token);
                            panic!("Should be access token but is instead response token")
                        }
                        TokenField::Tokens(AuthTokens { access, refresh }) => {
                            // get expiration timestamp of refresh token
                            let pool = create_pg_pool_connection().await;
                            let (created_at, expires_at): (DateTime<Utc>, DateTime<Utc>) =
                                refresh_token::get::created_at_and_expires_at_from_refresh_token(
                                    &pool, &refresh,
                                )
                                .await
                                .unwrap()
                                .expect("Invalid token");
                            // if expiration token more than 2 days
                            if expires_at - created_at != TimeDelta::days(1) {
                                println!("requires totp: {}", requires_totp);
                                println!("access token: {}", access);
                                println!("refresh token: {:?}", refresh);
                                println!("created_at: {:#?}", created_at);
                                println!("expires_at: {:#?}", expires_at);
                                panic!("Refresh token should last 1 day")
                            }

                            // check that uuid is the expected one
                            let secret = env::var("JWT_SECRET").unwrap();
                            let validation = jsonwebtoken::Validation::default();
                            let decoding_key =
                                jsonwebtoken::DecodingKey::from_secret(secret.as_ref());
                            let decoded = jsonwebtoken::decode::<UserClaims>(
                                &access,
                                &decoding_key,
                                &validation,
                            );
                            let user_uuid = env::var("TEST_UUID").unwrap();
                            if let Ok(token_data) = decoded {
                                assert!(user_uuid == token_data.claims.sub);
                            } else {
                                println!("{:#?}", decoded);
                                panic!("noo");
                            }
                        }
                    }
                }
            } else if let ResponseField::Error(error) = response_field {
                println!("error: {}", error);
                panic!("Should be a token but instead is an error");
            }
        } else if decoded.response_field == None {
            panic!("Should be a token but is instead None")
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_correct_password_with_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL_WITH_TOTP").unwrap();
        let req_message = EmailRequest { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = env::var("TEST_PASSWORD_WITH_TOTP").unwrap();
        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Success(Success {
                token,
                requires_totp,
            }) = response_field
            {
                if let Some(Token { token_field }) = token {
                    match token_field.unwrap() {
                        TokenField::Response(token) => {
                            println!("requires totp: {}", requires_totp);
                            println!("response token: {}", token);
                            panic!("Should be access token but is instead response token")
                        }
                        TokenField::Tokens(AuthTokens { access, refresh }) => {
                            let pool = create_pg_pool_connection().await;
                            let (created_at, expires_at): (DateTime<Utc>, DateTime<Utc>) =
                                refresh_token::get::created_at_and_expires_at_from_refresh_token(
                                    &pool, &refresh,
                                )
                                .await
                                .unwrap()
                                .expect("Invalid token");

                            if expires_at - created_at != TimeDelta::days(7) {
                                println!("requires totp: {}", requires_totp);
                                println!("access token: {}", access);
                                println!("refresh token: {:?}", refresh);
                                println!("created_at: {:#?}", created_at);
                                println!("expires_at: {:#?}", expires_at);
                                panic!("Refresh token should last a week")
                            }

                            // check that uuid is the expected one
                            let secret = env::var("JWT_SECRET").unwrap();
                            let validation = jsonwebtoken::Validation::default();
                            let decoding_key =
                                jsonwebtoken::DecodingKey::from_secret(secret.as_ref());
                            let decoded = jsonwebtoken::decode::<UserClaims>(
                                &access,
                                &decoding_key,
                                &validation,
                            );
                            let user_uuid = env::var("TEST_UUID_WITH_TOTP").unwrap();
                            if let Ok(token_data) = decoded {
                                assert!(user_uuid == token_data.claims.sub);
                            } else {
                                println!("{:#?}", decoded);
                                panic!("noo");
                            }

                            // Check that the refresh token is stored
                        }
                    }
                }
            } else if let ResponseField::Error(error) = response_field {
                println!("error: {}", error);
                panic!("Should be a token but instead is an error");
            }
        } else if decoded.response_field == None {
            panic!("Should be a token but is instead None");
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_wrong_password_no_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = EmailRequest { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("WrongPassword123");
        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Success(Success {
                token,
                requires_totp,
            }) = response_field
            {
                println!("totp status: {}", requires_totp);
                println!("token: {:?}", token);
                println!("Should be an error but instead is a token");
            } else if let ResponseField::Error(error) = response_field {
                assert!(error == Error::IncorrectPassword as i32);
            }
        } else if decoded.response_field == None {
            panic!("Should be a token but is instead None");
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_wrong_password_with_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL_WITH_TOTP").unwrap();
        let req_message = EmailRequest { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("WrongPassword123");
        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Success(Success {
                token,
                requires_totp,
            }) = response_field
            {
                println!("totp status: {}", requires_totp);
                println!("token: {:?}", token);
                println!("Should be an error but instead is a token");
            } else if let ResponseField::Error(error) = response_field {
                assert!(error == Error::IncorrectPassword as i32);
            }
        } else if decoded.response_field == None {
            panic!("Should be a token but is instead None");
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_password_no_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = EmailRequest { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("a");
        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Success(Success {
                token,
                requires_totp,
            }) = response_field
            {
                println!("totp status: {}", requires_totp);
                println!("token: {:?}", token);
                println!("Should be an error but instead is a token");
            } else if let ResponseField::Error(error) = response_field {
                assert!(error == Error::InvalidPassword as i32);
            }
        } else if decoded.response_field == None {
            panic!("Should be a token but is instead None");
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_password_with_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL_WITH_TOTP").unwrap();
        let req_message = EmailRequest { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("a");
        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Success(Success {
                token,
                requires_totp,
            }) = response_field
            {
                println!("totp status: {}", requires_totp);
                println!("token: {:?}", token);
                println!("Should be an error but instead is a token");
            } else if let ResponseField::Error(error) = response_field {
                assert!(error == Error::InvalidPassword as i32);
            }
        } else if decoded.response_field == None {
            panic!("Should be a token but is instead None");
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_correct_password_no_totp_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = EmailRequest {
            email: email.clone(),
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = env::var("TEST_PASSWORD").unwrap();

        let user: User = User::new(
            "username".to_string(),
            email,
            password.clone(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let user_uuid = user.get_uuid();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        let auth_token = String::from("Bearer ") + &access_token;

        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            println!("{:#?}", response_field);
            panic!("Should be None but is instead response field");
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_correct_password_with_totp_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL_WITH_TOTP").unwrap();
        let req_message = EmailRequest {
            email: email.clone(),
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = env::var("TEST_PASSWORD_WITH_TOTP").unwrap();

        let user: User = User::new(
            "username".to_string(),
            email,
            password.clone(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let user_uuid = user.get_uuid();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        let auth_token = String::from("Bearer ") + &access_token;

        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            println!("{:#?}", response_field);
            panic!("Should be None but is instead response field");
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_wrong_password_no_totp_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = EmailRequest {
            email: email.clone(),
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("WrongPassword123");

        let user: User = User::new(
            "username".to_string(),
            email,
            password.clone(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let user_uuid = user.get_uuid();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        let auth_token = String::from("Bearer ") + &access_token;

        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            println!("{:#?}", response_field);
            panic!("Should be None but is instead response field");
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_wrong_password_with_totp_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL_WITH_TOTP").unwrap();
        let req_message = EmailRequest {
            email: email.clone(),
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("WrongPassword123");

        let user: User = User::new(
            "username".to_string(),
            email,
            password.clone(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let user_uuid = user.get_uuid();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        let auth_token = String::from("Bearer ") + &access_token;

        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            println!("{:#?}", response_field);
            panic!("Should be None but is instead response field");
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_password_no_totp_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = EmailRequest {
            email: email.clone(),
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("a");

        let user: User = User::new(
            "username".to_string(),
            email,
            "Something123".to_string(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let user_uuid = user.get_uuid();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        let auth_token = String::from("Bearer ") + &access_token;

        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            println!("{:#?}", response_field);
            panic!("Should be None but is instead response field");
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_password_with_totp_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email))
                    .route("/password", web::post().to(post_password)),
            ),
        )
        .await;

        // Get token from email
        let email = env::var("TEST_EMAIL_WITH_TOTP").unwrap();
        let req_message = EmailRequest {
            email: email.clone(),
        };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let mut request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: EmailResponse = Message::decode(&response_buffer[..]).unwrap();

        let token = match decoded.response_field {
            None => panic!("Should be token but is none"),
            Some(response_field) => match response_field {
                EmailResponseField::Error(_) => panic!("Should be token but is error"),
                EmailResponseField::Token(token) => token,
            },
        };

        // post password
        let password = String::from("a");

        let user: User = User::new(
            "username".to_string(),
            email,
            password.clone(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let user_uuid = user.get_uuid();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        let auth_token = String::from("Bearer ") + &access_token;

        let remember_me = false;
        let req_message = Request {
            password,
            remember_me,
        };
        request_buffer = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        request = test::TestRequest::post()
            .uri("/login/password")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .append_header(("Login-Email-Token", token))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            println!("{:#?}", response_field);
            panic!("Should be None but is instead response field");
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }
}

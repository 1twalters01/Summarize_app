use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, HttpRequest, Responder, Result};

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
    queries::postgres::user::update::update_login_time,
    services::{
        cache_service::CacheService, response_service::ResponseService, token_service::TokenService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_password,
    },
};

pub async fn post_password(data: ProtoBuf<Request>, req: HttpRequest) -> Result<impl Responder> {
    let login_email_token: String = req
        .headers()
        .get("Login-Email-Token")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.get_user_from_token(&login_email_token);
    let user = match cache_result {
        Ok(Some(user)) => user,
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

    let Request {
        password,
        remember_me,
    } = data.0;

    if validate_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginPassword(Error::InvalidPassword),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    } else if user.check_password(&password).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginPassword(Error::IncorrectPassword),
            StatusCode::UNAUTHORIZED,
        ));
    }

    let user_uuid = user.get_uuid();
    let token_service = TokenService::from_uuid(&user_uuid);
    let app_response: AppResponse;

    if user.is_totp_activated() == true {
        let token: String = token_service.generate_opaque_token_of_length(25);
        let user_remember_me_json = serde_json::to_string(&(user, remember_me)).unwrap();
        let expiry_in_seconds: Option<i64> = Some(300);
        let cache_result =
            cache_service.store_key_value(&token, &user_remember_me_json, expiry_in_seconds);
        if cache_result.is_err() {
            return Ok(ResponseService::create_error_response(
                AppError::LoginPassword(Error::ServerError),
                StatusCode::FAILED_DEPENDENCY,
            ));
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
        let refresh_token = token_service.generate_refresh_token(remember_me);
        let access_token = token_service.generate_access_token().unwrap();

        // If remember_me then save the refresh token

        let auth_tokens = AuthTokens {
            refresh: refresh_token,
            access: access_token,
        };

        // create app response
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
        if update_login_time(&pool, chrono::Utc::now(), &user_uuid)
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
    use dotenv::dotenv;
    use prost::Message;
    use std::env;

    use crate::{
        generated::protos::accounts::{
            auth_tokens::AuthTokens,
            login::email::{
                request::Request as EmailRequest,
                response::{
                    response::ResponseField as EmailResponseField, Response as EmailResponse,
                },
            },
        },
        middleware,
        models::user::User,
        services::token_service::Claims,
        views::accounts::login::email::post_email,
    };

    #[actix_web::test]
    async fn test_post_correct_password_no_totp_while_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                            if let Some(ref refresh_token) = refresh {
                                println!("requires totp: {}", requires_totp);
                                println!("access token: {}", access);
                                println!("refresh token: {:?}", refresh_token);
                                panic!("Refresh token should be none")
                            }

                            // check that uuid is the expected one
                            let secret = env::var("JWT_SECRET").unwrap();
                            let validation = jsonwebtoken::Validation::default();
                            let decoding_key =
                                jsonwebtoken::DecodingKey::from_secret(secret.as_ref());
                            let decoded =
                                jsonwebtoken::decode::<Claims>(&access, &decoding_key, &validation);
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                            if refresh == None {
                                println!("requires totp: {}", requires_totp);
                                println!("access token: {}", access);
                                println!("refresh token: {:?}", refresh);
                                panic!("Refresh token should be Some(String)")
                            }

                            // check that uuid is the expected one
                            let secret = env::var("JWT_SECRET").unwrap();
                            let validation = jsonwebtoken::Validation::default();
                            let decoding_key =
                                jsonwebtoken::DecodingKey::from_secret(secret.as_ref());
                            let decoded =
                                jsonwebtoken::decode::<Claims>(&access, &decoding_key, &validation);
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
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

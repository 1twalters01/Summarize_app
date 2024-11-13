use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::login::email::{
        request::Request,
        response::{response::ResponseField, Error, Response},
    },
    models::user::User,
    services::{
        cache_service::CacheService, response_service::ResponseService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        tokens::generate_opaque_token_of_length,
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<Request>) -> Result<impl Responder> {
    // Get email from posted data
    let Request { email } = data.0;

    // Validate email
    if validate_email(&email).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginEmail(Error::InvalidEmail),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    // Get user from database
    let user_service = UserService::new(create_pg_pool_connection().await);
    let user: User = match user_service.get_user_from_email(&email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("No user found");
            return Ok(ResponseService::create_error_response(
                AppError::LoginEmail(Error::UnregisteredEmail),
                StatusCode::NOT_FOUND,
            ));
        },
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginEmail(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        },
    };

    // save {key: token, value: user} to redis cache for 300 seconds
    let token: String = generate_opaque_token_of_length(25);
    let expiry_in_seconds: Option<i64> = Some(300);

    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result = cache_service.store_token_for_user(&token, &user, expiry_in_seconds);
    if cache_result.is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginEmail(Error::ServerError),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    return Ok(ResponseService::create_success_response(
        AppResponse::LoginEmail(Response {
            response_field: Some(ResponseField::Token(token)),
        }),
        StatusCode::OK,
    ));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use dotenv::dotenv;
    use prost::Message;
    use std::env;

    use super::*;
    use crate::{datatypes::auth::AccessToken, middleware};

    #[actix_web::test]
    async fn test_known_email_not_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;

        let email = env::var("TEST_EMAIL").unwrap();
        let req_message = Request { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        assert!(true == false);
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Token(token) = response_field {
                assert!(token.len() == 25);
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
    async fn test_post_unknown_email_while_not_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;

        let email = String::from("fakeEmail@fake.com");
        let req_message = Request { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Token(token) = response_field {
                println!("Token: {}", token);
                panic!("Should be an error but instead is an token");
            } else if let ResponseField::Error(error) = response_field {
                assert!(error == Error::UnregisteredEmail as i32);
            }
        } else if decoded.response_field == None {
            panic!("Should be an error but is instead None")
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_email_while_not_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;

        let email = String::from("invalid");
        let req_message = Request { email };

        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("{:#?}", decoded);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Token(token) = response_field {
                println!("Token: {}", token);
                panic!("Should be an error but instead is an token");
            } else if let ResponseField::Error(error) = response_field {
                assert!(error == Error::InvalidEmail as i32);
            }
        } else if decoded.response_field == None {
            panic!("Should be an error but is instead None")
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_known_email_while_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;

        let email = env::var("TEST_EMAIL").unwrap();

        let user: User = User::new(
            "username".to_string(),
            email.clone(),
            "password123".to_string(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let token: String = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let req_message = Request { email };
        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("hi");
        println!("{:#?}", decoded.response_field);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Token(token) = response_field {
                println!("Token: {}", token);
                panic!("Should be None but is instead a token");
            } else if let ResponseField::Error(error) = response_field {
                println!("error: {}", error);
                panic!("Should be None but is instead an error");
            }
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_unknown_email_while_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;

        let email = String::from("fakeEmail@fake.com");

        let user: User = User::new(
            "username".to_string(),
            email.clone(),
            "password123".to_string(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let token: String = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let req_message = Request { email };
        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("hi");
        println!("{:#?}", decoded.response_field);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Token(token) = response_field {
                println!("Token: {}", token);
                panic!("Should be None but is instead a token");
            } else if let ResponseField::Error(error) = response_field {
                println!("error: {}", error);
                panic!("Should be None but is instead an error");
            }
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_email_while_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(middleware::authentication::not_authenticated::NotAuthenticated)
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;

        let email = String::from("invalid");

        let user: User = User::new(
            "username".to_string(),
            email.clone(),
            "password123".to_string(),
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let token: String = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let req_message = Request { email };
        let mut request_buffer: Vec<u8> = Vec::new();
        req_message.encode(&mut request_buffer).unwrap();

        let request = test::TestRequest::post()
            .uri("/login/email")
            .append_header(("Content-Type", "application/protobuf"))
            .append_header(("Authorization", auth_token.clone()))
            .set_payload(Bytes::from(request_buffer))
            .to_request();

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();
        println!("hi");
        println!("{:#?}", decoded.response_field);

        if let Some(response_field) = decoded.response_field {
            if let ResponseField::Token(token) = response_field {
                println!("Token: {}", token);
                panic!("Should be None but is instead a token");
            } else if let ResponseField::Error(error) = response_field {
                println!("error: {}", error);
                panic!("Should be None but is instead an error");
            }
        } else if decoded.response_field == None {
            assert!(true);
        } else {
            panic!("Error generating token");
        }
    }
}

use actix_protobuf::ProtoBuf;
use actix_web::{http::StatusCode, Responder, Result};

use crate::{
    datatypes::response_types::{AppError, AppResponse},
    generated::protos::accounts::login::email::{
        request::Request,
        response::{response::ResponseField, Error, Response},
    },
    services::{
        cache_service::CacheService, response_service::ResponseService,
        token_service::TokenService, user_service::UserService,
    },
    utils::{
        database_connections::{create_pg_pool_connection, create_redis_client_connection},
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<Request>) -> Result<impl Responder> {
    let Request { email } = data.0;
    if validate_email(&email).is_err() {
        return Ok(ResponseService::create_error_response(
            AppError::LoginEmail(Error::InvalidEmail),
            StatusCode::UNPROCESSABLE_ENTITY,
        ));
    }

    let user_service = UserService::new(create_pg_pool_connection().await);
    let user_uuid: uuid::Uuid = match user_service.get_user_uuid_from_email(&email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(ResponseService::create_error_response(
                AppError::LoginEmail(Error::UnregisteredEmail),
                StatusCode::NOT_FOUND,
            ));
        }
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginEmail(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    };

    let token_service = TokenService::new();
    let token: String = token_service.generate_opaque_token_of_length(25);
    let expiry_in_seconds: Option<i64> = Some(300);
    let mut cache_service = CacheService::new(create_redis_client_connection());
    let cache_result =
        cache_service.store_token_for_user_uuid(&token, &user_uuid, expiry_in_seconds);

    match cache_result {
        Ok(_) => {
            return Ok(ResponseService::create_success_response(
                AppResponse::LoginEmail(Response {
                    response_field: Some(ResponseField::Token(token)),
                }),
                StatusCode::OK,
            ));
        }
        Err(err) => {
            println!("{:#?}", err);
            return Ok(ResponseService::create_error_response(
                AppError::LoginEmail(Error::ServerError),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_http::Request as HttpRequest;
    use actix_web::{dev::ServiceResponse, test, web, App, Error as ActixError};
    use bytes::Bytes;
    use dotenv::dotenv;
    use prost::Message;
    use std::env;
    use uuid::Uuid;

    use super::*;
    use crate::{
        middleware::authentication::{AuthenticationMiddlewareFactory, NotAuthenticated},
        services::token_service::TokenService
    };

    async fn initialise_service(
    ) -> impl actix_web::dev::Service<HttpRequest, Response = ServiceResponse, Error = ActixError>
    {
        dotenv().ok();
        return test::init_service(
            App::new().service(
                web::scope("/login")
                    .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                    .route("/email", web::post().to(post_email)),
            ),
        )
        .await;
    }

    fn email_to_bytes(email: String) -> Bytes {
        let request_message = Request { email };
        let mut request_buffer: Vec<u8> = Vec::new();
        request_message.encode(&mut request_buffer).unwrap();
        Bytes::from(request_buffer)
    }

    fn post_test_request(request: Bytes, auth_token: Option<String>) -> HttpRequest {
        match auth_token {
            None => {
                return test::TestRequest::post()
                    .uri("/login/email")
                    .append_header(("Content-Type", "application/protobuf"))
                    .set_payload(request)
                    .to_request()
            }
            Some(token) => {
                return test::TestRequest::post()
                    .uri("/login/email")
                    .append_header(("Content-Type", "application/protobuf"))
                    .append_header(("Authorization", token))
                    .set_payload(request)
                    .to_request()
            }
        }
    }

    fn generate_auth_token() -> String {
        let user_uuid = Uuid::new_v4();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        String::from("Bearer ") + &access_token
    }

    #[actix_web::test]
    async fn test_known_email_not_authenticated() {
        let mut app = initialise_service().await;

        let email = env::var("TEST_EMAIL").unwrap();
        let request_bytes = email_to_bytes(email);
        let request = post_test_request(request_bytes, None);

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        match decoded.response_field {
            Some(ResponseField::Token(token)) => assert!(token.len() == 25),
            Some(ResponseField::Error(err)) => {
                println!("error: {}", err);
                panic!("Error instead of Token")
            }
            None => panic!("None instead of Token"),
        }
    }

    #[actix_web::test]
    async fn test_unknown_email_while_not_authenticated() {
        let mut app = initialise_service().await;

        let email = String::from("fakeEmail@fake.com");
        let request_bytes = email_to_bytes(email);
        let request = post_test_request(request_bytes, None);

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        match decoded.response_field {
            Some(ResponseField::Token(token)) => {
                println!("Token: {}", token);
                panic!("token instead of Error")
            }
            Some(ResponseField::Error(err)) => assert!(err == Error::UnregisteredEmail as i32),
            None => panic!("None instead of Error"),
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_email_while_not_authenticated() {
        let mut app = initialise_service().await;

        let email = String::from("invalid");
        let request_bytes = email_to_bytes(email);
        let request = post_test_request(request_bytes, None);

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        match decoded.response_field {
            Some(ResponseField::Token(token)) => {
                println!("Token: {}", token);
                panic!("token instead of Error")
            }
            Some(ResponseField::Error(err)) => assert!(err == Error::InvalidEmail as i32),
            None => panic!("None instead of Error"),
        }
    }

    #[actix_web::test]
    async fn test_post_known_email_while_authenticated() {
        let mut app = initialise_service().await;

        let email = env::var("TEST_EMAIL").unwrap();
        let request_bytes = email_to_bytes(email);
        let request = post_test_request(request_bytes, Some(generate_auth_token()));

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        match decoded.response_field {
            Some(ResponseField::Token(token)) => {
                println!("Token: {}", token);
                panic!("Token instead of None")
            }
            Some(ResponseField::Error(err)) => {
                println!("error: {}", err);
                panic!("Error instead of None")
            }
            None => assert!(true),
        }
    }

    #[actix_web::test]
    async fn test_post_unknown_email_while_authenticated() {
        let mut app = initialise_service().await;

        let email = String::from("fakeEmail@fake.com");
        let request_bytes = email_to_bytes(email);
        let request = post_test_request(request_bytes, Some(generate_auth_token()));

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        match decoded.response_field {
            Some(ResponseField::Token(token)) => {
                println!("Token: {}", token);
                panic!("Token instead of None")
            }
            Some(ResponseField::Error(err)) => {
                println!("error: {}", err);
                panic!("Error instead of None")
            }
            None => assert!(true),
        }
    }

    #[actix_web::test]
    async fn test_post_invalid_email_while_authenticated() {
        let mut app = initialise_service().await;

        let email = String::from("invalid");
        let request_bytes = email_to_bytes(email);
        let request = post_test_request(request_bytes, Some(generate_auth_token()));

        let resp = test::call_service(&mut app, request).await;
        let response_buffer: Vec<u8> = test::read_body(resp).await.to_vec();
        let decoded: Response = Message::decode(&response_buffer[..]).unwrap();

        match decoded.response_field {
            Some(ResponseField::Token(token)) => {
                println!("Token: {}", token);
                panic!("Token instead of None")
            }
            Some(ResponseField::Error(err)) => {
                println!("error: {}", err);
                panic!("Error instead of None")
            }
            None => assert!(true),
        }
    }
}

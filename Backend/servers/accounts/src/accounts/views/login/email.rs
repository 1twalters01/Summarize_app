use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpResponse, Responder, Result};

use crate::{
    accounts::{datatypes::users::User, queries::postgres::get_user_from_email_in_pg_users_table},
    generated::protos::accounts::login::email::{
        request::Request,
        response::{Error, Response, response::ResponseField},
    },
    utils::{
        database_connections::{
            create_pg_pool_connection, create_redis_client_connection, set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
        validations::validate_email,
    },
};

pub async fn post_email(data: ProtoBuf<Request>) -> Result<impl Responder> {
    // get request variable
    let Request { email } = data.0;

    // Validate the email from the request body
    let validated_email = validate_email(&email);
    if validated_email.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidEmail as i32)),
        };

        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // try to get the user from postgres using the email
    let pool = create_pg_pool_connection().await;
    let user_result: Result<Option<User>, sqlx::Error> =
        get_user_from_email_in_pg_users_table(&pool, &email).await;

    // if user does not exist or is none then return an error
    let user: User = match user_result {
        Err(err) => {
            println!("error: {:?}", err);

            let response: Response = Response {
                response_field: Some(ResponseField::Error(Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user_option) => match user_option {
            None => {
                let response: Response = Response {
                    response_field: Some(ResponseField::Error(
                        Error::UnregisteredEmail as i32,
                    )),
                };
                return Ok(HttpResponse::NotFound()
                    .content_type("application/x-protobuf; charset=utf-8")
                    .protobuf(response));
            }
            Some(user) => user,
        },
    };

    // create a token
    let token: String = generate_opaque_token_of_length(25);

    // serialize the user
    let user_json = serde_json::to_string(&user).unwrap();

    // save {key: token, value: user} to redis cache for 300 seconds
    let expiry_in_seconds: Option<i64> = Some(300);
    let con = create_redis_client_connection();
    let set_redis_result = set_key_value_in_redis(con, &token, &user_json, &expiry_in_seconds);

    // if redis fails then return an error
    if set_redis_result.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    let response: Response = Response {
        response_field: Some(ResponseField::Token(token)),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

#[cfg(test)]
mod tests {
    use std::env;
    use actix_web::{test, web, App};
    use dotenv::dotenv;
    use prost::Message;
    use bytes::Bytes;

    use super::{post_email, Error, Request, Response, ResponseField};
    use crate::{
        accounts::{datatypes::users::User, schema::auth::AccessToken},
        middleware,
    };

    #[actix_web::test]
    async fn test_post_known_email_while_not_authenticated() {
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

        let user: User = User::new("username".to_string(), email.clone(), "password123".to_string(), Some("First".to_string()), Some("Lastname".to_string())).unwrap();
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

        let user: User = User::new("username".to_string(), email.clone(), "password123".to_string(), Some("First".to_string()), Some("Lastname".to_string())).unwrap();
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

        let user: User = User::new("username".to_string(), email.clone(), "password123".to_string(), Some("First".to_string()), Some("Lastname".to_string())).unwrap();
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

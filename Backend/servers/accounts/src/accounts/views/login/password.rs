use actix_protobuf::{ProtoBuf, ProtoBufResponseBuilder};
use actix_web::{HttpRequest, HttpResponse, Responder, Result};

use crate::{
    accounts::{
        datatypes::{token_object::UserRememberMe, users::User},
        queries::redis::get_user_from_token_in_redis,
        schema::auth::AuthTokens,
    },
    generated::protos::accounts::{
        auth_tokens,
        login::password::{
            request::Request,
            response::{
                response::ResponseField, token::TokenField, Error, Response, Success, Token,
            },
        },
    },
    utils::{
        database_connections::{
            create_redis_client_connection, delete_key_in_redis, set_key_value_in_redis,
        },
        tokens::generate_opaque_token_of_length,
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

    let Request {
        password,
        remember_me,
    } = data.0;

    // try to get user from token in redis
    let mut con = create_redis_client_connection();
    let user: User = match get_user_from_token_in_redis(con, &login_email_token) {
        // if error return error
        Err(err) => {
            println!("Error, {:?}", err);

            let response: Response = Response {
                response_field: Some(ResponseField::Error(Error::InvalidCredentials as i32)),
            };
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
        Ok(user) => user,
    };

    // check if the entered password is a valid password
    let validated_password = validate_password(&password);
    if validated_password.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::InvalidPassword as i32)),
        };
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    println!("password: {:#?}", password);

    // check if password is correct for the given user
    let check_password: Result<(), std::io::Error> = user.check_password(&password);
    if check_password.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::IncorrectPassword as i32)),
        };
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

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
            let response: Response = Response {
                response_field: Some(ResponseField::Error(Error::ServerError as i32)),
            };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }

        // delete old token
        con = create_redis_client_connection();
        let delete_redis_result = delete_key_in_redis(con, &login_email_token);

        // if redis fails then return an error
        if delete_redis_result.await.is_err() {
            let response: Response = Response {
                response_field: Some(ResponseField::Error(Error::ServerError as i32)),
            };
            return Ok(HttpResponse::FailedDependency()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }

        // return success
        let response: Response = Response {
            response_field: Some(ResponseField::Success(Success {
                token: Some(Token {
                    token_field: Some(TokenField::Response(token)),
                }),
                requires_totp: true,
            })),
        };
        return Ok(HttpResponse::Ok()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }
    println!("inactive totp");

    // update last login time
    // generate tokens
    let auth_tokens: auth_tokens::AuthTokens = match AuthTokens::new(user, remember_me).await {
        Ok(tokens) => auth_tokens::AuthTokens {
            refresh: tokens.refresh_token,
            access: tokens.access_token.to_string(),
        },
        Err(err) => {
            println!("error: {:#?}", err);
            let response: Response = Response {
                response_field: Some(ResponseField::Error(Error::ServerError as i32)),
            };
            return Ok(HttpResponse::InternalServerError()
                .content_type("application/x-protobuf; charset=utf-8")
                .protobuf(response));
        }
    };
    println!("auth tokens: {:#?}", auth_tokens);

    // delete old token
    con = create_redis_client_connection();
    let delete_redis_result = delete_key_in_redis(con, &login_email_token);

    // if redis fails then return an error
    if delete_redis_result.await.is_err() {
        let response: Response = Response {
            response_field: Some(ResponseField::Error(Error::ServerError as i32)),
        };
        return Ok(HttpResponse::InternalServerError()
            .content_type("application/x-protobuf; charset=utf-8")
            .protobuf(response));
    }

    // return success
    let response: Response = Response {
        response_field: Some(ResponseField::Success(Success {
            token: Some(Token {
                token_field: Some(TokenField::Tokens(auth_tokens)),
            }),
            requires_totp: false,
        })),
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/x-protobuf; charset=utf-8")
        .protobuf(response));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use bytes::Bytes;
    use dotenv::dotenv;
    use prost::Message;
    use std::env;

    use crate::{
        accounts::{
            datatypes::users::User,
            schema::auth::AccessToken,
            views::login::{
                email::post_email,
                password::{
                    post_password, Error, Request, Response, ResponseField, Success, Token,
                    TokenField,
                },
            },
        },
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
                            let decoded = jsonwebtoken::decode::<
                                crate::accounts::schema::auth::Claims,
                            >(
                                &access, &decoding_key, &validation
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
                            let decoded = jsonwebtoken::decode::<
                                crate::accounts::schema::auth::Claims,
                            >(
                                &access, &decoding_key, &validation
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
        let req_message = EmailRequest { email: email.clone() };

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
        let access_token: String = AccessToken::new(&user).to_string();
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
        let req_message = EmailRequest { email: email.clone() };

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
        let access_token: String = AccessToken::new(&user).to_string();
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
        let req_message = EmailRequest { email: email.clone() };

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
        let access_token: String = AccessToken::new(&user).to_string();
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
        let req_message = EmailRequest { email: email.clone() };

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
        let access_token: String = AccessToken::new(&user).to_string();
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
        let req_message = EmailRequest { email: email.clone() };

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
        let access_token: String = AccessToken::new(&user).to_string();
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
        let req_message = EmailRequest { email: email.clone() };

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
        let access_token: String = AccessToken::new(&user).to_string();
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

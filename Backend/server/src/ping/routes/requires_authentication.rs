use crate::{
    accounts::schema::auth::Claims,
    ping::datatypes::{DualMessage, Message},
};
use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder, Result};

/// A get route function that requires the user to be authenticated
pub async fn ping_get_only_auth(req: HttpRequest) -> Result<impl Responder> {
    let message: Message = Message {
        message: String::from("Ping only authorized level from server"),
    };

    let claims: Claims = req.extensions().get::<Claims>().unwrap().clone();
    println!("Claims: {:#?}", claims);

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message));
}

/// A post route function that requires the user to be authenticated
pub async fn ping_post_only_auth(data: Json<Message>) -> Result<impl Responder> {
    let req_data: String = data.into_inner().message;

    let message_1: String = String::from("Ping only authorized level from server");
    let message_2: String = format!("Request data: {}", req_data);

    let dual_message: DualMessage = DualMessage {
        message_1,
        message_2,
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(dual_message));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use dotenv::dotenv;
    use serde_json::json;

    use crate::{
        accounts::{schema::auth::AccessToken, datatypes::users::User},
        middleware,
        ping::routes::requires_authentication::*,
    };

    #[actix_web::test]
    async fn test_ping_get_only_auth_not_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/ping")
                    .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
                    .route("/only_auth", web::get().to(ping_get_only_auth)),
            ),
        )
        .await;

        let request = test::TestRequest::get().uri("/ping/only_auth").to_request();
        let response = test::call_service(&mut app, request).await;
        println!("{}", response.status());
        assert!(response.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_ping_get_only_auth_authenticated() {
        dotenv().ok();

        let mut app = test::init_service(
            App::new().service(
                web::scope("/ping")
                    .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
                    .route("/only_auth", web::get().to(ping_get_only_auth)),
            ),
        )
        .await;

        let username = String::from("test123");
        let email = String::from("test123@gmail.com");
        let password = String::from("i23oj3rfw");

        let user: User = User::new(username, email, password).unwrap();
        let token: String = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let mut request = test::TestRequest::get()
            .uri("/ping/only_auth")
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::get()
            .uri("/ping/only_auth")
            .insert_header(("Authorization", auth_token))
            .to_request();
        response = test::call_service(&mut app, request).await;

        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({"message": "Ping only authorized level from server"})
        );
    }

    #[actix_web::test]
    async fn test_ping_post_only_auth_not_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/ping")
                    .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
                    .route("/only_auth", web::post().to(ping_post_only_auth)),
            ),
        )
        .await;

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/only_auth")
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_ping_post_only_auth_authenticated() {
        let mut app = test::init_service(
            App::new().service(
                web::scope("/ping")
                    .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
                    .route("/only_auth", web::post().to(ping_post_only_auth)),
            ),
        )
        .await;

        let username = String::from("test123");
        let email = String::from("test123@gmail.com");
        let password = String::from("i23oj3rfw");

        let user: User = User::new(username, email, password).unwrap();
        let token: String = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let mut request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/only_auth")
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();

        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/only_auth")
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        response = test::call_service(&mut app, request).await;

        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({
                "message_1": "Ping only authorized level from server",
                "message_2": format!("Request data: {}", data_text)
            })
        );
    }
}

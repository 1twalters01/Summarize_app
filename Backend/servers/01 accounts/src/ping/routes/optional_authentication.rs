use crate::ping::datatypes::{DualMessage, Message};
use actix_web::{web::Json, HttpResponse, Responder, Result};

/// A get route function that has no authentication requirements for the user
pub async fn ping_get_any_auth() -> Result<impl Responder> {
    let message: Message = Message {
        message: String::from("Ping any authorization level from server"),
    };

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message));
}

/// A post route function that has no authentication requirements for the user
pub async fn ping_post_any_auth(data: Json<Message>) -> Result<impl Responder> {
    println!("data: {:#?}", data);
    let req_data: String = data.into_inner().message;

    let message_1: String = String::from("Ping any authorization level from server");
    let message_2: String = format!("Request data: {}", req_data);

    let dual_message: DualMessage = DualMessage {
        message_1,
        message_2,
    };
    println!("dual_message: {:?}", dual_message);

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(dual_message));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use serde_json::json;

    use crate::{
        accounts::{datatypes::users::User, schema::auth::AccessToken},
        ping::routes::optional_authentication::*,
    };

    #[actix_web::test]
    async fn test_ping_get_any_auth_not_authenticated() {
        let mut app = test::init_service(
            App::new()
                .service(web::scope("/ping").route("/any_auth", web::get().to(ping_get_any_auth))),
        )
        .await;

        let mut request = test::TestRequest::get().uri("/ping/any_auth").to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::get().uri("/ping/any_auth").to_request();
        response = test::call_service(&mut app, request).await;
        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({"message": "Ping any authorization level from server"})
        );
    }

    #[actix_web::test]
    async fn test_ping_get_any_auth_authenticated() {
        let mut app = test::init_service(
            App::new()
                .service(web::scope("/ping").route("/any_auth", web::get().to(ping_get_any_auth))),
        )
        .await;

        let username = String::from("test123");
        let email = String::from("test123@gmail.com");
        let password = String::from("i23oj3rfw");

        let user: User = User::new(
            username,
            email,
            password,
            Some("First".to_string()),
            Some("Lastname".to_string()),
        )
        .unwrap();
        let token: String = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let mut request = test::TestRequest::get()
            .uri("/ping/any_auth")
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::get()
            .uri("/ping/any_auth")
            .insert_header(("Authorization", auth_token))
            .to_request();
        response = test::call_service(&mut app, request).await;
        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({"message": "Ping any authorization level from server"})
        );
    }

    #[actix_web::test]
    async fn test_ping_post_any_auth_not_authenticated() {
        let mut app =
            test::init_service(App::new().service(
                web::scope("/ping").route("/any_auth", web::post().to(ping_post_any_auth)),
            ))
            .await;

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let mut request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/any_auth")
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/any_auth")
            .to_request();
        response = test::call_service(&mut app, request).await;
        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({
                "message_1": "Ping any authorization level from server",
                "message_2": format!("Request data: {}", data_text)
            })
        );
    }

    #[actix_web::test]
    async fn test_ping_post_any_auth_authenticated() {
        let mut app =
            test::init_service(App::new().service(
                web::scope("/ping").route("/any_auth", web::post().to(ping_post_any_auth)),
            ))
            .await;

        let username = String::from("test123");
        let email = String::from("test123@gmail.com");
        let password = String::from("i23oj3rfw");
        let first_name = Some("First".to_string());
        let last_name = Some("Lastname".to_string());

        let user: User = User::new(username, email, password, first_name, last_name).unwrap();
        let token = AccessToken::new(&user).to_string();
        let auth_token = String::from("Bearer ") + &token;

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let mut request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/any_auth")
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::post()
            .set_json(&data)
            .uri("/ping/any_auth")
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        response = test::call_service(&mut app, request).await;
        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({
                "message_1": "Ping any authorization level from server",
                "message_2": format!("Request data: {}", data_text)
            })
        );
    }
}

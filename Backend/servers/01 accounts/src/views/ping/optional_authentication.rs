use crate::datatypes::ping::{DualMessage, Message};
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
    use super::*;
    use actix_http::Request as HttpRequest;
    use actix_web::{dev::ServiceResponse, test, web, App, Error as ActixError};
    use dotenv::dotenv;
    use serde_json::json;
    use uuid::Uuid;

    use crate::services::token_service::TokenService;

    async fn initialise_service(
    ) -> impl actix_web::dev::Service<HttpRequest, Response = ServiceResponse, Error = ActixError>
    {
        dotenv().ok();
        return test::init_service(
            App::new().service(
                web::scope("/ping")
                    .route("/get_any_auth", web::get().to(ping_get_any_auth))
                    .route("/post_any_auth", web::post().to(ping_post_any_auth)),
            ),
        )
        .await;
    }

    fn generate_auth_token() -> String {
        let user_uuid = Uuid::new_v4();
        let token_service = TokenService::from_uuid(&user_uuid);
        let access_token: String = token_service.generate_access_token().unwrap();
        String::from("Bearer ") + &access_token
    }

    #[actix_web::test]
    async fn test_ping_get_any_auth_not_authenticated() {
        let mut app = initialise_service().await;
        let uri = "/ping/get_any_auth";

        let mut request = test::TestRequest::get().uri(uri).to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::get().uri(uri).to_request();
        response = test::call_service(&mut app, request).await;
        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            res,
            json!({"message": "Ping any authorization level from server"})
        );
    }

    #[actix_web::test]
    async fn test_ping_get_any_auth_authenticated() {
        let mut app = initialise_service().await;
        let uri = "/ping/get_any_auth";

        let auth_token = generate_auth_token();

        let mut request = test::TestRequest::get()
            .uri(uri)
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::get()
            .uri(uri)
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
        let mut app = initialise_service().await;
        let uri = "/ping/post_any_auth";

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let mut request = test::TestRequest::post()
            .set_json(&data)
            .uri(uri)
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::post()
            .set_json(&data)
            .uri(uri)
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
        let mut app = initialise_service().await;
        let uri = "/ping/post_any_auth";

        let auth_token = generate_auth_token();

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let mut request = test::TestRequest::post()
            .set_json(&data)
            .uri(uri)
            .insert_header(("Authorization", auth_token.clone()))
            .to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::post()
            .set_json(&data)
            .uri(uri)
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

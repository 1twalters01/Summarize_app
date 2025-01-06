use crate::datatypes::{
    claims::UserClaims,
    ping::{DualMessage, Message},
};

use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder, Result};

/// A get route function that requires the user to be authenticated
pub async fn ping_get_only_auth(req: HttpRequest) -> Result<impl Responder> {
    let message: Message = Message {
        message: String::from("Ping only authorized level from server"),
    };

    let claims = req.extensions().get::<UserClaims>().unwrap().clone();
    println!("UserClaims: {:#?}", claims);

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
    use super::*;
    use actix_http::Request as HttpRequest;
    use actix_web::{dev::ServiceResponse, test, web, App, Error as ActixError};
    use dotenv::dotenv;
    use serde_json::json;
    use uuid::Uuid;

    use crate::{middleware, services::token_service::TokenService};

    async fn initialise_service(
    ) -> impl actix_web::dev::Service<HttpRequest, Response = ServiceResponse, Error = ActixError>
    {
        dotenv().ok();
        return test::init_service(
            App::new().service(
                web::scope("/ping")
                    .wrap(middleware::authentication::is_authenticated::IsAuthenticated)
                    .route("/get_only_auth", web::get().to(ping_get_only_auth))
                    .route("/post_only_auth", web::post().to(ping_post_only_auth)),
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
    async fn test_ping_get_only_auth_not_authenticated() {
        let mut app = initialise_service().await;
        let uri = "/ping/get_only_auth";

        let request = test::TestRequest::get().uri(uri).to_request();
        let response = test::call_service(&mut app, request).await;
        println!("{}", response.status());
        assert!(response.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_ping_get_only_auth_authenticated() {
        let mut app = initialise_service().await;
        let uri = "/ping/get_only_auth";

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
            json!({"message": "Ping only authorized level from server"})
        );
    }

    #[actix_web::test]
    async fn test_ping_post_only_auth_not_authenticated() {
        let mut app = initialise_service().await;
        let uri = "/ping/post_only_auth";

        let data_text: String = String::from("Ping from test");
        let data: Message = Message {
            message: data_text.clone(),
        };

        let request = test::TestRequest::post()
            .set_json(&data)
            .uri(uri)
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_ping_post_only_auth_authenticated() {
        let mut app = initialise_service().await;
        let uri = "/ping/post_only_auth";

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
                "message_1": "Ping only authorized level from server",
                "message_2": format!("Request data: {}", data_text)
            })
        );
    }
}

use actix_web::{get, post, HttpRequest, HttpResponse, Responder, Result, web::Json};
use crate::ping::datatypes::{Message, DualMessage};

// get (any auth)
#[get("ping/any_auth")]
async fn ping_get_any_auth() -> Result<impl Responder> {
    let message: Message = Message{
        message: String::from("Ping any authorization level from server")
    };

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message)
    )
}

// get (only auth)
#[get("ping/only_auth")]
async fn ping_get_only_auth() -> Result<impl Responder> {
    let message: Message = Message{
        message: String::from("Ping any authorization level from server")
    };

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message)
    )
}

// get (only not auth)
#[get("ping/not_auth")]
async fn ping_get_not_auth() -> Result<impl Responder> {
    let message: Message = Message{
        message: String::from("Ping any authorization level from server")
    };

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message)
    )
}

// post (any auth)
#[post("ping/any_auth")]
async fn ping_post_any_auth(data: Json<Message>) -> Result<impl Responder> {
    println!("data: {:#?}", data);
    let req_data: Message = data.into_inner();
    
    let message_1: String = String::from("Ping any authorization level from server");
    let message_2: String = format!("Request data: {}", req_data.message);
   
    let dual_message: DualMessage = DualMessage{
        message_1,
        message_2,
    };
    println!("dual_message: {:?}", dual_message);

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(dual_message)
    )
}

// post (only auth)
#[post("ping/only_auth")]
async fn ping_post_only_auth(data: Json<String>) -> Result<impl Responder> {
    let req_data: String = data.into_inner();
    
    let message_1: String = String::from("Ping only authorized level from server");
    let message_2: String = format!("Request data: {}", req_data);
    
    let dual_message: DualMessage = DualMessage{
        message_1,
        message_2,
    
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(dual_message)
    )
}

// post (only not auth)
#[post("ping/not_auth")]
async fn ping_post_not_auth(data: Json<String>) -> Result<impl Responder> {
    let req_data: String = data.into_inner();
    
    let message_1: String = String::from("Ping only not authorized level from server");
    let message_2: String = format!("Request data: {}", req_data);

    let dual_message: DualMessage = DualMessage{
        message_1,
        message_2,
    
    };
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(dual_message)
    )
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json::json;

    use super::*;

    #[actix_web::test]
    async fn test_ping_get_any_auth() {
        let mut app = test::init_service(App::new().service(ping_get_any_auth)).await;

        let mut request = test::TestRequest::get().uri("/ping/any_auth").to_request();
        let mut response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());

        request = test::TestRequest::get().uri("/ping/any_auth").to_request();
        response = test::call_service(&mut app, request).await;
        let res: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(res, json!({"message": "Ping any authorization level from server"}));
    }

    // #[actix_web::test]
    // fn test_ping_get_only_auth() {
    // }
    //
    // #[actix_web::test]
    // fn test_ping_get_not_auth() {
    // }

    #[actix_web::test]
    async fn test_ping_post_any_auth() {
        let mut app = test::init_service(App::new().service(ping_post_any_auth)).await;

        let data_text: String = String::from("Ping from test");
        let data: Message = Message { message: data_text.clone() };

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
        assert_eq!(res, json!({
            "message_1": "Ping any authorization level from server",
            "message_2": format!("Request data: {}", data_text)
        }));
    }

    // #[actix_web::test]
    // fn test_ping_post_only_auth() {
    // }
    //
    // #[actix_web::test]
    // fn test_ping_post_not_auth() {
    // }
}

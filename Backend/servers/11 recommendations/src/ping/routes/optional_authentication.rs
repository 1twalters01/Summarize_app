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

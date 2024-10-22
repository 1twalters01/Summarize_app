use crate::ping::datatypes::{DualMessage, Message};
use actix_web::{web::Json, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

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

use actix_web::{http::{header::ContentType, StatusCode}, web, HttpResponse, HttpResponseBuilder, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone)]
struct StripeEvent {
    #[serde(rename = "type")]
    event_type: String,
    data: serde_json::Value,
}

pub async fn handle_webhook(event: web::Json<StripeEvent>) -> Result<impl Responder> {
// pub async fn handle_webhook(event: web::Json<StripeEvent>) -> HttpResponse {
    match event.event_type.as_str() {
        "invoice.payment_succeeded" => {
            payment_success(event.clone().data);
            println!("Payment succeeded: {:?}", event.data);
            return Ok(HttpResponse::Ok());
        },
        "customer.deleted" => {
            customer_deleted(event.clone().data);
            return Ok(HttpResponse::Ok());
        },
        "customer.subscription.trial_will_end" => {
            subscription_trial_will_end(event.clone().data);
            return Ok(HttpResponse::Ok());
        },
        _ => {
            // let bad_request: HttpResponseBuilder = HttpResponse::BadRequest()
            //     .content_type(ContentType::plaintext())
            //     .body("Unhandled event type");

            let bad_request: HttpResponseBuilder = HttpResponse::BadRequest();

            return Ok(bad_request);
        },
    }
}


pub fn payment_success(data: serde_json::Value) {
    let customer_id = data["customer"].as_str().unwrap_or_default();
    let subscription_id = data["id"].as_str().unwrap_or_default();
}

pub fn customer_deleted(data: serde_json::Value) {
}

pub fn subscription_trial_will_end(data: serde_json::Value) {
}


use actix_web::{post, HttpRequest, HttpResponse, Responder, Result, web::Json};

// get (any auth)
#[get("ping/any_auth")]
async fn ping_get_any_auth() -> Result<impl Responder> {
    let message: String = String::from("Ping any authorization level from server");
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message)
    )
}

// get (only auth)
#[get("ping/only_auth")]
async fn ping_get_only_auth() -> Result<impl Responder> {
    let message: String = String::from("Ping only authorized level from server");
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message)
    )
}

// get (only not auth)
#[get("ping/not_auth")]
async fn ping_get_not_auth() -> Result<impl Responder> {
    let message: String = String::from("Ping only not authorized level from server");
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(message)
    )
}

// post (any auth)
#post("ping/any_auth")]
async fn ping_post_any_auth(data: Json<String>) -> Result<impl Responder> {
    let req_data: String = data.into_inner();
    let message_1: String = String::from("Ping any authorization level from server");
    let message_2: String = format!("Request data: {}", req_data);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json({
            "message_1": message_1,
            "message_2": message_2
        })
    )
}

// post (only auth)
#[post("ping/only_auth")]
async fn ping_post_only_auth(data: Json<String>) -> Result<impl Responder> {
    let req_data: String = data.into_inner();
    let message_1: String = String::from("Ping only authorized level from server");
    let message_2: String = format!("Request data: {}", req_data);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json({
            "message_1": message_1,
            "message_2": message_2
        })
    )
}

// post (only not auth)
#post("ping/not_auth")]
async fn ping_post_not_auth(data: Json<String>) -> Result<impl Responder> {
    let req_data: String = data.into_inner();
    let message_1: String = String::from("Ping only not authorized level from server");
    let message_2: String = format!("Request data: {}", req_data);
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json({
            "message_1": message_1,
            "message_2": message_2
        })
    )
}

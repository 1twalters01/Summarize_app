use actix_web::{web, HttpResponse, Responder, Result};

pub async fn refresh_token(data: web::Json<RefreshTokenQuery>) -> Result<impl Responder> {
}

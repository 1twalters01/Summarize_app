use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use chrono::Utc;
use futures_util::future::{ok, Ready};
use std::{future::Future, pin::Pin, rc::Rc};

use crate::services::token_service::TokenService;

pub struct IsVerified;
pub struct IsVerifiedMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Transform<S, ServiceRequest> for IsVerified
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = IsVerifiedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(IsVerifiedMiddleware {
            service: Rc::new(service),
        })
    }
}

impl<S, B> Service<ServiceRequest> for IsVerifiedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            let response: HttpResponse<EitherBody<B>>;

            if let Some(captcha_header) = req.headers().get("Captcha") {
                if let Ok(captcha_str) = captcha_header.to_str() {
                    if let Ok(claims) = TokenService::get_claims_from_captcha_token(captcha_str) {
                        let now = Utc::now().timestamp() as usize;
                        if claims.exp < now {
                            response = HttpResponse::Unauthorized()
                                .json(serde_json::json!({
                                    "error": "Invalid or expired token"
                                }))
                                .map_into_right_body();
                        } else {
                            let ip = req.peer_addr();
                            if Some(claims.ip) == ip {
                                let res = svc.call(req).await?;
                                return Ok(res.map_into_left_body());
                            } else {
                                response = HttpResponse::Unauthorized()
                                    .json(serde_json::json!({
                                        "error": "Invalid ip address"
                                    }))
                                    .map_into_right_body();
                            }
                        }
                    } else {
                        response = HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Invalid token claims"
                            }))
                            .map_into_right_body();
                    }
                } else {
                    response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": "Invalid captcha header"
                        }))
                        .map_into_right_body();
                }
            } else {
                response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "error": "Captcha header is missing"
                    }))
                    .map_into_right_body();
            }

            Ok(req.into_response(response))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::*;
    use crate::services::token_service::TokenService;
    use actix_http::{Request as HttpRequest, StatusCode};
    use actix_web::{
        dev::{Service, ServiceResponse},
        test,
        web::{self, get},
        App, Error as ActixError,
    };
    use dotenv::dotenv;
    use uuid::Uuid;

    async fn initialise_service(
    ) -> impl Service<HttpRequest, Response = ServiceResponse, Error = ActixError> {
        dotenv().ok();
        return test::init_service(
            App::new()
                .service(web::scope("/not-authenticated").route(
                    "/",
                    get().to(|| async {
                        actix_web::HttpResponse::Ok().body("not authenticated API")
                    }),
                ))
                .service(web::scope("/authenticated").wrap(IsVerified).route(
                    "/",
                    get().to(|| async { actix_web::HttpResponse::Ok().body("authenticated API") }),
                )),
        )
        .await;
    }

    #[actix_web::test]
    async fn test_authenticated_api_while_not_authenticated() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/authenticated/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::UNAUTHORIZED);

        let body = test::read_body(response).await;
        let text: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(text.get("error").unwrap() == "Captcha header is missing");
    }

    #[actix_web::test]
    async fn test_not_authenticated_api_while_not_authenticated() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get()
            .uri("/not-authenticated/")
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "not authenticated API");
    }

    #[actix_web::test]
    async fn test_authenticated_api_while_authenticated() {
        let mut app = initialise_service().await;
        let uuid = Uuid::new_v4();
        let mock_ip: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let token_service = TokenService::from_uuid(&uuid);
        let access_token = token_service.generate_captcha_token(mock_ip).unwrap();

        let request = test::TestRequest::get()
            .uri("/authenticated/")
            .peer_addr(mock_ip)
            .append_header(("Captcha", access_token))
            .to_request();
        let response = test::call_service(&mut app, request).await;
        println!("http status: {}", response.status());
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "authenticated API");
    }

    #[actix_web::test]
    async fn test_not_authenticated_api_while_authenticated() {
        let mut app = initialise_service().await;
        let uuid = Uuid::new_v4();
        let mock_ip: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let token_service = TokenService::from_uuid(&uuid);
        let access_token = token_service.generate_captcha_token(mock_ip).unwrap();

        let request = test::TestRequest::get()
            .uri("/not-authenticated/")
            .peer_addr(mock_ip)
            .append_header(("Captcha", format!("Bearer {}", access_token)))
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);
    }
}

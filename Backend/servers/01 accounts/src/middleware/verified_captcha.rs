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
                            if Some(claims.ip) != ip {
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

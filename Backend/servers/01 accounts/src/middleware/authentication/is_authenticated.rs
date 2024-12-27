use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use chrono::Utc;
use futures_util::future::{ok, Ready};
use std::{future::Future, pin::Pin, rc::Rc};

use crate::services::token_service::TokenService;

pub struct IsAuthenticated;

impl<S, B> Transform<S, ServiceRequest> for IsAuthenticated
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = IsAuthenticatedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(IsAuthenticatedMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct IsAuthenticatedMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for IsAuthenticatedMiddleware<S>
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

            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Ok(claims) = TokenService::get_claims_from_access_token(auth_str) {
                        let now = Utc::now().timestamp() as usize;
                        if claims.exp >= now {
                            req.extensions_mut().insert(claims);
                            let res = svc.call(req).await?;
                            return Ok(res.map_into_left_body());
                        } else {
                            response = HttpResponse::Unauthorized()
                                .json(serde_json::json!({
                                    "error": "Invalid or expired token"
                                }))
                            .map_into_right_body();
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
                            "error": "Invalid authorization header"
                        }))
                    .map_into_right_body();
                }
            } else {
                response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "Authorization header is missing"
                }))
                .map_into_right_body();
            }

            Ok(req.into_response(response))
        })
    }
}

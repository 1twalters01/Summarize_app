use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::HeaderValue,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::{env, future::Future, pin::Pin, rc::Rc};

pub struct IsAuthenticated;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

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
            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(token_data) = validate_auth_header(auth_header) {
                    req.extensions_mut().insert(token_data.claims);
                    let res = svc.call(req).await?;
                    return Ok(res.map_into_left_body());
                }
            }

            let response = HttpResponse::Unauthorized().finish().map_into_right_body();
            Ok(req.into_response(response))
        })
    }
}

pub fn validate_auth_header(auth_header: &HeaderValue) -> Result<TokenData<Claims>, ()> {
    let secret = env::var("JWT_SECRET").unwrap();
    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    if let Ok(auth_str) = auth_header.to_str() {
        if auth_str.starts_with("Bearer ") {
            let token = &auth_str[7..];

            if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
                return Ok(token_data);
            }
        }
    }

    return Err(());
}

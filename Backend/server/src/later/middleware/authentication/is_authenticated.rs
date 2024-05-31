use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpResponse,
};
use futures_util::future::{ok, Ready};
use std::task::{Context, Poll};
use std::rc::Rc;
use std::future::{ready, Future};
use std::pin::Pin;

use crate::accounts::auth::Claims;

pub struct IsAuthenticated;

impl<S, B> Transform<S, ServiceRequest> for IsAuthenticated
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            if let Some(auth_header) = req.headers().get("Authorization") {
                let validated_auth_header = validate_auth_header(auth_header);
                if validated_auth_header.is_ok() {
                    return svc.call(req).await;
                }
            }

            let response = HttpResponse::Unauthorized().finish().map_into_boxed_body();
            return Ok(req.into_response(response));
        })
    }
}

pub fn validate_auth_header(auth_header: String) -> Result<(), ()> {
    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    if auth_str.starts_with("Bearer ") {
    let token = &auth_header[7..];

        if let Ok(_token_data) = decode::<Claims>(token, &decoding_key, &validation) {
            return Ok(());
        }
    }

    return Err(());
}

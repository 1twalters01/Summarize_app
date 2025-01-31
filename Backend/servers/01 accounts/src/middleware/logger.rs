use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpResponse};
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures::Future;
use std::pin::Pin;
use std::rc::Rc;
use log::error;

pub struct LoggerMiddleware {
    enabled: bool,
};

pub struct LoggerMiddlewareService<S> {
    service: Rc<S>,
    enabled: bool,
}

impl<S, B> Transform<S, ServiceRequest> for LoggerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggerMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerMiddlewareService {
            service: Rc::new(service),
            enabled: self.enabled,
        })
    }
}

impl<S, B> Service<ServiceRequest> for LoggerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let res = svc.call(req).await;

            if enabled {
                match &res {
                    Ok(response) => {
                        let status = response.status();
                        if status.is_client_error() || status.is_server_error() {
                            error!(
                                "Error: Status Code = {}, Path = {}",
                                status,
                                response.request().path()
                            );
                        }
                    }
                    Err(err) => {
                        error!("Internal Error: {}", err);
                    }
                }
            }

            res
        })
    }
}

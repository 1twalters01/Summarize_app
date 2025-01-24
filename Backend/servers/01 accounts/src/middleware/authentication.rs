use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use chrono::Utc;
use futures_util::future::{ok, Ready};
use std::{any::TypeId, future::Future, pin::Pin, rc::Rc};

use crate::{datatypes::claims::UserClaims, services::token_service::TokenService};

pub trait AuthenticationBehaviour {
    fn is_request_allowed(auth_header: Option<&str>) -> Result<(), &str>;
}

pub struct NotAuthenticated;
impl AuthenticationBehaviour for NotAuthenticated {
    fn is_request_allowed(auth_header: Option<&str>) -> Result<(), &str> {
        if let Some(auth_str) = auth_header {
            // change to getting claims from bearer token
            if TokenService::get_claims_from_access_token(auth_str).is_ok() {
                return Err("Authenticated");
            } else {
                return Err("Invalid token claims");
            }
        } else {
            return Ok(());
        }
    }
}

pub struct Authenticated;
impl AuthenticationBehaviour for Authenticated {
    fn is_request_allowed(auth_header: Option<&str>) -> Result<(), &str> {
        if let Some(auth_str) = auth_header {
            if let Ok(claims) = TokenService::get_claims_from_bearer_token(auth_str) {
                let now = Utc::now().timestamp() as usize;
                if claims.exp >= now {
                    return Ok(());
                } else {
                    return Err("Invalid or expired token");
                }
            } else {
                return Err("Invalid token claims");
            }
        }
        return Err("No authentication header");
    }
}
impl Authenticated {
    fn get_claims(auth_header: Option<&str>) -> Result<UserClaims, String> {
        if let Some(auth_str) = auth_header {
            // change to getting claims from bearer token
            match TokenService::get_claims_from_bearer_token(auth_str) {
                Ok(claims) => return Ok(claims),
                Err(_) => return Err(String::from("Invalid token")),
            }
        }

        return Err(String::from("No token"));
    }
}

/* I am using a middleware factory so that external
   traits can be implemented on the factory rather than
   type 'T'. If you implement external traits on a generic
   then you get an error. See link below.

   https://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/
*/
pub struct AuthenticationMiddlewareFactory<T>
where
    T: AuthenticationBehaviour,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T> AuthenticationMiddlewareFactory<T>
where
    T: AuthenticationBehaviour,
{
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct AuthenticationMiddleware<S, T>
where
    T: AuthenticationBehaviour,
{
    service: Rc<S>,
    _marker: std::marker::PhantomData<T>,
}

// Implement `Transform` for the factory
impl<S, B, T> Transform<S, ServiceRequest> for AuthenticationMiddlewareFactory<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    T: AuthenticationBehaviour + 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S, T>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(service),
            _marker: std::marker::PhantomData,
        })
    }
}

// Implement `Service` for the middleware
impl<S, B, T> Service<ServiceRequest> for AuthenticationMiddleware<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    T: AuthenticationBehaviour + 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|header| header.to_str().ok());
            let is_request_allowed = T::is_request_allowed(auth_header);
            if is_request_allowed.is_ok() {
                if TypeId::of::<T>() == TypeId::of::<Authenticated>() {
                    req.extensions_mut()
                        .insert(Authenticated::get_claims(auth_header).ok());
                }

                let res = svc.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": is_request_allowed.err()
                }))
                .map_into_right_body();
            return Ok(req.into_response(response));
        })
    }
}

#[cfg(test)]
mod tests {
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
                .service(
                    web::scope("/not-authenticated")
                        .wrap(AuthenticationMiddlewareFactory::<NotAuthenticated>::new())
                        .route(
                            "/",
                            get().to(|| async {
                                actix_web::HttpResponse::Ok().body("not authenticated API")
                            }),
                        ),
                )
                .service(
                    web::scope("/authenticated")
                        .wrap(AuthenticationMiddlewareFactory::<Authenticated>::new())
                        .route(
                            "/",
                            get().to(|| async {
                                actix_web::HttpResponse::Ok().body("authenticated API")
                            }),
                        ),
                ),
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
        assert!(text.get("error").unwrap() == "No authentication header");
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
        let token_service = TokenService::from_uuid(&uuid);
        let access_token = token_service.generate_access_token().unwrap();
        // generate opaque token with prefix SITE_
        // save opaque token and access_token

        let request = test::TestRequest::get()
            .uri("/authenticated/")
            // opaque token, not access token
            .append_header(("Authorization", format!("Bearer {}", access_token)))
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
        let token_service = TokenService::from_uuid(&uuid);
        let access_token = token_service.generate_access_token().unwrap();
        // generate opaque token with prefix SITE_
        // save opaque token and access_token

        let request = test::TestRequest::get()
            .uri("/not-authenticated/")
            // opaque token, not access token
            .append_header(("Authorization", format!("Bearer {}", access_token)))
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::UNAUTHORIZED);

        let body = test::read_body(response).await;
        let text: serde_json::Value = serde_json::from_slice(&body).unwrap();
        println!("{:#?}", text);
        assert!(text.get("error").unwrap() == "Authenticated");
    }
}

use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::{
    rc::Rc,
    task::{Context, Poll},
};

use crate::{
    queries::redis::general::set_key_value_in_redis,
    utils::database_connections::create_redis_client_connection,
};
use redis::{Commands, Connection, RedisResult};

struct RateLimiter {
    limit: i64,
    expiry_in_seconds: Option<i64>,
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimiterMiddleware {
            service: Rc::new(service),
            limit: self.limit,
            expiry_in_seconds: self.expiry_in_seconds,
        })
    }
}

struct RateLimiterMiddleware<S> {
    service: Rc<S>,
    limit: i64,
    expiry_in_seconds: Option<i64>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    // type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // get ip
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        // query if ip is in cache
        let mut con = create_redis_client_connection();
        let ip_result: Result<Option<i64>, String> = get_count_from_ip_in_redis(&mut con, &ip);

        // if ip_result is error then return fail
        if ip_result.is_err() {
            return Box::pin(async {
                return Ok(req.into_response(
                    HttpResponse::InternalServerError()
                        .json(serde_json::json!({
                            "error": "Internal Server Error"
                        }))
                        .map_into_right_body(),
                ));
            });
        }

        let limit = self.limit;
        let expiry_in_seconds = self.expiry_in_seconds;
        match ip_result.ok().unwrap() {
            // if ip not in cache then set count to 1 and save to redis for x seconds then continue
            None => {
                let count = 1;

                match set_key_value_in_redis(&mut con, &ip, &count.to_string(), expiry_in_seconds) {
                    Ok(_) => {}
                    Err(_) => {
                        return Box::pin(async {
                            return Ok(req.into_response(
                                HttpResponse::InternalServerError()
                                    .json(serde_json::json!({
                                        "error": "Internal Server Error"
                                    }))
                                    .map_into_right_body(),
                            ));
                        })
                    }
                };

                let fut = self.service.call(req);

                return Box::pin(async move {
                    let res = fut.await?;
                    return Ok(res.map_into_left_body());
                });
            }

            // if ip less than limit then increase count by 1, save and then continue
            Some(mut count) if count <= limit => {
                println!("count: {}, limit: {}", count, limit);
                count += 1;

                match set_key_value_in_redis(&mut con, &ip, &count.to_string(), expiry_in_seconds) {
                    Ok(_) => {}
                    Err(_) => {
                        return Box::pin(async {
                            return Ok(req.into_response(
                                HttpResponse::InternalServerError()
                                    .json(serde_json::json!({
                                        "error": "Internal Server Error"
                                    }))
                                    .map_into_right_body(),
                            ));
                        })
                    }
                };

                let fut = self.service.call(req);

                return Box::pin(async move {
                    // let res = svc.call(req).await?;
                    let res = fut.await?;
                    return Ok(res.map_into_left_body());
                    // return Ok(res);
                });
            }

            Some(_) => {
                return Box::pin(async {
                    // sleep(Duration::from_secs(180)).await; // Optional delay before responding with error
                    return Ok(req.into_response(
                        HttpResponse::TooManyRequests()
                            .json(serde_json::json!({
                                "error": "Too many requests"
                            }))
                            .map_into_right_body(),
                    ));
                });
            }
        }
    }
}

pub fn get_count_from_ip_in_redis(con: &mut Connection, ip: &str) -> Result<Option<i64>, String> {
    let redis_result: RedisResult<Option<i64>> = con.get(ip);
    match redis_result {
        Ok(Some(res)) => return Ok(Some(res)),
        Ok(None) => return Ok(None),
        Err(err) => {
            println!("{:#?}", err);
            return Err(err.to_string());
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::{Request as HttpRequest, StatusCode};
    use actix_web::{
        dev::{Service, ServiceResponse},
        test,
        web::{self, get},
        App, Error as ActixError,
    };
    use dotenv::dotenv;
    use std::{thread, time};

    async fn initialise_service(
    ) -> impl Service<HttpRequest, Response = ServiceResponse, Error = ActixError> {
        dotenv().ok();
        return test::init_service(
            App::new()
                .service(
                    web::scope("/limited")
                        .wrap(RateLimiter {
                            limit: 3,
                            expiry_in_seconds: Some(5),
                        })
                        .route(
                            "/",
                            get()
                                .to(|| async { actix_web::HttpResponse::Ok().body("limited API") }),
                        ),
                )
                .service(web::scope("/unlimited").route(
                    "/",
                    get().to(|| async { actix_web::HttpResponse::Ok().body("unlimited API") }),
                )),
        )
        .await;
    }

    #[actix_web::test]
    async fn test_limited_api_while_under_rate_limit() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "limited API");
    }

    #[actix_web::test]
    async fn test_unlimited_api_while_under_rate_limit() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "unlimited API");
    }

    #[actix_web::test]
    async fn test_limited_api_while_at_rate_limit() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let response = test::call_service(&mut app, request).await;
        println!("{}", response.status());
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "limited API");
    }

    #[actix_web::test]
    async fn test_unlimited_api_while_at_rate_limit() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "unlimited API");
    }

    #[actix_web::test]
    async fn test_limited_api_while_over_rate_limit() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::TOO_MANY_REQUESTS);

        let body = test::read_body(response).await;
        let text: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(text.get("error").unwrap() == "Too many requests");
    }

    #[actix_web::test]
    async fn test_unlimited_api_while_over_rate_limit() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/unlimited/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "unlimited API");
    }

    #[actix_web::test]
    async fn test_limited_api_with_wait() {
        let mut app = initialise_service().await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let _response = test::call_service(&mut app, request).await;
        thread::sleep(time::Duration::from_secs(8));
        let request = test::TestRequest::get().uri("/limited/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status() == StatusCode::OK);

        let body = test::read_body(response).await;
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text == "limited API");
    }
}

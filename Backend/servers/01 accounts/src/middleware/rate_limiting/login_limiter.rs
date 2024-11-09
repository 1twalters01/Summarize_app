use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::time::Duration;
use std::{
    rc::Rc,
    task::{Context, Poll},
};
use tokio::time::sleep;

use crate::{
    queries::redis::general::set_key_value_in_redis,
    utils::database_connections::create_redis_client_connection,
};
use redis::{Commands, Connection, ErrorKind, RedisResult};

struct RateLimiter;

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
        })
    }
}

struct RateLimiterMiddleware<S> {
    service: Rc<S>,
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
                        .finish()
                        .map_into_right_body(),
                ));
            });
        }

        let limit = 5;

        match ip_result.ok().unwrap() {
            // if ip not in cache then set count to 1 and save to redis for x seconds then continue
            None => {
                let count = 1;

                let expiry_in_seconds = Some(60);
                match set_key_value_in_redis(&mut con, &ip, &count.to_string(), expiry_in_seconds) {
                    Ok(_) => {}
                    Err(_) => {
                        return Box::pin(async {
                            return Ok(req.into_response(
                                HttpResponse::InternalServerError()
                                    .finish()
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
            Some(mut count) if count < limit => {
                count += 1;

                let expiry_in_seconds = Some(60);
                match set_key_value_in_redis(&mut con, &ip, &count.to_string(), expiry_in_seconds) {
                    Ok(_) => {}
                    Err(_) => {
                        return Box::pin(async {
                            return Ok(req.into_response(
                                HttpResponse::InternalServerError()
                                    .finish()
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
                    sleep(Duration::from_secs(180)).await; // Optional delay before responding with error
                    return Ok(req.into_response(
                        HttpResponse::TooManyRequests()
                            .finish()
                            .map_into_right_body(),
                    ));
                });
            }
        }
    }
}

pub fn get_count_from_ip_in_redis(con: &mut Connection, ip: &str) -> Result<Option<i64>, String> {
    let redis_result: RedisResult<String> = con.get(ip);
    match redis_result {
        Ok(res) => match res.parse::<i64>() {
            Ok(res) => return Ok(Some(res)),
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => {
            match err.kind() {
                // Defo wrong, need to test what happens when trying to get a value that doesn't exist in the redis cache and change the error kind to that
                ErrorKind::ResponseError => return Ok(None),
                _ => return Err(err.to_string()),
            }
        }
    };
}

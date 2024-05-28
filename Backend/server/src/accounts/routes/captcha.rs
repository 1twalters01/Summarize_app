use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;

#[get("/captcha")]
async fn get_captcha() -> impl Result<Responder> {
    // generate captcha
    // get answer for captcha
    // generate 64 bit token
    // save { key: token, value: answer } to redis
    // send image and token
}


#[derive(Deserialize)]
struct CaptchaResponse {
    token: String,
    response: String,
}

#[post("/verify_captcha")]
async fn verify_captcha(data: web::Json<CaptchaResponse>) -> impl Result<Responder> {
    // Retrieve the solution from the session or database
    let con = create_redis_client_connection();
    let solution: User = match get_captcha_solution_from_token_in_redis(con, &data.token) {
        // if error return error
        Err(err) => {
            let error: AccountError = AccountError {
                is_error: true,
                error_message: Some(err),
            };
            res_body.account_error = error;
            return Ok(HttpResponse::UnprocessableEntity()
                .content_type("application/json; charset=utf-8")
                .json(res_body));
        }
        Ok(solution) => solution,
    };

    if data.response == solution {
        return Ok(HttpResponse::Ok());
    } else {
        return Ok(HttpResponse::Unauthorized());
    }
}

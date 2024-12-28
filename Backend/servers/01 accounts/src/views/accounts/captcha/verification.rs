use actix_web::{web, HttpResponse, Responder, Result};
use captcha::{
    filters::{Dots, Noise},
    Captcha,
};

pub async fn verify_captcha(data: web::Json<CaptchaResponse>) -> Result<impl Responder> {
    let CaptchaResponse { token, response } = data.into_inner();
    let mut res_body: CaptchaResponseSchema = CaptchaResponseSchema::new();

    // Retrieve the solution from the session or database
    let con = create_redis_client_connection();
    let solution: String = match get_code_from_token_in_redis(con, &token) {
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

    if response == solution {
        res_body.success = true;
        return Ok(HttpResponse::UnprocessableEntity()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    } else {
        return Ok(HttpResponse::Unauthorized()
            .content_type("application/json; charset=utf-8")
            .json(res_body));
    }
}
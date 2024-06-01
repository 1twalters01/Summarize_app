use crate::settings::schema::{
    ChangeThemeRequestStruct, ChangeThemeResponseStruct, GetThemeResponseStruct,
};
use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder, Result};


#[get("get-theme")]
async fn get_theme(req: HttpRequest) -> Result<impl Responder> {
    let res_body: GetThemeResponseStruct = GetThemeResponseStruct::new();
    // get user's device - from header? - this means no req_body
    // get user's theme for the device
    // if error when getting the user's theme then return error

    // return ok

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

#[post("change-theme")]
async fn change_theme(
    req_body: Json<ChangeThemeRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeThemeRequestStruct { theme } = req_body.into_inner();
    let res_body: ChangeThemeResponseStruct = ChangeThemeResponseStruct::new();

    // get user's device (linux app, windows app, mac app, android app, ios app, desktop website, mobile website)
    // set user's theme for the current device to what was entered
    // if setting error then return error

    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}


use crate::settings::schema::{
    ChangeLanguageRequestStruct,
    ChangeLanguageResponseStruct,
};
use actix_web::{post, web::Json, HttpRequest, HttpResponse, Responder, Result};


#[post("change-language")]
async fn change_language(
    req_body: Json<ChangeLanguageRequestStruct>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let ChangeLanguageRequestStruct { language } = req_body.into_inner();
    let res_body: ChangeLanguageResponseStruct = ChangeLanguageResponseStruct::new();

    // validate the language
    // update the user's language to the new one
    // if error when updating then error
    return Ok(HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(res_body));
}

use actix_web::{get, post,  HttpResponse, Responder, Result, web::{Json, Bytes}};
// use users::credentials::Credentials;
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};
// use users::user::User;

#[get("/get-routes/")]
async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 8] = [
        Route::from(
            String::from("/login/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Log in to Summarize")),
        Route::from(
            String::from("/login/totp/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Authenticate user")),
        Route::from(
            String::from("/logout/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Log out of Summarize")),
        Route::from(
            String::from("/register/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Registers a new, unactivated user")),
        Route::from(
            String::from("/activate/{uidb64}/{token}/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Activate a newly registered user")),
        Route::from(
            String::from("/username-reset/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Reset the user's username")),
        Route::from(
            String::from("/password-reset/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Send a password reset email")),
        Route::from(
            String::from("/password-reset/{uidb64}/{token}/"),
            Vec::from([Method::Get, Method::Post]),
            String::from("Reset a password for a user")),
    ]; 

    Ok(Json(routes))
}

#[get("/{param:.*?}")]
async fn main_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/main.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/104.bundle.js")]
async fn login() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/104.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/575.bundle.js")]
async fn login_totp() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/575.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[post("/login/2fa")]
async fn login_totp_post(req_body: actix_web::HttpRequest) -> Result<impl Responder> {
    println!("{:?}", req_body);

    Ok(Json(true))
}

#[get("/837.bundle.js")]
async fn logout() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/837.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/116.bundle.js")]
async fn register() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/116.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/452.bundle.js")]
async fn activate() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/116.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/18.bundle.js")]
async fn username_reset() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/18.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/615.bundle.js")]
async fn username_reset_token() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/615.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/767.bundle.js")]
async fn password_reset() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/767.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

#[get("/407.bundle.js")]
async fn password_reset_token() -> Result<impl Responder> {
    let script_path: PathBuf = "../content/dist/main/javascript/407.bundle.js".into();
    let script_data = Bytes::from(fs::read(&script_path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(script_data))
}

// #[get("/login/")]
// async fn login() -> Result
    // let user_obj: User = users::user::User::new(
    //     String::from("username"),
    //     String::from("email"),
    //     String::from("password")
    // ).unwrap();
    // println!("{:#?}", user_obj);
    //
    // println!("{:?}", std::time::SystemTime::now());
    //
    // println!("\ncheck password: {:?}", user_obj.check_password(String::from("password")));
    // println!("\ncheck password: {:?}", user_obj.check_password(String::from("passwords")));
    // Ok(Json(true))
// }

// #[get("/login/totp/")]
// async fn login_totp() -> Result<impl Responder> {
//     let script_path: PathBuf = "../../../Frontend/dist/login_totp.js".into();
//     let script_data = Bytes::from(fs::read(&script_path).unwrap());
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/javascript")
//         .body(script_data))
// }
//
// #[post("/login/totp/")]
// async fn login_totp_post(req_body: Json<TotpCredentials>) -> Result<impl Responder> {
//     let totp_credentials: totpCredentials = totpCredentials {
//         totp: req_body.into_inner().totp,
//         user_id: req_body.into_inner().user_id,
//         token: req_body.into_inner().token,
//         auth_token: req_body.into_inner().auth_code,
//     };
//
//     if totp_credentials.validate_credentials() == false { return Ok(Json(false)); }
//
//     let token = totp_credentials.login().unwrap();
// }
//
//
//
//
// #[get("/activate/{uidb64}/{token}")]
// async fn activate() -> Result<impl Responder> {
//     let script_path: PathBuf = "../../../Frontend/dist/register.js".into();
//     let script_data = Bytes::from(fs::read(&script_path).unwrap());
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/javascript")
//         .body(script_data))
// }
//
// #[post("/activate/{uidb64}/{token}")]
// async fn activate_post(req_body: Json<Credentials>) -> Result<impl Responder> {
//     let register_struct: RegisterStruct = RegisterStruct {
//         username: req_body.into_inner().username,
//         password: req_body.into_inner().password,
//         email: req_body.into_inner().email,
//     };
//
//     Ok(Json(true))
// }
//
// #[get("/password-reset/")]
// async fn password_reset() -> Result<impl Responder> {
//     let script_path: PathBuf = "../../../Frontend/dist/register.js".into();
//     let script_data = Bytes::from(fs::read(&script_path).unwrap());
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/javascript")
//         .body(script_data))
// }
//
// #[post("/password-reset/")]
// async fn password_reset_post(req_body: Json<Credentials>) -> Result<impl Responder> {
//     let register_struct: RegisterStruct = RegisterStruct {
//         username: req_body.into_inner().username,
//         password: req_body.into_inner().password,
//         email: req_body.into_inner().email,
//     };
//
//     Ok(Json(true))
// }
//
// #[get("/username-reset/")]
// async fn username_reset() -> Result<impl Responder> {
//     let script_path: PathBuf = "../../../Frontend/dist/register.js".into();
//     let script_data = Bytes::from(fs::read(&script_path).unwrap());
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/javascript")
//         .body(script_data))
// }
//
// #[get("/password-reset/{uidb64}/{token}")]
// async fn register() -> Result<impl Responder> {
//     let script_path: PathBuf = "../../../Frontend/dist/register.js".into();
//     let script_data = Bytes::from(fs::read(&script_path).unwrap());
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/javascript")
//         .body(script_data))
// }
//
// #[post("/password-reset/{uidb64}/{token}")]
// async fn register_post(req_body: Json<Credentials>) -> Result<impl Responder> {
//     let register_struct: RegisterStruct = RegisterStruct {
//         username: req_body.into_inner().username,
//         password: req_body.into_inner().password,
//         email: req_body.into_inner().email,
//     };
//
//     Ok(Json(true))
// }
//

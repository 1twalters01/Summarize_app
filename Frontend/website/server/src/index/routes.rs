use actix_web::{get,  HttpResponse, Responder, Result, web::{Json, Bytes}};
use std::{fs, path::PathBuf};
use crate::datatypes::route::{Route, Method};

// #[get("/get-routes/")]
pub async fn get_routes() -> Result<impl Responder> {
    let routes: [Route; 21] = [
        Route::from(
            String::from("/"),
            Vec::from([Method::Get]),
            String::from("Index page HTML")),
        Route::from(
            String::from("/index.css"),
            Vec::from([Method::Get]),
            String::from("Index page CSS")),
        Route::from(
            String::from("/index.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/pricing/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/pricing.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/web-clipper/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/web-clipper.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/mobile/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/mobile.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/desktop/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/download/desktop.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/library/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/library.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/community/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/community.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/terms/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/terms.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/privacy/"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
        Route::from(
            String::from("/privacy.js"),
            Vec::from([Method::Get]),
            String::from("Index page JS")),
    ]; 

    Ok(Json(routes))
}


#[get("/favicon.ico")]
async fn favicon() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/public/favicons/favicon.ico".into();
    let image_content = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(image_content))
}

#[get("/517.bundle.js")]
pub async fn index() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/517.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}

#[get("/index.css")]
async fn index_css() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/index.css".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/css; charset=UTF-8")
        .body(data))
}

#[get("/720.bundle.js")]
pub async fn pricing() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/720.bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript")
        .body(data))
}
#[get("/pricing/")]
async fn pricing_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/pricing.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/pricing.js")]
async fn pricing_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/pricing.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}


#[get("/library/")]
async fn library_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/library.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/library.js")]
async fn library_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/library.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/community/")]
async fn community_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/community.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/community.js")]
async fn community_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/community.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/sync/")]
async fn sync_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/sync.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/sync.js")]
async fn sync_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/sync.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
      .content_type("text/javascript; charset=UTF-8")
      .body(data))
}

#[get("/terms/")]
async fn terms_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/terms.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/terms.js")]
async fn terms_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/terms.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/terms/pdf")]
async fn terms_pdf() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/Terms Statement.pdf".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
      .content_type("application/pdf")
      .body(data))
}

#[get("/privacy/")]
async fn privacy_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/privacy.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/privacy.js")]
async fn privacy_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/privacy.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/privacy/pdf")]
async fn privacy_pdf() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/Privacy Statement.pdf".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
      .content_type("application/pdf")
      .body(data))
}

#[get("/blog/")]
async fn blog_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/blog.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/blog.js")]
async fn blog_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/blog.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/ai/")]
async fn ai_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/ai.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/ai.js")]
async fn ai_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/ai.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/releases/")]
async fn releases_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/releases.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/releases.js")]
async fn releases_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/home/releases.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

#[get("/email-us/")]
async fn email_us_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/website/home/email-us.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/email-us.js")]
async fn email_us_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/website/home/email-us.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}

// #[get("/{param:.*?}")]
pub async fn main_html() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/index.html".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(data))
}

#[get("/main.js")]
async fn main_js() -> Result<impl Responder> {
    let path: PathBuf = "../content/dist/main/javascript/bundle.js".into();
    let data = Bytes::from(fs::read(&path).unwrap());

    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=UTF-8")
        .body(data))
}


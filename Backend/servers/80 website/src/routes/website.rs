use crate::{views::website, middleware};
use actix_web::web::{get, post, scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    let website_dir: String = env::var("WEBSITE_DIR").unwrap();
    cfg.service(
        scope("")
            .route("/favicon.ico", get().to(website::favicon::favicon_ico))
            .route("/main.js", get().to(website::main_js::main_js))
            .route("/", get().to(website::main_html::main_html))
            .route("{param:.*[^.bundle.js]}", get().to(website::main_html::main_html))
    )
    .service(files::Files::new("", format!("{}/dist/main/javascript", website_dir)))
    .default_service(
        route().to(|| async {
            let website_dir: String = env::var("WEBSITE_DIR").unwrap();
            let path: PathBuf = format!("{}/dist/main/index.html", website_dir).into();
            let data = Bytes::from(fs::read(&path).unwrap());

            HttpResponse::Ok()
                .content_type("text/html; charset=UTF-8")
                .body(data)
        })
    )
}

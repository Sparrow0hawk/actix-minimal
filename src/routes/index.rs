use actix_web::{guard, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HelloData {
    pub text: String,
}

pub fn index_html_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index_html)));
}

pub fn index_json_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .guard(
                guard::All(guard::Header("Accept", "application/json"))
                    .and(guard::Not(guard::Header("Accept", "text/html"))),
            )
            .route(web::get().to(index_json)),
    );
}

async fn index_html() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world!</h1>")
}

async fn index_json() -> impl Responder {
    let hello = HelloData {
        text: String::from("Hello world!"),
    };
    web::Json(hello)
}

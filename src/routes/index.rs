use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HelloData {
    pub text: String,
}

pub async fn index_html() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world!</h1>")
}

pub async fn index_json() -> impl Responder {
    let hello = HelloData {
        text: String::from("Hello world!"),
    };
    web::Json(hello)
}

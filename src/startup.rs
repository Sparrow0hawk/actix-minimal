use std::net::TcpListener;

use crate::routes::{index_html, index_json, not_found};

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/")
                    .guard(
                        guard::Any(guard::Header("Accept", "*/*"))
                            .or(guard::Header("Accept", "text/html")),
                    )
                    .route("", web::get().to(index_html)),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Accept", "application/json"))
                    .guard(guard::Not(guard::Header("Accept", "text/html")))
                    .route("", web::get().to(index_json)),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .default_service(web::get().to(not_found))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

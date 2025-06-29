use std::net::TcpListener;

use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    Ok(
        HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
            .listen(listener)?
            .run(),
    )
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

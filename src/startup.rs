use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use crate::routes;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    Ok(
        HttpServer::new(|| App::new().route("/health_check", web::get().to(routes::health_check)))
            .listen(listener)?
            .run(),
    )
}

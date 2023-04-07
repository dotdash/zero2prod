use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> std::io::Result<Server> {
    let connection_pool = web::Data::new(connection_pool);

    Ok(HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run())
}

use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.1:{}", cfg.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind 127.1:8000.");

    let connection_pool = PgPool::connect(&cfg.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

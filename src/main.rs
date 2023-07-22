use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cfg = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.1:{}", cfg.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind 127.1:8000.");

    let connection_pool = PgPool::connect(&cfg.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

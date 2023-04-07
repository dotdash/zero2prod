use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
};

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.1:0").expect("Failed to bind random port on localhost.");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.1:{port}");

    let mut cfg = get_configuration().expect("Failed to read configuration.");
    cfg.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&cfg.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        connection_pool,
    }
}

async fn configure_database(cfg: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&cfg.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, cfg.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&cfg.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    connection_pool
}

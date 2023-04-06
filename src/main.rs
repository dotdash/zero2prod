use std::net::TcpListener;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.1:0").expect("Failed to bind 127.1:8000."))?.await
}

use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run(TcpListener::bind("127.1:8000").expect("Failed to bind port"))?.await
}

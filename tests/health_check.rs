use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.1:0").expect("Failed to bind random port on localhost.");

    let port = listener.local_addr().unwrap().port();
    tokio::spawn(zero2prod::startup::run(listener).expect("Failed to bind address."));

    format!("http://127.1:{port}")
}

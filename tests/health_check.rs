use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(zero2prod::run(listener).expect("Failed to listen on address"));
    format!("http://127.1:{port}")
}

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

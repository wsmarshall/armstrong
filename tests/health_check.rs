use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    //brings in reqwest to perform HTTP requests against the app
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    //Asserts
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    //retrieve OS assigned port
    let port = listener.local_addr().unwrap().port();
    let server = armstrong::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

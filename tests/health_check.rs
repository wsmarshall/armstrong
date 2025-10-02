#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn app");
    //brings in reqwest to perform HTTP requests against the app
    let client = reqwest::Client::new();
}

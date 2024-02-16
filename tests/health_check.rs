#[tokio::test]
async fn health_check_works() {
    spawn_app(); //.await.expect("Failed to spawn our app.");
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

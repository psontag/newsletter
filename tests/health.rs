use std::net::TcpListener;

use actix_web::error;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let server = newsletter::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=Jon%20Snow&email=jon.snow%40mail.com";

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let tests = vec![
        ("", "missing both name and email"),
        ("name=Jon%20Snow", "missing email"),
        ("email=jon.snow@mail.com", "missing name"),
    ];

    for (invalid_form, error_msg) in tests {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_form)
            .send()
            .await
            .expect("Failed request");

        assert_eq!(400, response.status().as_u16(), "No 400 with {}", error_msg);
    }
}

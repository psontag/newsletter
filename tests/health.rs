use std::net::TcpListener;

use newsletter::configuration::{get_configuration, DatabaseSettings};
use newsletter::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);

    let mut config = get_configuration().expect("Failed to read configuration.");
    config.database.name = Uuid::new_v4().to_string();

    let pool = configure_database(&config.database).await;
    let server = run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, pool }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.conn_str_base())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create DB");

    let pool = PgPool::connect(&config.conn_str())
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed migrations");

    pool
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=Jon%20Snow&email=jon.snow%40mail.com";

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.pool)
        .await
        .expect("Failed to query DB");

    assert_eq!(saved.email, "jon.snow@mail.com");
    assert_eq!(saved.name, "Jon Snow");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let tests = vec![
        ("", "missing both name and email"),
        ("name=Jon%20Snow", "missing email"),
        ("email=jon.snow@mail.com", "missing name"),
    ];

    for (invalid_form, error_msg) in tests {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_form)
            .send()
            .await
            .expect("Failed request");

        assert_eq!(400, response.status().as_u16(), "No 400 with {}", error_msg);
    }
}

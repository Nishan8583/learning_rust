//! tests/health_check.rs

use secrecy::ExposeSecret;
use secrecy::Secret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::sync::LazyLock;
use std::{fmt::format, net::TcpListener};
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("unable to bind at any port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut config = get_configuration().expect("issue while getting db config");
    config.database.database_name = Uuid::new_v4().to_string();
    let pool = configure_database(&config.database).await;

    let server = run(listener, pool.clone()).expect("failed to bind to address");
    let _ = tokio::spawn(server);
    TestApp {
        address: address,
        db_pool: pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let maintainance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: Secret::new("password".to_string()),
        ..config.clone()
    };

    let mut connection =
        PgPool::connect(&maintainance_settings.connection_string().expose_secret())
            .await
            .expect("unable to create connection");

    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
        .await
        .expect("failed");

    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed");

    return connection_pool;
}

#[tokio::test]
async fn test_health_check() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    println!("Lets get address {}", &test_app.address);
    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("failed to execute request");

    let status = response.status();
    let content_length = response.content_length();
    println!("{}", response.text().await.unwrap());
    assert!(status.is_success());
    assert_eq!(Some(0), content_length);
}

#[tokio::test]
async fn test_200_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=test%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute valid subscription");

    assert_eq!(200, response.status().as_u16());

    // query() returns a ananomous struct type generated at compile time the
    // columns as fields
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch from saved connection");

    assert_eq!(saved.email, "test@gmail.com");
}

#[tokio::test]
async fn test_400() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=test", "missing the email"),
        ("email=test%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        let code = response.status().as_u16();
        //let body = response.text().await.expect("could not extract text");
        //assert_eq!(error_message, body);
        assert_eq!(400, code);
    }
}

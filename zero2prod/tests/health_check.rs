//! tests/health_check.rs

use std::{fmt::format, net::TcpListener};

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("unable to bind at any port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to bind to address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn test_health_check() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    println!("Lets get address {}", address);
    let response = client
        .get(format!("{}/health_check", address))
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
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=test%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute valid subscription");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_400() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=test", "missing the email"),
        ("email=test%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app_address))
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

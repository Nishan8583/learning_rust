use axum::{routing::post, Router};
mod api;
mod auth;
mod db;
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/register_user", post(api::api::register_user))
        .route("/delete_user", post(api::api::delete_user))
        .route("/login_user", post(api::api::login_user));

    println!("Server will listen in port 8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

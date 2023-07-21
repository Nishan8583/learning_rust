use axum::{routing::post, Router};
mod api;
mod db;
#[tokio::main]
async fn main() {
    let app = Router::new().route("/register_user", post(api::api::register_user));

    println!("Server will listen in port 8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{http::StatusCode, Json};
use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserCreatedMessage {
    pub username: String,
}

pub async fn register_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserCreatedMessage>) {
    println!("Created user for {}", payload.username);

    let user_created = UserCreatedMessage {
        username: String::from(format!("{}", payload.username)),
    };

    (StatusCode::CREATED, Json(user_created))
}

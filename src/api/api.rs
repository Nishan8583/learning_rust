use crate::db::db;
use axum::{http::StatusCode, Json};
use serde_derive::{Deserialize, Serialize};
use warp::reject::PayloadTooLarge;

// CreateUserRequest is struct for user query to create a new password
#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// UserCreatedMessage struct for result
#[derive(Deserialize, Serialize)]
pub struct UserCreatedMessage {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteUserrequest {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

// register_user handles route for creating a user
// it creates a db conneciton, and creates a new user
pub async fn register_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserCreatedMessage>) {
    println!("Attempting to create user {}", payload.username);

    let user_created = UserCreatedMessage {
        username: String::from(format!("{}", payload.username)),
    };

    let db_conn = db::DBConn::new().await;
    match db_conn {
        Err(err) => {
            println!("Error while establishing conneciton to database {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(user_created));
        }
        Ok(mut conn) => {
            if let Err(err) = conn.register_user(&payload).await {
                println!("Error while creating user {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(user_created));
            }
        }
    }
    println!("User has been created");
    return (StatusCode::CREATED, Json(user_created));
}

pub async fn delete_user(
    Json(payload): Json<DeleteUserrequest>,
) -> (StatusCode, Json<UserCreatedMessage>) {
    println!("Attemtpint to delete the user {}", payload.username);

    let db_conn = db::DBConn::new().await;
    let user_delete = UserCreatedMessage {
        username: String::from(format!("{}", payload.username)),
    };
    match db_conn {
        Err(err) => {
            println!(
                "ERROR while attempting to delete the user {:?}",
                payload.username
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(user_delete));
        }
        Ok(mut conn) => {
            if let Err(err) = conn.delete_user(&payload).await {
                println!("ERROR while attemtping to delete user {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(user_delete));
            }
        }
    }

    println!("User delete success");
    return (StatusCode::CREATED, Json(user_delete));
}

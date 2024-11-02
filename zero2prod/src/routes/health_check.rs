use actix_web::{HttpResponse, Responder};

// Respponder is a trait, which requires certain functions to be completed so that a struct can be changed to HTTP response
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

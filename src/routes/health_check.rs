use actix_web::{HttpResponse, Responder};
use log::info;
pub async fn health_check() -> impl Responder {
    info!("Health check hit"); // Log message
    HttpResponse::Ok()
}

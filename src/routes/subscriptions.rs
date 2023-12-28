use actix_web::{web, HttpResponse};
use log::info;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    info!("Subscribing email: {} with name: {}", form.email, form.name); // Log message
    HttpResponse::Ok().finish()
}

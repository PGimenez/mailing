use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::{info, warn};
use serde::Deserialize;
use std::net::TcpListener;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    info!("Greeting request for name: {}", &name); // Log message
    format!("Hello {}!", &name)
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    info!("Subscribing email: {} with name: {}", form.email, form.name); // Log message
    HttpResponse::Ok().finish()
}

async fn health_check() -> impl Responder {
    info!("Health check hit"); // Log message
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let _ = env_logger::try_init_from_env(env_logger::Env::default().default_filter_or("info")); // Initialize the logger

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// Your main function here

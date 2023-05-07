use std::env;

use actix_web::{ HttpServer, HttpResponse, Responder, App, get, web };
use actix_files as fs;

#[derive(Clone)]
struct AppConfig {
    port: u16,
    ip: String,
    repo_path: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig {
        ip: env::var("HOSTNAME").unwrap_or("0.0.0.0".to_string()),
        port: env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap_or(8080),
        repo_path: env::var("REPO_PATH").unwrap_or("/fdroid".to_string()),
    };

    let app_config_clone = app_config.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_config.clone()))
            .service(health)
            .service(fs::Files::new("/fdroid", &app_config.repo_path))
    })
        .bind((app_config_clone.ip.as_str(), app_config_clone.port))?
        .run().await
}
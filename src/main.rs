use std::env;

use actix_web::{ HttpServer, HttpResponse, Responder, App, get, web, middleware::Logger };
use actix_files as fs;
use env_logger::Env;
use log::{ info, debug };

#[derive(Clone, Debug)]
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
    // Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // App Config
    let app_config = AppConfig {
        ip: env::var("RM_IP").unwrap_or("127.0.0.1".to_string()),
        port: env::var("RM_PORT").unwrap_or("80".to_string()).parse().unwrap_or(80),
        repo_path: env::var("RM_REPO_PATH").unwrap_or("/fdroid".to_string()),
    };

    let app_config_clone = app_config.clone();

    info!("Server started!");
    debug!("App Config: {:#?}", app_config);

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .app_data(web::Data::new(app_config.clone()))
            .wrap(logger)
            .service(health)
            .service(fs::Files::new("/fdroid", &app_config.repo_path))
    })
        .bind((app_config_clone.ip.as_str(), app_config_clone.port))?
        .run().await
}
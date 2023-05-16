mod utils;
mod routes;
mod guards;
mod repository;

use actix_web::middleware;
use actix_web::{ HttpServer, HttpResponse, Responder, App, get, web, middleware::Logger };
use actix_files as fs;
use env_logger::Env;
use log::{ info, debug };
use repository::Repository;

use crate::routes::config::{ get_config, post_config };
use crate::utils::app_config::AppConfig;
use crate::guards::auth_guard::AuthGuard;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Ok!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // App Config
    let app_config = AppConfig::from_env();

    let app_config_clone = app_config.clone();

    info!("Server started!");
    debug!("App Config: {:#?}", app_config);

    let fdroid_repository = web::Data::new(Repository::new(&app_config.repo_path));
    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            // provide app config
            .app_data(web::Data::new(app_config.clone()))
            // provide fdroid repository
            .app_data(fdroid_repository.clone())
            // normalize routes (add / to all routes)
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
            // add logger as middleware
            .wrap(logger)
            // health service for HEALTHCHECK in docker
            .service(health)
            // fdroid repo for fdroid
            .service(
                fs::Files
                    ::new("/fdroid", &app_config.repo_path)
                    .show_files_listing()
                    // remove acces to hidden files
                    .path_filter(|path, _| {
                        // files that shouldn't be accessible
                        let hidden_files = vec!["config.yml", "keystore.p12"];
                        path.file_name()
                            .map(|file_name| !hidden_files.contains(&file_name.to_str().unwrap()))
                            .unwrap_or(true)
                    })
            )
            // config services for manipulating fdroid config file
            .service(
                web::scope("/config").service(get_config).service(post_config).guard(AuthGuard)
            )
    })
        .bind((app_config_clone.ip.as_str(), app_config_clone.port))?
        .run().await
}
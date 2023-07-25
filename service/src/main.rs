use actix_files as fs;
use actix_multipart::form::MultipartFormConfig;
use actix_web::middleware;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use fdroid::repository::Repository;
use log::{debug, info};

use crate::guards::auth_guard::AuthGuard;
use crate::routes::app::*;
use crate::routes::config::{
  get_config, get_keystore, get_keystore_password, get_picture, post_config, upload_picture,
};
use crate::utils::app_config::{AppConfig, WrappedValue};

mod guards;
mod routes;
mod utils;

// TODO: refactor project structure and crate visibilities, maybe consider splitting into a library?
// TODO: update README.md

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

  let fdroid_repository = web::Data::new(Repository::new(app_config.repo_path.value().clone()));
  HttpServer::new(move || {
    let logger = Logger::default();

    App::new()
      // provide app config
      .app_data(web::Data::new(app_config.clone()))
      // provide fdroid repository
      .app_data(fdroid_repository.clone())
      // multipart config
      .app_data(MultipartFormConfig::default().total_limit(*app_config.max_payload_size.value()))
      // normalize routes (add / to all routes)
      .wrap(middleware::NormalizePath::new(
        middleware::TrailingSlash::Trim,
      ))
      // compress responses
      .wrap(middleware::Compress::default())
      // add logger as middleware
      .wrap(logger)
      // health service for HEALTHCHECK in docker
      .service(health)
      // fdroid repo for fdroid
      .service(
        fs::Files::new("/fdroid", app_config.repo_path.value())
          .show_files_listing()
          // remove acces to hidden files
          .path_filter(|path, _| {
            // files that shouldn't be accessible
            let hidden_files = vec!["config.yml", "keystore.p12"];
            path
              .file_name()
              .map(|file_name| !hidden_files.contains(&file_name.to_str().unwrap()))
              .unwrap_or(true)
          }),
      )
      // config services for manipulating fdroid config file
      .service(
        web::scope("/config")
          .service(get_config)
          .service(post_config)
          .service(get_keystore)
          .service(get_keystore_password)
          .service(upload_picture)
          .service(get_picture)
          .guard(AuthGuard),
      )
      // app services for manipulating apps
      .service(
        web::scope("/apps")
          .service(get_apps)
          .service(upload_app)
          .service(delete_app)
          .service(get_metadata)
          .service(delete_all)
          .service(cleanup_files)
          .service(update_metadata)
          .service(sign_app)
          .guard(AuthGuard),
      )
  })
  .bind((*app_config_clone.ip.value(), *app_config_clone.port.value()))?
  .run()
  .await
}

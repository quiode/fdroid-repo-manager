use actix_cors::Cors;
use actix_multipart::form::MultipartFormConfig;
use actix_web::{
  get, middleware, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use env_logger::Env;
use log::{debug, info};

use crate::routes::config::{
  get_config, get_keystore, get_keystore_password, get_picture, post_config, upload_picture,
};

use crate::guards::auth_guard::AuthGuard;
use crate::routes::app::{
  cleanup_files, delete_all, delete_app, get_apps, get_metadata, sign_app, update_metadata,
  upload_app,
};
use crate::utils::app_config::{AppConfig, WrappedValue};
use actix_files as fs;
use actix_files::NamedFile;
use fdroid::Repository;

mod guards;
mod routes;
mod utils;

// TODO: update README.md
// TODO: ci integration workflow (how to use the backend to upload a new app from a ci)

#[get("/health")]
async fn health() -> impl Responder {
  HttpResponse::Ok().body("Ok!")
}

#[post("/auth")]
async fn auth(config: web::Data<AppConfig>, body: String) -> impl Responder {
  if body == *config.admin_password.value() {
    HttpResponse::Ok().body("true")
  } else {
    HttpResponse::Ok().body("false")
  }
}

/// Returns the index.html file
async fn index() -> actix_web::Result<NamedFile> {
  let app_config = AppConfig::from_env();

  Ok(NamedFile::open(
    app_config.frontend_path.value().join("index.html"),
  )?)
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

  let fdroid_repository =
    web::Data::new(Repository::new(app_config.repo_path.value().clone()).unwrap());
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
      // add cors rules
      .wrap(Cors::default().allow_any_header().allow_any_method().allow_any_origin())
      .service(
        web::scope("/api")
          // health service for HEALTHCHECK in docker
          .service(health)
          // service for checking password
          .service(auth)
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
          ),
      )
      // fdroid repo for fdroid
      .service(
        fs::Files::new("/fdroid", app_config.repo_path.value())
          .show_files_listing()
          // remove access to hidden files
          .path_filter(|path, _| {
            // files that shouldn't be accessible
            let hidden_files = ["config.yml", "keystore.p12"];
            path
              .file_name()
              .map(|file_name| !hidden_files.contains(&file_name.to_str().unwrap()))
              .unwrap_or(true)
          }),
      )
      // Frontend
      .service(
        actix_files::Files::new("/", app_config.frontend_path.value()).index_file("index.html"),
      )
      // redirect all requests to the frontend if not used by backend
      .default_service(web::get().to(index))
  })
  .bind((*app_config_clone.ip.value(), *app_config_clone.port.value()))?
  .run()
  .await
}

#[cfg(test)]
mod test {

  #[test]
  fn builds() {
    // tests that this builds
  }
}

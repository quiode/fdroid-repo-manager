//! Route used to edit apps and their metadata

use crate::routes::FileUploadForm;
use crate::utils::error::*;
use crate::utils::persist_temp_file;
use actix_multipart::form::MultipartForm;
use actix_web::{
  delete, get, post, put,
  web::{self, Json},
  Responder,
};
use fdroid::repository::app_metadata::AppMetadata;
use fdroid::repository::Repository;
use log::{debug, info};

#[get("")]
async fn get_apps(repo: web::Data<Repository>) -> Result<impl Responder> {
  let apps = repo.get_apps()?;

  Ok(Json(apps))
}

#[post("")]
async fn upload_app(
  repo: web::Data<Repository>,
  form: MultipartForm<FileUploadForm>,
) -> Result<impl Responder> {
  let file_name = form.0.app.file_name.clone().unwrap_or("NONE".to_owned());
  info!("Uploading a new app: \"{}\"...", file_name);

  let file_path = persist_temp_file(form.0.app)?;
  repo.upload_app(&file_path)?;

  debug!("Finished uploading app: \"{}\"!", file_name);
  Ok("")
}

/// upload an apk and sign it
#[post("/sign")]
async fn sign_app(
  repo: web::Data<Repository>,
  form: MultipartForm<FileUploadForm>,
) -> Result<impl Responder> {
  let file_name = form.0.app.file_name.clone().unwrap_or("NONE".to_owned());
  info!("Uploading and Signing a new app: \"{}\"", file_name);

  let file_path = persist_temp_file(form.0.app)?;
  repo.sign_app(&file_path)?;

  Ok("")
}

#[delete("{apk_name}")]
async fn delete_app(
  path: web::Path<String>,
  repo: web::Data<Repository>,
) -> Result<impl Responder> {
  repo.delete_app(&path)?;

  Ok("")
}

/// Get the metadata for a package
#[get("/metadata/{package_name}")]
async fn get_metadata(
  path: web::Path<String>,
  repo: web::Data<Repository>,
) -> Result<impl Responder> {
  Ok(Json(repo.get_metadata(&path)?))
}

/// Update the metadata for a package
#[put("/metadata/{package_name}")]
async fn update_metadata(
  path: web::Path<String>,
  metadata: Json<AppMetadata>,
  repo: web::Data<Repository>,
) -> Result<impl Responder> {
  repo.set_metadata(&path, &metadata.0)?;

  Ok("")
}

/// Deletes all metadata and apk's
#[delete("")]
async fn delete_all(repo: web::Data<Repository>) -> Result<impl Responder> {
  repo.clear()?;

  Ok("")
}

/// Cleanup the repository
#[put("/cleanup")]
async fn cleanup_files(repo: web::Data<Repository>) -> Result<impl Responder> {
  repo.cleanup()?;

  Ok("")
}

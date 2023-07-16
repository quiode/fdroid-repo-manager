//! Route used to edit apps and their metadata

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
  delete, get, post, put,
  web::{self, Json},
  Responder, Result,
};
use log::debug;

use crate::repository::app_metadata::AppMetadata;
use crate::repository::Repository;

// TODO: sign apks
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
  debug!("Uploading a new app: \"{}\"...", file_name);

  repo.upload_app(form.0.app)?;

  debug!("Finished uploading app: \"{}\"!", file_name);
  Ok("")
}

/// upload an apk and sign it
#[post("")]
async fn sign_app(
  repo: web::Data<Repository>,
  form: MultipartForm<FileUploadForm>,
) -> Result<impl Responder> {
  let file_name = form.0.app.file_name.clone().unwrap_or("NONE".to_owned());
  debug!("Uploading and Signing a new app: \"{}\"", file_name);

  repo.sign_app(form.0.app)?;

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

#[derive(MultipartForm)]
pub struct FileUploadForm {
  app: TempFile,
}

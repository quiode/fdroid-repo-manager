//! Route used to edit apps and their metadata

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
  delete, get, post,
  web::{self, Json},
  Responder, Result,
};
use log::debug;

use crate::repository::Repository;

// TODO: update app metadata
// TODO: cleanup: "fdroid rewritemeta"
// TODO: categories management
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
  Ok("Ok")
}

#[delete("{apk_name}")]
async fn delete_app(
  path: web::Path<String>,
  repo: web::Data<Repository>,
) -> Result<impl Responder> {
  repo.delete_app(&path)?;

  Ok("Ok")
}

#[get("/metadata/{package_name}")]
async fn get_metadata(
  path: web::Path<String>,
  repo: web::Data<Repository>,
) -> Result<impl Responder> {
  Ok(Json(repo.get_metadata(&path)?))
}

/// Deletes all metadata and apk's
#[delete("/all")]
async fn delete_all(repo: web::Data<Repository>) -> Result<impl Responder> {
  repo.clear()?;

  Ok("Ok")
}

#[derive(MultipartForm)]
pub struct FileUploadForm {
  app: TempFile,
}

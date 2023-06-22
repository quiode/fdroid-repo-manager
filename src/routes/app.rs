//! Route used to edit apps and their metadata

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, Responder, Result};
use log::debug;

use crate::repository::Repository;

// TODO: add apps, get apps, update app metadata
// TODO: cleanup: "fdroid rewritemeta"
// TODO: "clean" (delete all metadatas and apk's)
// TODO: categories management
// TODO: sign apks
#[get("")]
async fn get_apps(repo: web::Data<Repository>) -> Result<impl Responder> {
  let apps = repo.get_apps()?;

  Ok(web::Json(apps))
}

#[post("")]
async fn upload_app(
  repo: web::Data<Repository>,
  form: MultipartForm<FileUploadForm>,
) -> Result<impl Responder> {
  debug!(
    "\"Uploading a new app: {}\"",
    form.0.app.file_name.clone().unwrap_or("NONE".to_owned())
  );

  repo.upload_app(form.0.app)?;

  Ok("Ok")
}

#[derive(MultipartForm)]
struct FileUploadForm {
  app: TempFile,
}

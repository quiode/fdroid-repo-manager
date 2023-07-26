//! Route used to edit the config.yml file#[get("")]

use crate::routes::FileUploadForm;
use crate::utils::error::*;
use crate::utils::persist_temp_file;
use actix_multipart::form::MultipartForm;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpRequest, Responder};
use fdroid::repository::{PublicConfig, Repository};
use log::debug;
use std::collections::HashMap;
use std::fs;

// TODO: update general info
// TODO: build apps using fdroid import, build
#[get("")]
async fn get_config(repo: web::Data<Repository>) -> Result<impl Responder> {
  let config = repo.get_public_config()?;

  Ok(Json(config))
}

#[post("")]
async fn post_config(
  repo: web::Data<Repository>,
  public_config: Json<PublicConfig>,
) -> Result<impl Responder> {
  repo.set_config(&public_config.0)?;

  Ok(public_config)
}

/// Returns the keystore as a file
#[get("/keystore")]
async fn get_keystore(request: HttpRequest, repo: web::Data<Repository>) -> Result<impl Responder> {
  debug!("Downloading keystore!");

  let keystore = actix_files::NamedFile::open_async(repo.keystore_path()).await?;

  Ok(keystore.into_response(&request))
}

/// returns the password of the keystore
#[get("/keystore/password")]
async fn get_keystore_password(repo: web::Data<Repository>) -> Result<impl Responder> {
  debug!("Downloading keystore password!");

  let password = repo.get_keystore_password()?;
  let mut map = HashMap::new();
  map.insert("password", password);
  Ok(Json(map))
}

/// Set the store picture
#[post("/picture")]
async fn upload_picture(
  repo: web::Data<Repository>,
  form: MultipartForm<FileUploadForm>,
) -> Result<impl Responder> {
  let file_path = persist_temp_file(form.0.app)?;
  repo.save_image(&file_path)?;
  fs::remove_file(file_path)?;

  Ok("")
}

/// Gets the store picture
#[get("/picture")]
async fn get_picture(request: HttpRequest, repo: web::Data<Repository>) -> Result<impl Responder> {
  debug!("Downloading Image!");

  let image = actix_files::NamedFile::open_async(repo.get_image_path()?).await?;

  Ok(image.into_response(&request))
}

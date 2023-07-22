//! Route used to edit the config.yml file#[get("")]

use actix_web::{get, post, web, Responder, Result};

use crate::repository::{config::PublicConfig, Repository};

// TODO: update general info
// TODO: backup keystore (get keystore and password),
// TODO: update store picture
// TODO: build apps using fdroid import, build
#[get("")]
pub(crate) async fn get_config(repo: web::Data<Repository>) -> Result<impl Responder> {
  let config = repo.get_public_config()?;

  Ok(web::Json(config))
}

#[post("")]
pub(crate) async fn post_config(
  repo: web::Data<Repository>,
  public_config: web::Json<PublicConfig>,
) -> Result<impl Responder> {
  repo.set_config(&public_config.0)?;

  Ok(public_config)
}

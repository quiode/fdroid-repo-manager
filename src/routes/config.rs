//! Route used to edit the config.yml file#[get("")]

use actix_web::{ get, web, Result, Responder, post };

use crate::{ repository::{ Repository, config::PublicConfig } };

// TODO: update general info, backup keystore (get keystore and password), update store picture
#[get("")]
async fn get_config(repo: web::Data<Repository>) -> Result<impl Responder> {
    let config = repo.get_public_config()?;

    Ok(web::Json(config))
}

#[post("")]
async fn post_config(
    repo: web::Data<Repository>,
    public_config: web::Json<PublicConfig>
) -> Result<impl Responder> {
    repo.set_config(&public_config.0)?;

    Ok(public_config)
}
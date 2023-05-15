//! Route used to edit the config.yml file#[get("")]

use actix_web::{ get, web, Result, Responder };

use crate::utils::{ config_editor::{ get_public_config }, app_config::AppConfig };

// TODO: update general info, backup keystore (get keystore and password), update store picture
#[get("")]
async fn get_config(app_config: web::Data<AppConfig>) -> Result<impl Responder> {
    let config = get_public_config(&(app_config.repo_path.clone() + "/config.yml"))?;

    Ok(web::Json(config))
}
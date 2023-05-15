//! Route used to edit the config.yml file
use actix_web::{ get, web::{ self, Json } };

use crate::utils::{ config_editor::{ self, ConfigFile }, app_config::AppConfig, error::Result };

// TODO: update general info, backup keystore (get keystore and password), update store picture

#[get("")]
async fn get_config(config: web::Data<AppConfig>) -> Result<Json<ConfigFile>> {
    let config = config_editor::get_config(&(config.repo_path.clone() + "/config.yml"));

    config.map(|config| web::Json(config))
}
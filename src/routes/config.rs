//! Route used to edit the config.yml file#[get("")]

use actix_web::{ get, web, Result, Responder, post };

use crate::utils::{
    config_editor::{ get_public_config, PublicConfig, set_config },
    app_config::AppConfig,
};

// TODO: update general info, backup keystore (get keystore and password), update store picture
#[get("")]
async fn get_config(app_config: web::Data<AppConfig>) -> Result<impl Responder> {
    let config = get_public_config(&(app_config.repo_path.clone() + "/config.yml"))?;

    Ok(web::Json(config))
}

#[post("")]
async fn post_config(
    app_config: web::Data<AppConfig>,
    public_config: web::Json<PublicConfig>
) -> Result<impl Responder> {
    set_config(&(app_config.repo_path.clone() + "/config.yml"), &public_config.0)?;

    Ok(public_config)
}
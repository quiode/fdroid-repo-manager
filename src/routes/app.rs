//! Route used to edit apps and their metadata

use actix_web::{get, web, Result, Responder};

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
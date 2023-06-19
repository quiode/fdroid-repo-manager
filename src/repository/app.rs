use serde::Serialize;

use crate::utils::error::{Error, Result};

use super::Repository;

#[derive(Clone, Serialize)]
pub struct App {
  categories: Vec<String>,
  suggested_version_code: String,
  license: String,
  name: String,
  added: i64,
  package_name: String,
  last_updated: i64,
  packages: Vec<Package>,
}

impl App {
  fn from_json(value: serde_json::Value) -> Option<Vec<Self>> {
    let apps = value.get("apps")?;
    let packages = value.get("packages")?;

    let mut apps_vec = vec![];

    for app in apps.as_array()? {
      let name = app.get("name")?.as_str()?.to_owned();
      let suggested_version_code = app.get("suggestedVersionCode")?.as_str()?.to_owned();
      let license = app.get("license")?.as_str()?.to_owned();
      let package_name = app.get("packageName")?.as_str()?.to_owned();
      let last_updated = app.get("lastUpdated")?.as_i64()?.to_owned();
      let added = app.get("added")?.as_i64()?.to_owned();

      let mut categories = vec![];

      let mut packages = vec![];

      // TODO
      todo!();

      apps_vec.push(App {
        name,
        suggested_version_code,
        license,
        package_name,
        last_updated,
        added,
        packages,
        categories,
      });
    }

    Some(apps_vec)
  }
}

#[derive(Clone, Serialize)]
pub struct Package {
  added: i64,
  apk_name: String,
  hash: String,
  hash_type: String,
  max_sdk_version: u32,
  min_sdk_version: u32,
  nativecode: Vec<String>,
  package_name: String,
  sig: String,
  signer: String,
  size: u64,
  target_sdk_version: u32,
  uses_permission: Vec<(String, u32)>,
  uses_permission_sdk23: Vec<(String, u32)>,
  version_code: u64,
  version_name: String,
}

impl TryFrom<serde_json::Value> for Package {
  type Error = Error;

  fn try_from(value: serde_json::Value) -> Result<Self> {
    todo!()
  }
}

impl Repository {
  pub fn get_apps(&self) -> Result<Vec<App>> {
    App::from_json(
      serde_json::from_str(self.path.join("/repo/index-v1.json").to_str().ok_or(
        Error::JsonConvertError("Could not read repository index file!".to_owned()),
      )?)
      .map_err(|_| Error::JsonConvertError("Could not read repository index file!".to_owned()))?,
    )
    .ok_or(Error::JsonConvertError(
      "Could not read repository index file!".to_owned(),
    ))
  }
}

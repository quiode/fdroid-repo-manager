use serde::Serialize;

use crate::utils::error::{Error, Result};

use super::Repository;

#[derive(Clone, Serialize)]
pub struct App {
  categories: Vec<String>,
  suggestedVersionCode: String,
  license: String,
  name: String,
  added: i64,
  packageName: String,
  lastUpdated: i64,
  packages: Vec<Package>,
}

impl App {
  fn from_json(value: serde_json::Value) -> Option<Vec<Self>> {
    let apps = value.get("apps")?;
    let packages = value.get("packages")?;

    let mut apps_vec = vec![];

    for app in apps.as_array()? {
      let name = app.get("name")?.as_str()?.to_owned();
      let suggestedVersionCode = app.get("suggestedVersionCode")?.as_str()?.to_owned();
      let license = app.get("license")?.as_str()?.to_owned();
      let packageName = app.get("packageName")?.as_str()?.to_owned();
      let lastUpdated = app.get("lastUpdated")?.as_i64()?.to_owned();
      let added = app.get("added")?.as_i64()?.to_owned();

      let mut categories = vec![];

      let mut packages = vec![];

      // TODO
      todo!();

      apps_vec.push(App {
        name,
        suggestedVersionCode,
        license,
        packageName,
        lastUpdated,
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
  apkName: String,
  hash: String,
  hashType: String,
  maxSdkVersion: u32,
  minSdkVersion: u32,
  nativecode: Vec<String>,
  packageName: String,
  sig: String,
  signer: String,
  size: u64,
  targetSdkVersion: u32,
  usesPermission: Vec<(String, u32)>,
  usesPermissionSdk23: Vec<(String, u32)>,
  versionCode: u64,
  versionName: String,
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

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
  /// Reads a Json Value and tries to extract all fields to create an instance of App
  /// 
  /// returns None if any field can't be converted
  fn from_json(value: &serde_json::Value) -> Option<Vec<Self>> {
    // get both lists
    let apps = value.get("apps")?;
    let packages = value.get("packages")?;

    let mut apps_vec = vec![];

    // map all app fields
    for app in apps.as_array()? {
      let name = app.get("name")?.as_str()?.to_owned();
      let suggested_version_code = app.get("suggestedVersionCode")?.as_str()?.to_owned();
      let license = app.get("license")?.as_str()?.to_owned();
      let package_name = app.get("packageName")?.as_str()?.to_owned();
      let last_updated = app.get("lastUpdated")?.as_i64()?.to_owned();
      let added = app.get("added")?.as_i64()?.to_owned();

      let mut categories = vec![];

      // get all categories (are saved in a map)
      for category in app.get("categories")?.as_object()?.values() {
        categories.push(category.as_str()?.to_string());
      }

      let mut packages_vec = vec![];

      let package = packages.get(&package_name)?;

      // map all package fields
      for package_entry in package.as_object()?.values() {
        packages_vec.push(Package::from_json(package_entry)?);
      }

      apps_vec.push(App {
        name,
        suggested_version_code,
        license,
        package_name,
        last_updated,
        added,
        packages: packages_vec,
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

impl Package {
  fn from_json(value: &serde_json::Value) -> Option<Self> {
    // TODO
    todo!()
  }
}

impl Repository {
  pub fn get_apps(&self) -> Result<Vec<App>> {
    App::from_json(
      &serde_json::from_str(self.path.join("/repo/index-v1.json").to_str().ok_or(
        Error::JsonConvertError("Could not read repository index file!".to_owned()),
      )?)
      .map_err(|_| Error::JsonConvertError("Could not read repository index file!".to_owned()))?,
    )
    .ok_or(Error::JsonConvertError(
      "Could not read repository index file!".to_owned(),
    ))
  }
}

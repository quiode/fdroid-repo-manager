use std::{
  fs::{self, File},
  io::{self, Read},
  path::PathBuf,
};

use actix_multipart::form::tempfile::TempFile;
use log::{info, warn};
use serde::Serialize;

use crate::utils::{
  aapt::{get_apk_info, get_name, get_version_code},
  error::{Error, Result},
};

use super::Repository;

#[derive(Clone, Serialize)]
pub struct App {
  pub package_name: String,
  pub categories: Vec<String>,
  pub suggested_version_code: String,
  pub license: String,
  pub name: String,
  pub added: i64,
  pub last_updated: i64,
  pub packages: Vec<Package>,
}

impl App {
  /// Reads a Json Value and tries to extract all fields to create a list of apps
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
      for category in app.get("categories")?.as_array()? {
        categories.push(category.as_str()?.to_string());
      }

      let mut packages_vec = vec![];

      let package = packages.get(&package_name)?;

      // map all package fields
      for package_entry in package.as_array()? {
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
  // Exist
  pub added: i64,
  pub apk_name: String,
  pub hash: String,
  pub hash_type: String,
  pub package_name: String,
  pub size: u64,
  pub version_name: String,
  // Can be Missing
  pub nativecode: Vec<String>,
  pub max_sdk_version: Option<u32>,
  pub min_sdk_version: Option<u32>,
  pub sig: Option<String>,
  pub signer: Option<String>,
  pub target_sdk_version: Option<u32>,
  pub uses_permission: Vec<(String, Option<u32>)>,
  pub version_code: Option<u64>,
}

impl Package {
  /// Reads a Json Value and tries to extract all fields to create an instance of Package
  ///
  /// returns None if any field can't be converted
  fn from_json(value: &serde_json::Value) -> Option<Self> {
    // Always Exist
    let added = value.get("added")?.as_i64()?;
    let apk_name = value.get("apkName")?.as_str()?.to_owned();
    let hash = value.get("hash")?.as_str()?.to_owned();
    let hash_type = value.get("hashType")?.as_str()?.to_owned();
    let package_name = value.get("packageName")?.as_str()?.to_owned();
    let size = value.get("size")?.as_u64()?;
    let version_name = value.get("versionName")?.as_str()?.to_owned();

    // Can be missing
    let max_sdk_version = value
      .get("maxSdkVersion")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    let min_sdk_version = value
      .get("minSdkVersion")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    let mut nativecode = vec![];

    for nativecode_entry in value
      .get("nativecode")
      .and_then(|val| val.as_array())
      .unwrap_or(&vec![])
    {
      nativecode.push(nativecode_entry.as_str()?.to_owned());
    }

    let sig = value
      .get("sig")
      .and_then(|val| val.as_str())
      .map(|val| val.to_owned());

    let signer = value
      .get("signer")
      .and_then(|val| val.as_str())
      .map(|val| val.to_owned());

    let target_sdk_version = value
      .get("targetSdkVersion")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    let mut uses_permission = vec![];

    for uses_permission_entry in value
      .get("uses-permission")
      .unwrap_or(&serde_json::Value::Null)
      .as_array()
      .unwrap_or(&vec![])
    {
      uses_permission.push((
        uses_permission_entry.get(0)?.as_str()?.to_owned(),
        uses_permission_entry
          .get(1)?
          .as_i64()
          .and_then(|val| val.try_into().ok()),
      ));
    }

    let version_code = value
      .get("versionCode")
      .and_then(|val| val.as_u64())
      .and_then(|val| val.try_into().ok());

    Some(Self {
      added,
      apk_name,
      hash,
      hash_type,
      max_sdk_version,
      min_sdk_version,
      nativecode,
      package_name,
      sig,
      signer,
      size,
      target_sdk_version,
      uses_permission,
      version_code,
      version_name,
    })
  }
}

impl Repository {
  /// Reads the index file generated by fdroid and returns all apps
  ///
  /// Returns an error if the json file can't be mapped correctely
  pub fn get_apps(&self) -> Result<Vec<App>> {
    let index_file = self.get_repo_path().join("index-v1.json");

    if !index_file.exists() {
      // if no index file exists, no apps exist
      return Ok(vec![]);
    }

    let mut file = File::open(index_file)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    App::from_json(
      &serde_json::from_str(&file_content)
        .map_err(|_| Error::JsonConvert("Could not read repository index file!".to_owned()))?,
    )
    .ok_or(Error::JsonConvert(
      "Could not map repository index file!".to_owned(),
    ))
  }

  /// Uploads a file directly to the app repository
  pub fn upload_app(&self, file: TempFile) -> Result<()> {
    // save file
    let new_file_path = self.get_repo_path().join(
      file
        .file_name
        .as_ref()
        .ok_or(Error::User("File has no name!".to_owned()))?,
    );

    // if file already exists, warn
    if new_file_path.exists() {
      warn!(
        "File already exists, overriding existing file: {:?}",
        new_file_path
      );
    }

    self.persist_temp_file(file, new_file_path.clone())?;

    // update meta data
    let update_result = self.update();

    // cleanup if error
    if update_result.is_err() && new_file_path.exists() && new_file_path.is_file() {
      fs::remove_file(new_file_path)?;
    }

    Ok(())
  }

  /// Deletes an apk (if it exists)
  pub fn delete_app(&self, apk_name: &str) -> Result<()> {
    info!("Deleting \"{}\"", apk_name);
    let file_path = self.get_repo_path().join(apk_name);

    // check if file exists
    if file_path.exists() {
      // check if file as really a file
      if file_path.is_file() {
        // delete the file
        fs::remove_file(file_path)?;

        // update metadata
        self.update()
      } else {
        Err(Error::User("Provided file is not a file!".to_owned()))
      }
    } else {
      warn!("Trying to delete \"{}\" but file does not exist!", apk_name);
      Ok(())
    }
  }

  /// Signs an apk and uploads it
  ///
  /// - parses apk metadata
  /// - Uploads apk to unsigned folder
  /// - signs apk
  pub fn sign_app(&self, file: TempFile) -> Result<()> {
    // get apk metadata
    let apk_metadata = get_apk_info(&file.file.path().to_owned())?;

    // get version and name
    let apk_version = get_version_code(&apk_metadata)?;
    let apk_name = get_name(&apk_metadata)?;

    // Upload apk to unsigned folder
    let file_path = self
      .get_unsigned_path()?
      .join(format!("{}_{}.apk", apk_name, apk_version));
    self.persist_temp_file(file, file_path.clone())?;

    // check if metadata exists
    let metadata = self.get_metadata(&apk_name);
    if metadata.is_err() {
      warn!("No metadata for this package exists, creating empty metadata file!");
      self.create_metadata(&apk_name)?;
    }

    // run fdroid publish
    self.publish()?;

    // run fdroid update
    self.update()?;

    Ok(())
  }

  /// Saves a temporary file to a final location
  ///
  /// Needed because file.persist() throws error if the destination directory is mounted inside a docker container
  pub fn persist_temp_file(&self, file: TempFile, path: PathBuf) -> Result<File> {
    // create temporary directory
    let temp_dir_path = PathBuf::from("/tmp/files");
    if !temp_dir_path.exists() {
      fs::create_dir_all(temp_dir_path.clone())?;
    }

    // save file to temporary directory
    let persistent_temp_file_path = temp_dir_path.join(
      path
        .file_name()
        .ok_or(Error::Custom("File Name not provided!".to_owned()))?,
    );

    // persist file to temporary location
    file
      .file
      .persist(persistent_temp_file_path.clone())
      .map_err(io::Error::from)?;

    // copy file
    let file_copy_result = fs::copy(persistent_temp_file_path.clone(), path.clone()).map(|_| ());

    // remove old file
    fs::remove_file(persistent_temp_file_path)?;

    // if copy operation was unsucessful, return here
    file_copy_result?;

    File::open(path).map_err(Error::from)
  }
}

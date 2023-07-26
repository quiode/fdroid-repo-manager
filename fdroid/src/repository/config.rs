//! Extension of Repository used to modify the config file

use log::debug;
use std::fs;
use std::path::PathBuf;

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

use super::Repository;

/// Actual Structure of the config.yml file
#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
  // immutable part
  sdk_path: String,
  repo_keyalias: String,
  keystore: String,
  keystorepass: String,
  keypass: String,
  keydname: String,
  // changeaple part
  // repo
  #[serde(skip_serializing_if = "Option::is_none")]
  repo_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  repo_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  repo_icon: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  repo_description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  apksigner: Option<String>,
  // archive
  #[serde(skip_serializing_if = "Option::is_none")]
  archive_url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  archive_name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  archive_icon: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  archive_description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  archive_older: Option<u8>,
  // TODO: update_stats
}

impl ConfigFile {
  /// Creates new ConfigFile with public fields
  fn merge_with_public(&self, public: &PublicConfig) -> Self {
    Self {
      sdk_path: self.sdk_path.clone(),
      repo_keyalias: self.repo_keyalias.clone(),
      keystore: self.keystore.clone(),
      keystorepass: self.keystorepass.clone(),
      keypass: self.keypass.clone(),
      keydname: self.keydname.clone(),
      apksigner: self.apksigner.clone(),
      repo_url: public.repo_url.clone(),
      repo_name: public.repo_name.clone(),
      repo_icon: public.repo_icon.clone(),
      archive_icon: public.archive_icon.clone(),
      repo_description: public.repo_description.clone(),
      archive_description: public.archive_description.clone(),
      archive_name: public.archive_name.clone(),
      archive_older: public.archive_older,
      archive_url: public.archive_url.clone(),
    }
  }
}

/// Part of the config file that can be changed
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PublicConfig {
  // repo
  pub repo_url: Option<String>,
  pub repo_name: Option<String>,
  pub repo_icon: Option<String>,
  pub repo_description: Option<String>,
  // archive
  pub archive_url: Option<String>,
  pub archive_name: Option<String>,
  pub archive_icon: Option<String>,
  pub archive_description: Option<String>,
  pub archive_older: Option<u8>,
}

impl From<ConfigFile> for PublicConfig {
  fn from(value: ConfigFile) -> Self {
    Self {
      repo_url: value.repo_url,
      repo_name: value.repo_name,
      repo_icon: value.repo_icon,
      repo_description: value.repo_description,
      archive_url: value.archive_url,
      archive_name: value.archive_name,
      archive_icon: value.archive_icon,
      archive_description: value.archive_description,
      archive_older: value.archive_older,
    }
  }
}

impl Repository {
  /// Get Public Part of the Config File
  pub fn get_public_config(&self) -> Result<PublicConfig> {
    self.get_config().map(|config| config.into())
  }

  // Set Config (don't change private part of the config file)
  pub fn set_config(&self, public_config: &PublicConfig) -> Result<()> {
    let config_file = self.get_config()?;

    let merged_config = config_file.merge_with_public(public_config);

    self.write_to_config(&merged_config)
  }

  /// Returns the keystore password
  pub fn get_keystore_password(&self) -> Result<String> {
    let config_file = self.get_config()?;

    Ok(config_file.keystorepass)
  }

  /// Saves the store image
  pub fn save_image(&self, new_image_path: &PathBuf) -> Result<()> {
    debug!("Saving to repository image!");

    let image_path = self.get_image_path()?;

    // check if it is the same image type
    let new_image_type = new_image_path
      .extension()
      .ok_or(Error::User("Image does not have a file type!".to_owned()))?;
    let current_image_type = image_path.extension().ok_or(Error::Custom(
      "Current Image does not have a file name!".to_owned(),
    ))?;

    // if image types are not the same, throw an error
    if new_image_type != current_image_type {
      Err(Error::User(
        format!("Image type should be: {:?}", current_image_type).to_string(),
      ))
    } else {
      // safe the image
      fs::copy(new_image_path, &image_path)?;

      Ok(())
    }
  }

  /// Gets the path to the repository image
  pub fn get_image_path(&self) -> Result<PathBuf> {
    let image_name = self
      .get_config()?
      .repo_icon
      .unwrap_or("icon.png".to_owned());

    Ok(self.repo_path().join("icons").join(image_name))
  }

  /// Get Config File as it is
  fn get_config(&self) -> Result<ConfigFile> {
    let yml_string = fs::read_to_string(self.config_path())?;

    serde_yaml::from_str::<ConfigFile>(&yml_string).map_err(Error::from)
  }

  /// writes to the actual config file
  fn write_to_config(&self, config_file: &ConfigFile) -> Result<()> {
    // convert to yml string
    let yml_string = serde_yaml::to_string(config_file)?;

    // write to file
    fs::write(self.config_path(), yml_string)?;

    // update repository
    self.update()
  }
}

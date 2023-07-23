use std::path::PathBuf;
use std::{fs, fs::File, io::Read};

use log::{debug, warn};
use serde::{Deserialize, Serialize};

use crate::utils::error::{Error, Result};

use super::Repository;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct AppMetadata {
  pub Categories: Option<Vec<Categories>>,
  pub AuthorName: Option<String>,
  pub AuthorEmail: Option<String>,
  pub AuthorWebSite: Option<String>,
  pub License: Option<String>,
  pub AutoName: Option<String>,
  pub Name: Option<String>,
  pub WebSite: Option<String>,
  pub SourceCode: Option<String>,
  pub IssueTracker: Option<String>,
  pub Translation: Option<String>,
  pub Changelog: Option<String>,
  pub Donate: Option<String>,
  pub FlattrID: Option<String>,
  pub Liberapay: Option<String>,
  pub OpenCollective: Option<String>,
  pub Bitcoin: Option<String>,
  pub Litecoin: Option<String>,
  pub Summary: Option<String>,
  pub Description: Option<String>,
  pub MaintainerNotes: Option<String>,
  pub RepoType: Option<RepoType>,
  pub Repo: Option<String>,
  pub Binaries: Option<String>,
  pub Builds: Option<Builds>,
  pub AllowedAPKSigningKeys: Option<String>,
  pub AntiFeatures: Option<AntiFeatures>,
  pub Disabled: Option<String>,
  pub RequiresRoot: Option<bool>,
  pub ArchivePolicy: Option<u32>,
  pub UpdateCheckMode: Option<UpdateCheckMode>,
  pub UpdateCheckIgnore: Option<String>,
  pub VercodeOperation: Option<String>,
  pub UpdateCheckName: Option<String>,
  pub UpdateCheckData: Option<String>,
  pub AutoUpdateMode: Option<AutoUpdateMode>,
  pub CurrentVersion: Option<String>,
  pub CurrentVersionCode: Option<String>,
  pub NoSourceSince: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct Builds {
  pub versionName: Option<String>,
  pub versionCode: Option<String>,
  pub commit: Option<String>,
  pub disable: Option<String>,
  pub subdir: Option<String>,
  pub submodules: Option<bool>,
  pub sudo: Option<String>,
  pub timeout: Option<u64>,
  pub init: Option<String>,
  pub oldsdkloc: Option<bool>,
  pub target: Option<String>,
  pub androidupdate: Option<AndroidUpdate>,
  pub encoding: Option<String>,
  pub forceversion: Option<bool>,
  pub forcevercode: Option<bool>,
  pub rm: Option<Vec<String>>,
  pub extlibs: Option<Vec<String>>,
  pub srclibs: Option<Vec<String>>,
  pub patch: Option<String>,
  pub prebuild: Option<String>,
  pub scanignore: Option<Vec<String>>,
  pub scandelete: Option<Vec<String>>,
  pub build: Option<String>,
  pub buildjni: Option<String>,
  pub ndk: Option<String>,
  pub gradle: Option<Vec<String>>,
  pub maven: Option<String>,
  pub preassemble: Option<Vec<String>>,
  pub gradleprops: Option<Vec<String>>,
  pub antcommands: Option<Vec<String>>,
  pub output: Option<String>,
  pub postbuild: Option<String>,
  pub novcheck: Option<bool>,
  pub antifeatures: Option<Vec<AntiFeatures>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
pub enum Categories {
  Connectivity,
  Development,
  Games,
  Graphics,
  Internet,
  Money,
  Multimedia,
  Navigation,
  #[serde(rename = "Phone & SMS")]
  PhoneSms,
  Reading,
  #[serde(rename = "Science & Education")]
  ScienceEducation,
  Security,
  #[serde(rename = "Sports & Health")]
  SportsHealth,
  System,
  Theming,
  Time,
  Writing,
  Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum RepoType {
  #[serde(rename = "git")]
  Git,
  #[serde(rename = "svn")]
  Svn,
  #[serde(rename = "git-svn")]
  GitSvn,
  #[serde(rename = "hg")]
  Hg,
  #[serde(rename = "bzr")]
  Bzr,
  #[serde(rename = "srclib")]
  Srclib,
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum AntiFeatures {
  Ads,
  Tracking,
  NonFreeNet,
  NonFreeAdd,
  NonFreeDep,
  #[serde(rename = "NSFW")]
  Nsfw,
  UpstreamNonFree,
  NonFreeAssets,
  KnownVuln,
  ApplicationDebuggable,
  NoSourceSince,
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum UpdateCheckMode {
  None,
  Static,
  RepoManifest,
  RepoTrunk,
  Tags,
  #[serde(rename = "HTTP")]
  Http,
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum AutoUpdateMode {
  None,
  Version,
}

#[derive(Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum AndroidUpdate {
  #[serde(rename = "auto")]
  Auto,
  #[serde(rename = "dirs")]
  Dirs(Vec<String>),
}

impl Repository {
  /// gets the file path of an metadata file
  fn get_meta_file_path(&self, package_name: &str) -> PathBuf {
    self.get_metadata_path().join(format!("{package_name}.yml"))
  }

  /// creates the metadata dir if it does not exist
  fn create_metadata_dir(&self) -> Result<()> {
    let metadata_path = self.get_metadata_path();

    if !metadata_path.exists() {
      fs::create_dir_all(metadata_path)?;
      Ok(())
    } else {
      Ok(())
    }
  }

  /// Readas the metadata from an app
  ///
  /// # Error
  /// - throws an error if the data does not exist
  /// - throws an error if the file can't be mapped
  pub fn get_metadata(&self, package_name: &str) -> Result<AppMetadata> {
    debug!("Getting metadata for {package_name}!");
    // get metadata file path
    let meta_file_path = self.get_meta_file_path(package_name);

    if meta_file_path.exists() && meta_file_path.is_file() {
      // get file
      let mut file = File::open(meta_file_path)?;
      // parse file content to a string
      let mut file_content = String::new();
      // map file to rust struct
      file.read_to_string(&mut file_content)?;

      serde_yaml::from_str(&file_content).map_err(Error::from)
    } else {
      Err(Error::User("Metadata file does not exist!".to_owned()))
    }
  }

  /// Sets the metadata for an app
  pub fn set_metadata(&self, package_name: &str, metadata: &AppMetadata) -> Result<()> {
    debug!("Writing new metadata for {package_name}!");
    debug!("Metadata:\n{metadata:#?}");
    // get metadata file path
    let meta_file_path = self.get_meta_file_path(package_name);

    // convert data to string
    let file_content = serde_yaml::to_string(metadata)?;

    // write data to file
    fs::write(meta_file_path, file_content).map_err(Error::from)
  }

  /// Creates an empty metadata file (if none exist) and runs fdroid rewritemeta
  pub fn create_metadata(&self, package_name: &str) -> Result<()> {
    let file_path = self.get_meta_file_path(package_name);

    if file_path.is_file() {
      warn!("Metadata file already exists!");
    } else if file_path.exists() {
      warn!("File Path already exists but is not a file!");
    } else {
      // create empty metadata file
      self.create_metadata_dir()?;
      fs::write(file_path, "")?;
    }

    // run fdroid rewritemeta to create basic meta file information
    self.cleanup()
  }
}

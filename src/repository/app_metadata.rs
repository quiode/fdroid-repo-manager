use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::utils::error::{Error, Result};

use super::Repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct AppMetadata {
  Categories: Option<Vec<Categories>>,
  AuthorName: Option<String>,
  AuthorEmail: Option<String>,
  AuthorWebSite: Option<String>,
  License: Option<String>,
  AutoName: Option<String>,
  Name: Option<String>,
  WebSite: Option<String>,
  SourceCode: Option<String>,
  IssueTracker: Option<String>,
  Translation: Option<String>,
  Changelog: Option<String>,
  Donate: Option<String>,
  FlattrID: Option<String>,
  Liberapay: Option<String>,
  OpenCollective: Option<String>,
  Bitcoin: Option<String>,
  Litecoin: Option<String>,
  Summary: Option<String>,
  Description: Option<String>,
  MaintainerNotes: Option<String>,
  RepoType: Option<RepoType>,
  Repo: Option<String>,
  Binaries: Option<String>,
  Builds: Option<Builds>,
  AllowedAPKSigningKeys: Option<String>,
  AntiFeatures: Option<AntiFeatures>,
  Disabled: Option<String>,
  RequiresRoot: Option<bool>,
  ArchivePolicy: Option<u32>,
  UpdateCheckMode: Option<UpdateCheckMode>,
  UpdateCheckIgnore: Option<String>,
  VercodeOperation: Option<String>,
  UpdateCheckName: Option<String>,
  UpdateCheckData: Option<String>,
  AutoUpdateMode: Option<AutoUpdateMode>,
  CurrentVersion: Option<String>,
  CurrentVersionCode: Option<String>,
  NoSourceSince: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Builds {
  versionName: Option<String>,
  versionCode: Option<String>,
  commit: Option<String>,
  disable: Option<String>,
  subdir: Option<String>,
  submodules: Option<bool>,
  sudo: Option<String>,
  timeout: Option<u64>,
  init: Option<String>,
  oldsdkloc: Option<bool>,
  target: Option<String>,
  androidupdate: Option<AndroidUpdate>,
  encoding: Option<String>,
  forceversion: Option<bool>,
  forcevercode: Option<bool>,
  rm: Option<Vec<String>>,
  extlibs: Option<Vec<String>>,
  srclibs: Option<Vec<String>>,
  patch: Option<String>,
  prebuild: Option<String>,
  scanignore: Option<Vec<String>>,
  scandelete: Option<Vec<String>>,
  build: Option<String>,
  buildjni: Option<String>,
  ndk: Option<String>,
  gradle: Option<Vec<String>>,
  maven: Option<String>,
  preassemble: Option<Vec<String>>,
  gradleprops: Option<Vec<String>>,
  antcommands: Option<Vec<String>>,
  output: Option<String>,
  postbuild: Option<String>,
  novcheck: Option<bool>,
  antifeatures: Option<Vec<AntiFeatures>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum UpdateCheckMode {
  None,
  Static,
  RepoManifest,
  RepoTrunk,
  Tags,
  #[serde(rename = "HTTP")]
  Http,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AutoUpdateMode {
  None,
  Version,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AndroidUpdate {
  #[serde(rename = "auto")]
  Auto,
  #[serde(rename = "dirs")]
  Dirs(Vec<String>),
}

impl Repository {
  /// Readas the metadata from an app
  ///
  /// # Error
  /// - throws an error if the data does not exist
  /// - throws an error if the file can't be mapped
  pub fn get_metadata(&self, package_name: &str) -> Result<AppMetadata> {
    // get metadata file path
    let meta_file_path = self.get_metadata_path().join(format!("{package_name}.yml"));

    if meta_file_path.exists() && meta_file_path.is_file() {
      // get file
      let mut file = File::open(meta_file_path).map_err(Error::from)?;
      // parse file content to a string
      let mut file_content = String::new();
      // map file to rust struct
      file
        .read_to_string(&mut file_content)
        .map_err(Error::from)?;

      serde_yaml::from_str(&file_content).map_err(Error::from)
    } else {
      Err(Error::User("Metadata file does not exist!".to_owned()))
    }
  }
}

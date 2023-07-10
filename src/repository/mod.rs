use std::{fs, path::PathBuf, process::Command};

use log::{debug, info, warn};

use crate::utils::error::{Error, Result};

pub mod app;
pub mod app_metadata;
pub mod config;

#[derive(Debug, Clone)]
pub struct Repository {
  /// absolute path of the /fdroid repository
  path: PathBuf,
}

impl Repository {
  /// get path to the config.yml file
  fn get_config_path(&self) -> PathBuf {
    self.path.join("config.yml")
  }

  /// get the path of the metadata directory
  fn get_metadata_path(&self) -> PathBuf {
    self.path.join("metadata")
  }

  // Create a new repository with the provided path
  // returns the path to the app repository
  pub fn get_repo_path(&self) -> PathBuf {
    self.path.join("repo")
  }

  /// Runs fdroid init if config.yml misses
  pub fn new(path: PathBuf) -> Self {
    let repository = Self { path };

    // check if config.yml exists
    if repository.get_config_path().exists() {
      repository
    } else {
      // initialize directory
      repository.initialize();

      repository
    }
  }

  /// Initializes a new repository by calling fdroid init
  fn initialize(&self) {
    info!("Initializing a new fdroid repository!");

    self
      .run("init", &vec![])
      .expect("Failed to initialize the repository!");
  }

  /// Runs "fdroid update -c"
  fn update(&self) -> Result<()> {
    debug!("Updating Repository (Running fdroid update -c)");

    self.run("update", &vec!["-c"])
  }

  /// Runs an fdroid command with the specified arguments
  fn run(&self, command: &str, args: &Vec<&str>) -> Result<()> {
    let run_result = Command::new("fdroid")
      .arg(command)
      .args(args)
      .current_dir(&self.path)
      .spawn()
      .ok()
      .and_then(|mut process| process.wait().ok());

    let error_message =
      format!("Failed to run command: \"fdroid {command}\" with arguemnts: \"{args:#?}\"");

    if run_result.is_none() {
      warn!("{}", error_message);
    }

    run_result.map(|_| ()).ok_or(Error::Custom(error_message))
  }

  /// Deletes all apps and metadata (but keeps everything else)
  pub fn clear(&self) -> Result<()> {
    warn!("Clearing the repository!");

    // Delete all apps
    fs::remove_dir_all(self.get_repo_path()).map_err(Error::from)?;
    // Create directory again
    fs::create_dir(self.get_repo_path()).map_err(Error::from)?;

    // Delete all metadata files
    fs::remove_dir_all(self.get_metadata_path()).map_err(Error::from)?;
    // Create metadata directory
    fs::create_dir(self.get_metadata_path()).map_err(Error::from)?;

    // update index files etc
    self.update()
  }
}

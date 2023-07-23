use std::{fs, path::PathBuf, process::Command};

use log::{debug, info, warn};

use crate::utils::error::{Error, Result};

pub mod app;
pub mod app_metadata;
pub mod config;
mod tests;

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

  /// gets the path to the unsigned files
  ///
  /// also creates the directory if it does not already exist
  fn get_unsigned_path(&self) -> Result<PathBuf> {
    let path = self.path.join("unsigned");

    // check if path is valid (could be invalid)
    if path.exists() {
      if path.is_dir() {
        Ok(path)
      } else {
        Err(Error::Custom("unsigned directory is a file!".to_owned()))
      }
    } else {
      fs::create_dir(path.clone())?;
      Ok(path)
    }
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

  /// Runs "fdroid update -c; fdroid update"
  fn update(&self) -> Result<()> {
    debug!("Updating Repository (Running fdroid update -c; fdroid update)");

    self.run("update", &vec!["-c"])?;
    self.run("update", &vec![])
  }

  /// Runs "fdroid publish"
  fn publish(&self) -> Result<()> {
    debug!("Running fdroid publish");

    self.run("publish", &vec![])
  }

  /// Runs an fdroid command with the specified arguments
  fn run(&self, command: &str, args: &Vec<&str>) -> Result<()> {
    let run_result = Command::new("fdroid")
      .arg(command)
      .args(args)
      .current_dir(&self.path)
      .spawn()
      .map_err(|err| {
        debug!("Error spawning run command: {err:#?}");
        err
      })
      .ok()
      .and_then(|mut process| {
        process
          .wait()
          .map_err(|err| {
            debug!("Error while running process: {process:#?}");
            err
          })
          .ok()
      });

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
    fs::remove_dir_all(self.get_repo_path())?;
    // Create directory again
    fs::create_dir(self.get_repo_path())?;

    // Delete all metadata files
    fs::remove_dir_all(self.get_metadata_path())?;
    // Create metadata directory
    fs::create_dir(self.get_metadata_path())?;

    // update index files etc
    self.update()
  }

  /// Runs "fdroid rewritemeta"
  pub fn cleanup(&self) -> Result<()> {
    debug!("Cleaning up metadata files!");
    self.run("rewritemeta", &vec![])
  }

  /// Returns the path to the keystore file
  pub fn get_keystore_path(&self) -> PathBuf {
    self.path.join("keystore.p12")
  }
}

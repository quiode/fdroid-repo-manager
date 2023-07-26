use std::fs;
use std::path::PathBuf;

use actix_multipart::form::tempfile::TempFile;
use error::*;

pub mod app_config;
pub mod error;

/// Saves a temporary file
///
/// Returns the location of the saved file
///
/// Needed because file.persist() throws error if the destination directory is mounted inside a docker container
pub fn persist_temp_file(temp_file: TempFile) -> Result<PathBuf> {
  // create temporary directory
  let temp_dir_path = PathBuf::from("/tmp/files");
  if !temp_dir_path.exists() {
    fs::create_dir_all(temp_dir_path.clone())?;
  }

  // save file to temporary directory
  let persistent_temp_file_path = temp_dir_path.join(
    temp_file
      .file_name
      .ok_or(Error::UserError("File Name not provided!".to_owned()))?,
  );

  // persist file to temporary location
  temp_file .file .persist(&persistent_temp_file_path).map_err(|_err| Error::UnexpectedError("An Error occurred while persisting a temporary file. Are you running this service inside a docker container?".to_string()))?;

  Ok(persistent_temp_file_path)
}

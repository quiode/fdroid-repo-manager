//! Module for working with aapt

use std::{path::PathBuf, process::Command};

use regex::Regex;

use super::error::{Error, Result};

/// Returns metadata of an apk as a string
///
/// # Error
/// Returns an error if the file does not exist or can't be parsed
fn get_apk_info(apk_path: &PathBuf) -> Result<String> {
  if apk_path.is_file() {
    // run aapt command
    let output = Command::new("aapt")
      .arg("badging")
      .arg(apk_path)
      .output()
      .map_err(|_| Error::Custom("Failed to get metadata from apk!".to_owned()))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
  } else {
    Err(Error::Custom("APK Path is not a valid file!".to_owned()))
  }
}

/// gets the version code from an apk metadata string
fn get_version_code(metadata: &str) -> Result<u32> {
  let regex = Regex::new(r"versionCode='(\d+)'").unwrap();

  // apply regext to string
  let Some(captures) = regex.captures(metadata) else {
    return Err(Error::Custom("versionCode not found!".to_owned()));
  };

  let string_version_code = captures
    .get(1)
    .ok_or(Error::Custom("versionCode not found!".to_owned()))?;

  string_version_code
    .as_str()
    .parse()
    .map_err(|_| Error::Custom("versionCode is not a valid number!".to_owned()))
}
/// gets the name from an apk metadata string
fn get_name(metadata: &str) -> Result<String> {
  let regex = Regex::new(r"name='((?:[[:alpha:]]|\.)+)'").unwrap();

  // apply regext to string
  let Some(captures) = regex.captures(metadata) else {
    return Err(Error::Custom("name not found!".to_owned()));
  };

  let name = captures
    .get(1)
    .ok_or(Error::Custom("name not found!".to_owned()))?;

  Ok(name.as_str().to_string())
}

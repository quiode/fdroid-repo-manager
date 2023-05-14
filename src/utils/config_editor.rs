use std::fs::{ self };
use serde::{ Serialize, Deserialize };

use super::error::{ Error, Result };

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    sdk_path: String,
    repo_keyalias: String,
    keystore: String,
    keystorepass: String,
    keypass: String,
    keydname: String,
    apksigner: String,
    repo_url: Option<String>,
    repo_name: Option<String>,
    repo_icon: Option<String>,
    archive_icon: Option<String>,
    repo_description: Option<String>,
}

pub fn get_config(file_path: &String) -> Result<ConfigFile> {
    let yml_string = fs::read_to_string(file_path).map_err(|err| Error::from(err))?;

    return serde_yaml::from_str::<ConfigFile>(&yml_string).map_err(|err| Error::from(err));
}
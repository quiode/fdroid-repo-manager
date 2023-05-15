use std::fs::{ self };
use serde::{ Serialize, Deserialize };

use super::error::{ Error, Result };

/// Actual Structure of the config.yml file
#[derive(Serialize, Deserialize, Debug)]
struct ConfigFile {
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

impl ConfigFile {
    /// Creates new ConfigFile with public fields, if they exist, else own
    fn merge_with_public(&self, public: &PublicConfig) -> Self {
        Self {
            sdk_path: self.sdk_path.clone(),
            repo_keyalias: self.repo_keyalias.clone(),
            keystore: self.keystore.clone(),
            keystorepass: self.keystorepass.clone(),
            keypass: self.keypass.clone(),
            keydname: self.keydname.clone(),
            apksigner: self.apksigner.clone(),
            repo_url: public.repo_url.clone().or(self.repo_url.clone()),
            repo_name: public.repo_name.clone().or(self.repo_name.clone()),
            repo_icon: public.repo_icon.clone().or(self.repo_icon.clone()),
            archive_icon: public.archive_icon.clone().or(self.archive_icon.clone()),
            repo_description: public.repo_description.clone().or(self.repo_description.clone()),
        }
    }
}

/// Part of the config file that can be changed
pub struct PublicConfig {
    repo_url: Option<String>,
    repo_name: Option<String>,
    repo_icon: Option<String>,
    archive_icon: Option<String>,
    repo_description: Option<String>,
}

impl From<ConfigFile> for PublicConfig {
    fn from(value: ConfigFile) -> Self {
        Self {
            repo_url: value.repo_url,
            repo_name: value.repo_name,
            repo_icon: value.repo_icon,
            archive_icon: value.archive_icon,
            repo_description: value.repo_description,
        }
    }
}

/// Get Public Part of the Config File
pub fn get_public_config(file_path: &String) -> Result<PublicConfig> {
    get_config(file_path).map(|config| config.into())
}

// Set Config (don't change private part of the config file)
pub fn set_config(file_path: &String, public_config: &PublicConfig) -> Result<()> {
    let config_file = get_config(file_path)?;

    let merged_config = config_file.merge_with_public(public_config);

    write_to_config(file_path, &merged_config)
}

/// Get Config File as it is
fn get_config(file_path: &String) -> Result<ConfigFile> {
    let yml_string = fs::read_to_string(file_path).map_err(Error::from)?;

    return serde_yaml::from_str::<ConfigFile>(&yml_string).map_err(Error::from);
}

/// writes to the actual config file
fn write_to_config(file_path: &String, config_file: &ConfigFile) -> Result<()> {
    // convert to yml string
    let yml_string = serde_yaml::to_string(config_file).map_err(Error::from)?;

    // write to file
    fs::write(file_path, yml_string).map_err(Error::from)
}
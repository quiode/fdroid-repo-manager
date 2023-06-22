use std::{path::PathBuf, process::Command};

use log::{debug, info, warn};

use crate::utils::error::{Error, Result};

pub mod app;
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

    // Create a new repository with the provided path
    /// Runs fdroid init if config.yml misses
    pub fn new(path: PathBuf) -> Self {
        let repository = Self { path };

        // check if config.yml exists
        if repository.get_config_path().exists() {
            return repository;
        } else {
            // initialize directory
            repository.initialize();

            return repository;
        }
    }

    // returns the path to the app repository
    pub fn repo_path(&self) -> PathBuf {
        self.path.join("repo")
    }

    /// Initializes a new repository by calling fdroid init
    fn initialize(&self) {
        info!("Initializing a new fdroid repository!");

        self.run("init", &vec![])
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
            .spawn();

        if run_result.is_err() {
            warn!("Failed to run command: \"fdroid {command}\" with arguemnts: \"{args:#?}\"");
        }

        run_result.map(|_| ()).map_err(Error::from)
    }
}

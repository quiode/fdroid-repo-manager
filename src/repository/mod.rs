use std::{ path::{ PathBuf } };

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
    pub fn new(path: &String) -> Self {
        let repository = Self {
            path: PathBuf::from(path),
        };
        // check if config.yml exists
        if repository.get_config_path().exists() {
            return repository;
        } else {
            // initialize directory
            repository.initialize();

            return repository;
        }
    }

    /// Initializes a new repository by calling fdroid init
    fn initialize(&self) {
        todo!()
    }
}
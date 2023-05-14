use std::env;

/// Immutable Configuration struct for the whole application.
/// <br>
/// Set by environment variables at the beginning.
#[derive(Clone, Debug)]
pub struct AppConfig {
    // RM_PORT
    pub port: u16,
    // RM_IP
    pub ip: String,
    // RM_REPO_PATH
    pub repo_path: String,
    // RM_ADMIN_PASSWORD
    pub admin_password: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 80,
            ip: "127.0.0.1".to_string(),
            repo_path: "/fdroid".to_string(),
            admin_password: "admin".to_string(),
        }
    }
}

impl AppConfig {
    /// Creates a new Object from environment variables or the default values
    /// if no environment variables are set
    pub fn from_env() -> Self {
        let default = Self::default();

        Self {
            port: env
                ::var("RM_PORT")
                .unwrap_or(default.port.to_string())
                .parse()
                .unwrap_or(default.port),
            ip: env::var("RM_IP").unwrap_or(default.ip),
            repo_path: env::var("REPO_PATH").unwrap_or(default.repo_path),
            admin_password: env::var("RM_ADMIN_PASSWORD").unwrap_or(default.admin_password),
        }
    }
}
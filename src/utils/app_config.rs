use std::{env, net::Ipv4Addr, path::PathBuf};

/**
 * Trait that descibes a struct that wraps a value.
 */
pub trait WrappedValue<T> {
  /**
   * Return the value which the struct wraps
   */
  fn value(&self) -> &T;
}

#[derive(Clone, Debug)]
pub struct Port(u16);

impl Default for Port {
  fn default() -> Self {
    Self(80)
  }
}

impl From<u16> for Port {
  fn from(value: u16) -> Self {
    Self(value)
  }
}

impl ToString for Port {
  fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl WrappedValue<u16> for Port {
  fn value(&self) -> &u16 {
    &self.0
  }
}

#[derive(Clone, Debug)]
pub struct Ip(Ipv4Addr);

impl Default for Ip {
  fn default() -> Self {
    Self(Ipv4Addr::LOCALHOST)
  }
}

impl From<Ipv4Addr> for Ip {
  fn from(value: Ipv4Addr) -> Self {
    Self(value)
  }
}

impl ToString for Ip {
  fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl WrappedValue<Ipv4Addr> for Ip {
  fn value(&self) -> &Ipv4Addr {
    &self.0
  }
}

#[derive(Clone, Debug)]
pub struct RepoPath(PathBuf);

impl Default for RepoPath {
  fn default() -> Self {
    Self(PathBuf::from("/fdroid"))
  }
}

impl From<PathBuf> for RepoPath {
  fn from(value: PathBuf) -> Self {
    Self(value)
  }
}

impl ToString for RepoPath {
  fn to_string(&self) -> String {
    self.0.to_string_lossy().to_string()
  }
}

impl WrappedValue<PathBuf> for RepoPath {
  fn value(&self) -> &PathBuf {
    &self.0
  }
}

#[derive(Clone, Debug)]
pub struct Password(String);

impl Default for Password {
  fn default() -> Self {
    Self("admin".to_owned())
  }
}

impl From<String> for Password {
  fn from(value: String) -> Self {
    Self(value)
  }
}

impl ToString for Password {
  fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl WrappedValue<String> for Password {
  fn value(&self) -> &String {
    &self.0
  }
}

#[derive(Clone, Debug)]
pub struct MaxPayloadSize(usize);

impl MaxPayloadSize {
  /// returns MiB converted to B
  #[allow(non_snake_case)]
  fn mega_to_bytes(MiB: usize) -> usize {
    MiB * 1048576
  }
}

impl Default for MaxPayloadSize {
  fn default() -> Self {
    Self(Self::mega_to_bytes(250))
  }
}

impl From<usize> for MaxPayloadSize {
  fn from(value: usize) -> Self {
    Self(value)
  }
}

impl ToString for MaxPayloadSize {
  fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl WrappedValue<usize> for MaxPayloadSize {
  fn value(&self) -> &usize {
    &self.0
  }
}

/// Immutable Configuration struct for the whole application.
/// <br>
/// Set by environment variables at the beginning.
#[derive(Clone, Debug, Default)]
pub struct AppConfig {
  // RM_PORT
  pub port: Port,
  // RM_IP
  pub ip: Ip,
  // RM_REPO_PATH
  pub repo_path: RepoPath,
  // RM_ADMIN_PASSWORD
  pub admin_password: Password,
  // Payload Size
  pub max_payload_size: MaxPayloadSize,
}

impl AppConfig {
  /// Creates a new Object from environment variables or the default values
  /// if no environment variables are set
  pub fn from_env() -> Self {
    Self {
      port: env::var("RM_PORT")
        .map(|string| {
          let parsed_string = string.parse();

          match parsed_string {
            Ok(port) => Port(port),
            Err(_) => Port::default(),
          }
        })
        .unwrap_or_default(),
      ip: env::var("RM_IP")
        .map(|string| {
          let parsed_string = string.parse();

          match parsed_string {
            Ok(ip) => Ip(ip),
            Err(_) => Ip::default(),
          }
        })
        .unwrap_or_default(),
      repo_path: env::var("REPO_PATH")
        .map(|string| {
          let parsed_string = string.parse();

          match parsed_string {
            Ok(path) => RepoPath(path),
            Err(_) => RepoPath::default(),
          }
        })
        .unwrap_or_default(),
      admin_password: env::var("RM_ADMIN_PASSWORD")
        .map(|string| {
          let parsed_string = string.parse();

          match parsed_string {
            Ok(password) => Password(password),
            Err(_) => Password::default(),
          }
        })
        .unwrap_or_default(),
      max_payload_size: env::var("RM_MAX_PAYLOAD_SIZE")
        .map(|string| {
          let parsed_string = string.parse();

          match parsed_string {
            Ok(max_payload_size) => MaxPayloadSize(max_payload_size),
            Err(_) => MaxPayloadSize::default(),
          }
        })
        .unwrap_or_default(),
    }
  }
}

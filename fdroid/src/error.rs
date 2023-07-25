use core::fmt;
use std::{error, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
  File(io::Error),
  YAMLConvert(serde_yaml::Error),
  JsonConvert(String),
  /// Custom Error message
  Custom(String),
  /// User Error
  User(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::File(err) => write!(f, "Error while working with a file: {err:#?}"),
      Error::YAMLConvert(err) => write!(f, "Error while converting a yml file: {err:#?}"),
      Error::JsonConvert(err) => write!(f, "Error while converting a json file: {err}"),
      Error::Custom(err) => write!(f, "{err}"),
      Error::User(err) => write!(f, "{err}"),
    }
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::File(err) => Some(err),
      Error::YAMLConvert(err) => Some(err),
      Error::JsonConvert(_) => None,
      Error::Custom(_) => None,
      Error::User(_) => None,
    }
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::File(error)
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(error: serde_yaml::Error) -> Self {
    Self::YAMLConvert(error)
  }
}

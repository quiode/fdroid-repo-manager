use actix_web::{http::StatusCode, ResponseError};
use core::fmt;
use std::{error, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  FileError(io::Error),
  YAMLConvertError(serde_yaml::Error),
  JsonConvertError(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::FileError(err) => write!(f, "Error while working with a file: {err:#?}"),
      Error::YAMLConvertError(err) => write!(f, "Error while converting a yml file: {err:#?}"),
      Error::JsonConvertError(err) => write!(f, "Error while converting a json file: {err}"),
    }
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::FileError(err) => Some(err),
      Error::YAMLConvertError(err) => Some(err),
      Error::JsonConvertError(_err) => None,
    }
  }
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Self::FileError(error)
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(error: serde_yaml::Error) -> Self {
    Self::YAMLConvertError(error)
  }
}

impl ResponseError for Error {
  fn status_code(&self) -> actix_web::http::StatusCode {
    match self {
      Error::FileError(err) => err.status_code(),
      Error::YAMLConvertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::JsonConvertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

use std::fmt::{Display, Formatter};
use std::io;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

use self::Error::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
  User(String),
  Unexpected(String),
  Custom(String, StatusCode),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<fdroid::error::Error> for Error {
  fn from(value: fdroid::error::Error) -> Self {
    match value {
      fdroid::error::Error::File(error) => {
        Unexpected(format!("An Unexpected File Error occurred: {}", error))
      }
      fdroid::error::Error::YAMLConvert(error) => Unexpected(format!(
        "An Unexpected YAML-Convert Error occurred: {}",
        error
      )),
      fdroid::error::Error::JsonConvert(error) => Unexpected(format!(
        "An Unexpected JSON-Convert Error occurred: {}",
        error
      )),
      fdroid::error::Error::InvalidFile(_) => Unexpected(format!(
        "An Unexpected Invalid File Error occurred: {}",
        value
      )),
      fdroid::error::Error::NotAFile(_) => Unexpected(format!(
        "An Unexpected Invalid File Error occurred: {}",
        value
      )),
      fdroid::error::Error::NotADirectory(_) => Unexpected(format!(
        "An Unexpected Invalid Directory Error occurred: {}",
        value
      )),
      fdroid::error::Error::Init => {
        Unexpected("An error occurred while initializing the repository!".to_string())
      }
      fdroid::error::Error::Update => {
        Unexpected("An error occurred while updating the repository!".to_string())
      }
      fdroid::error::Error::Run(_) => Unexpected(format!(
        "An error occurred while running a command: {}",
        value
      )),
    }
  }
}

impl From<io::Error> for Error {
  fn from(value: io::Error) -> Self {
    User(format!("An Unexpected IO-Error occurred: {}", value))
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      User(message) => write!(f, "UserError: \"{}\"", message),
      Unexpected(message) => write!(f, "UnexpectedError: \"{}\"", message),
      Custom(message, _error_code) => write!(f, "{}", message),
    }
  }
}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match self {
      User(_) => StatusCode::BAD_REQUEST,
      Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Custom(_, status_code) => *status_code,
    }
  }
}

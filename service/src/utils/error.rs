use self::Error::*;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug, Clone)]
pub enum Error {
  UserError(String),
  UnexpectedError(String),
  Custom(String, StatusCode),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<fdroid::error::Error> for Error {
  fn from(value: fdroid::error::Error) -> Self {
    match value {
      fdroid::error::Error::File(error) => UnexpectedError(format!(
        "An Unexpected File Error occurred: {}",
        error.to_string()
      )),
      fdroid::error::Error::YAMLConvert(error) => UnexpectedError(format!(
        "An Unexpected YAML-Convert Error occurred: {}",
        error.to_string()
      )),
      fdroid::error::Error::JsonConvert(error) => UnexpectedError(format!(
        "An Unexpected JSON-Convert Error occurred: {}",
        error.to_string()
      )),
      fdroid::error::Error::Custom(message) => UnexpectedError(message),
      fdroid::error::Error::User(message) => UserError(message),
    }
  }
}

impl From<io::Error> for Error {
  fn from(value: io::Error) -> Self {
    UserError(format!(
      "An Unexpected IO-Error occurred: {}",
      value.to_string()
    ))
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      UserError(message) => write!(f, "UserError: \"{}\"", message),
      UnexpectedError(message) => write!(f, "UnexpectedError: \"{}\"", message),
      Custom(message, _error_code) => write!(f, "{}", message),
    }
  }
}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match self {
      UserError(_) => StatusCode::BAD_REQUEST,
      UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Custom(_, status_code) => status_code.clone(),
    }
  }
}

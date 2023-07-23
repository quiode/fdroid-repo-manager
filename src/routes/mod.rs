use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;

pub mod app;
pub mod config;

#[derive(MultipartForm)]
pub struct FileUploadForm {
  pub app: TempFile,
}

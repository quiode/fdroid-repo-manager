// Warns if docs are missing
#![warn(missing_docs)]

mod aapt;
pub mod error;
mod repository;

// Re-Export
pub use repository::*;

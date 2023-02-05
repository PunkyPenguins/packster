mod error;
mod project_manifest;

type RepresentationResult<T> = std::result::Result<T, error::RepresentationError>;

pub use error::RepresentationError;
pub use project_manifest::*;


mod error;
mod portage_manifest;

type RepresentationResult<T> = std::result::Result<T, error::RepresentationError>;

pub use error::RepresentationError;
pub use portage_manifest::*;


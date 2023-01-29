mod error;
mod portage_manifest;

type DtoResult<T> = std::result::Result<T, error::DtoError>;

pub use error::DtoError;
pub use portage_manifest::*;


use std::{
    fmt,
    error,
    io, path::PathBuf
};
use crate::mandatory::dto::DtoError;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DtoError(DtoError),
    ManifesPathIsADirectory(PathBuf),
    ManifestPathDoesNotExist(PathBuf)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Io(e) => write!(f, "Io error : {}", e),
            DtoError(e) => write!(f, "DTO error : {}", e),
            ManifesPathIsADirectory(path) => write!(f, "Manifest path is not a directory : {}", path.to_string_lossy()),
            ManifestPathDoesNotExist(path) => write!(f, "Manifest path does not exist : {}", path.to_string_lossy()),

        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Io(e) => Some(e),
            DtoError(e) => Some(e),
            ManifesPathIsADirectory(e) => None,
            ManifestPathDoesNotExist(e) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Error::Io(error) }
}


impl From<DtoError> for Error {
    fn from(error: DtoError) -> Self { Error::DtoError(error) }
}


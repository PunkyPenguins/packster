use std::{
    fmt,
    error,
    io, path::PathBuf
};
use crate::application::RepresentationError;

#[derive(Debug)]
pub enum Error { //TODO split errors into each tier
    Io(io::Error), //would be infra
    WalkDir(walkdir::Error),
    DtoError(RepresentationError), //would be app
    ManifesPathIsADirectory(PathBuf), //would be biz
    ManifestPathDoesNotExist(PathBuf) //would be biz
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Io(e) => write!(f, "Io error : {e}"),
            WalkDir(e) => write!(f, "WalkDir error : {e}"),
            DtoError(e) => write!(f, "DTO error : {e}"),
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
            WalkDir(e) => Some(e),
            DtoError(e) => Some(e),
            ManifesPathIsADirectory(_) => None,
            ManifestPathDoesNotExist(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Error::Io(error) }
}

impl From<walkdir::Error> for Error {
    fn from(error: walkdir::Error) -> Self { Error::WalkDir(error) }
}

impl From<RepresentationError> for Error {
    fn from(error: RepresentationError) -> Self { Error::DtoError(error) }
}


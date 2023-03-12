use std::{fmt,error, path::PathBuf};

use crate::Absolute;

#[derive(Debug)]
pub enum Error {
    Infrastructure(Box<dyn error::Error>),
    Application(Box<dyn error::Error>),
    ManifesPathIsADirectory(PathBuf),
    ManifestPathDoesNotExist(PathBuf),
    MissingMandatoryField { entity_name: &'static str, field_name: &'static str },
    BaseNotInPath { base: Absolute<PathBuf>, path: Absolute<PathBuf> },
    PathIsAbsolute(PathBuf),
    PathIsRelative(PathBuf),
    LocationPathIsNotADirectory(PathBuf),
    LocationManifestPathIsNotAFile(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Infrastructure(error) => write!(f, "Infrastructure error : {error}"),
            Application(error) => write!(f, "Application error : {error}"),
            ManifesPathIsADirectory(path) => write!(f, "Manifest path is not a directory : {}", path.to_string_lossy()),
            ManifestPathDoesNotExist(path) => write!(f, "Manifest path does not exist : {}", path.to_string_lossy()),
            MissingMandatoryField { entity_name, field_name } => write!(f, "Missing infrastructure field {entity_name} for entity {field_name}"),
            PathIsAbsolute(path) => write!(f, "Path is absolute : {}", path.to_string_lossy()),
            PathIsRelative(path) => write!(f, "Path is relative : {}", path.to_string_lossy()),
            BaseNotInPath { base, path } => write!(f, "Base \"{}\" not in path \"{}\"", base.as_ref().to_string_lossy(), path.as_ref().to_string_lossy()),
            LocationPathIsNotADirectory(path) => write!(f, "Location path exists but is not a directory {}", path.to_string_lossy()),
            LocationManifestPathIsNotAFile(path) => write!(f, "Location manifest path exists but is not a file {}", path.to_string_lossy())
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Infrastructure(error) => Some(error.as_ref()),
            Application(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}
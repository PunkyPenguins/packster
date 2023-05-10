use std::{fmt,error, path::PathBuf};

use crate::path::Absolute;

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
    PackageChecksumDoNotMatch{package_path: PathBuf, package_id: String, package_checksum: String},
    PackageAlreadyDeployedInLocation(String),
    AncestorIsAFile{ancestor: PathBuf, path: PathBuf},
    NodeAlreadyExists(PathBuf)
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
            LocationManifestPathIsNotAFile(path) => write!(f, "Location manifest path exists but is not a file {}", path.to_string_lossy()),
            PackageChecksumDoNotMatch{ package_path, package_id, package_checksum } => write!(
                f,
                "Package {} checksum {} does not match with file {}",
                package_id,
                package_checksum,
                package_path.to_string_lossy()
            ),
            PackageAlreadyDeployedInLocation(package_id) => write!(f,"Package {package_id} already exists in location"),
            AncestorIsAFile{ ancestor, path } => write!(f, "Ancestor {} of {} is a file", ancestor.to_string_lossy(), path.to_string_lossy()),
            NodeAlreadyExists(path) => write!(f,"Resource {} already exists", path.to_string_lossy()),
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
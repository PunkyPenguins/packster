use std::{fmt,error, path::{PathBuf, StripPrefixError}};

#[derive(Debug)]
pub enum Error {
    Infrastructure(Box<dyn error::Error>),
    Application(Box<dyn error::Error>),
    ManifesPathIsADirectory(PathBuf),
    ManifestPathDoesNotExist(PathBuf),
    MissingMandatoryField { entity_name: &'static str, field_name: &'static str },
    StripPrefixError(StripPrefixError),
    PathIsAbsolute(PathBuf),
    PathIsRelative(PathBuf),
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
            StripPrefixError(error) => write!(f, "Strip prefix error : {error}")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Infrastructure(error) => Some(error.as_ref()),
            Application(error) => Some(error.as_ref()),
            StripPrefixError(error) => Some(error),
            _ => None,
        }
    }
}


impl From<StripPrefixError> for Error {
    fn from(error: StripPrefixError) -> Self { Error::StripPrefixError(error) }
}
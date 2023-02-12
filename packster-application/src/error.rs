use std::{
    fmt,
    error,
    io, path::PathBuf,
};

use toml;


#[derive(Debug)]
pub enum Error {
    TomlDeserialize(toml::de::Error),
    ManifesPathIsADirectory(PathBuf),
    ManifestPathDoesNotExist(PathBuf),
    MissingMandatoryField { entity_name: &'static str, field_name: &'static str }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            TomlDeserialize(error) => write!(f, "Toml deserialization error : {error}"),
            ManifesPathIsADirectory(path) => write!(f, "Manifest path is not a directory : {}", path.to_string_lossy()),
            ManifestPathDoesNotExist(path) => write!(f, "Manifest path does not exist : {}", path.to_string_lossy()),
            MissingMandatoryField { entity_name, field_name } => write!(f, "Missing infrastructure field {entity_name} for entity {field_name}")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            TomlDeserialize(error) => Some(error),
            ManifesPathIsADirectory(_) => None,
            ManifestPathDoesNotExist(_) => None,
            MissingMandatoryField { .. } => None
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self { Error::TomlDeserialize(error) }
}

impl From<Error> for packster_core::error::Error {
    fn from(value: Error) -> Self {
        packster_core::error::Error::Application(Box::new(value))
    }
}
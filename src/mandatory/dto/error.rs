use std::{
    fmt,
    error
};
use toml;

#[derive(Debug)]
pub enum DtoError {
    DeserializeToml(toml::de::Error),
    MissingMandatoryField { entity_name: &'static str, field_name: &'static str }
}

impl fmt::Display for DtoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DtoError::*;
        match self {
            MissingMandatoryField { entity_name, field_name } => write!(f, "Missing mandatory field {} for entity {}", entity_name, field_name),
            DeserializeToml(e) => write!(f, "Deserialize toml error : {}", e),
        }
    }
}

impl error::Error for DtoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use DtoError::*;
        match self {
            MissingMandatoryField { .. } => None,
            DeserializeToml(e) => Some(e),
        }
    }
}

impl From<toml::de::Error> for DtoError {
    fn from(error: toml::de::Error) -> Self { DtoError::DeserializeToml(error) }
}

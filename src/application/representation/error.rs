use std::{
    fmt,
    error
};
use toml;

#[derive(Debug)]
pub enum RepresentationError {
    DeserializeToml(toml::de::Error),
    MissingMandatoryField { entity_name: &'static str, field_name: &'static str }
}

impl fmt::Display for RepresentationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RepresentationError::*;
        match self {
            MissingMandatoryField { entity_name, field_name } => write!(f, "Missing infrastructure field {entity_name} for entity {field_name}"),
            DeserializeToml(e) => write!(f, "Deserialize toml error : {e}"),
        }
    }
}

impl error::Error for RepresentationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use RepresentationError::*;
        match self {
            MissingMandatoryField { .. } => None,
            DeserializeToml(e) => Some(e),
        }
    }
}

impl From<toml::de::Error> for RepresentationError {
    fn from(error: toml::de::Error) -> Self { RepresentationError::DeserializeToml(error) }
}

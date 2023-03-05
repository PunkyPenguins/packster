use std::{
    fmt,
    error,
    io,
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    WalkDir(walkdir::Error),
    TomlDeserialize(toml::de::Error),
    TomlSerialize(toml::ser::Error),
    JsonSerde(serde_json::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Io(e) => write!(f, "Io error : {e}"),
            WalkDir(e) => write!(f, "WalkDir error : {e}"),
            TomlDeserialize(e) => write!(f, "Toml deserialize error : {e}"),
            TomlSerialize(e) => write!(f, "Toml serialize error : {e}"),
            JsonSerde(e) => write!(f, "Json deserialize error: {e}")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Io(e) => Some(e),
            WalkDir(e) => Some(e),
            TomlDeserialize(e) => Some(e),
            TomlSerialize(e) => Some(e),
            JsonSerde(e) => Some(e),
        }
    }
}

impl From<Error> for packster_core::Error {
    fn from(value: Error) -> Self {
        packster_core::Error::Infrastructure(Box::new(value))
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Error::Io(error) }
}

impl From<walkdir::Error> for Error {
    fn from(error: walkdir::Error) -> Self { Error::WalkDir(error) }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self { Error::TomlDeserialize(error) }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self { Error::TomlSerialize(error) }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self { Error::JsonSerde(error) }
}

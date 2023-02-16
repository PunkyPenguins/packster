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
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Io(e) => write!(f, "Io error : {e}"),
            WalkDir(e) => write!(f, "WalkDir error : {e}"),
            TomlDeserialize(e) => write!(f, "Toml deserialize error : {e}")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Io(e) => Some(e),
            WalkDir(e) => Some(e),
            TomlDeserialize(e) => Some(e)
        }
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

impl From<Error> for packster_core::error::Error {
    fn from(value: Error) -> Self {
        packster_core::error::Error::Infrastructure(Box::new(value))
    }
}
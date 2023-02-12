use std::{fmt,error};

#[derive(Debug)]
pub enum Error {
    Infrastructure(Box<dyn error::Error>),
    Application(Box<dyn error::Error>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Infrastructure(error) => write!(f, "Infrastructure error : {}", error),
            Application(error) => write!(f, "Application error : {}", error),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            Infrastructure(error) => Some(error.as_ref()),
            Application(error) => Some(error.as_ref()),
        }
    }
}
use packster_core::{Result, port::{Parser, Serializer}};
use serde::{de::DeserializeOwned, Serialize};
use toml::{from_str, to_string};

use crate::Error;
pub struct Toml;

//TODO add some logging and integration tests
impl Parser for Toml {
    fn parse<S: AsRef<str>, T: DeserializeOwned>(&self, s: S) -> Result<T> {
        Ok(from_str(s.as_ref()).map_err(Error::from)?)
    }
}

impl Serializer for Toml {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<String> {
        Ok(to_string(value).map_err(Error::from)?)
    }
}
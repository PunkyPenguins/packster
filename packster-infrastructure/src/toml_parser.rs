use packster_core::{Result, Parser};
use serde::de::DeserializeOwned;
use toml::from_str;

use crate::Error;
pub struct TomlParser;

//TODO add some logging and integration tests
impl Parser for TomlParser {
    fn parse<S: AsRef<str>, T: DeserializeOwned>(&self, s: S) -> Result<T> {
        Ok(from_str(s.as_ref()).map_err(Error::from)?)
    }
}
use packster_core::{Result, Parser};
use serde::de::DeserializeOwned;
use toml;

use crate::Error;
pub struct TomlParser;

impl Parser for TomlParser {
    fn parse<S: AsRef<str>, T: DeserializeOwned>(&self, s: S) -> Result<T> {
        Ok(toml::from_str(s.as_ref()).map_err(Error::from)?)
    }
}
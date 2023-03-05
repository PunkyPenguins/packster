use packster_core::{Result, Parser, Serializer};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{ from_str, to_string } ;

use crate::Error;

pub struct Json;

impl Parser for Json {
    fn parse<S: AsRef<str>, T: DeserializeOwned>(&self, s: S) -> Result<T> {
        Ok(from_str(s.as_ref()).map_err(Error::from)?)
    }
}

impl Serializer for Json {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<String> {
        Ok(to_string(value).map_err(Error::from)?)
    }
}
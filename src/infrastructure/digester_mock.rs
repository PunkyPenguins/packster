use std::{io::Read, fmt};

use crate::{Result, essential::port::IDigester};


pub struct DigesterMock(pub String);

impl DigesterMock {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        DigesterMock(s.as_ref().to_string())
    }
}

impl IDigester for DigesterMock {
    fn generate_checksum<R: Read>(&self, _reader: R) -> Result<Vec<u8>> {
        let decoded = hex::decode(&self.0).unwrap();
        Ok(decoded)
    }
}

impl fmt::Display for DigesterMock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mock")
    }
}
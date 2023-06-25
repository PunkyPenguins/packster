pub mod entity {
    use core::fmt;
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};

    use crate::{Error, Result};

    #[cfg(test)]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Identifier(pub String);

    #[cfg(not(test))]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Identifier(String);

    impl FromStr for Identifier {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self> {
            Ok(Identifier(s.to_string())) //TODO proper identifier validation
        }
    }

    impl fmt::Display for Identifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl AsRef<str> for Identifier {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    #[cfg(test)]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Version(pub String);

    #[cfg(not(test))]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
    pub struct Version(String);

    impl Version {
        pub fn new<S: AsRef<str>>(version_str: S) -> Self {
            Version(version_str.as_ref().to_owned())
        }

        pub fn as_bytes(&self) -> &[u8] {
            self.0.as_bytes()
        }
    }

    impl fmt::Display for Version {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl FromStr for Version {
        type Err = Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Ok(Version(s.to_string())) //TODO enforce semver through Version type ( from_str )
        }
    }

    impl AsRef<str> for Version {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
    pub struct Checksum(#[serde(with = "hex")] Vec<u8>);

    impl FromStr for Checksum {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self> {
            Ok(hex::decode(s).map(Checksum)?)
        }
    }

    impl ToString for Checksum {
        fn to_string(&self) -> String {
            hex::encode(&self.0)
        }
    }

    impl AsRef<[u8]> for Checksum {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl From<Vec<u8>> for Checksum {
        fn from(value: Vec<u8>) -> Self {
            Checksum(value)
        }
    }
}

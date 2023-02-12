use semver::Version;

pub struct Identifier(String);

pub struct Project {
    identifier: Identifier,
    version: Version
}

impl Project {
    fn new(identifier: Identifier, version: Version) -> Self {
        Project {
            identifier,
            version
        }
    }
}
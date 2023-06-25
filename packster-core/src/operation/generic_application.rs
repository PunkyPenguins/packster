use crate::{entity::{DeployLocation, Package}, port::{Serializer, Parser, FileSystem, ReadOnlyFileSystem, Digester}, Result, Error};
use super::{Operation, AsLocationPath, AsPackagePath, AsPackage, AsLocation};

pub struct ParsedPackage<P> {
    pub previous_state: P,
    pub package: Package
}

//Example about how to factorize common behaviors such as parsing package from path, providing an extra level of indirection
impl <S, R>Operation<S, R> where Self: AsPackagePath {
    pub fn parse_package_path(self) -> Result<Operation<ParsedPackage<S>, R>> {
        let package = Package::from_path(self.as_package_path())?;
        Self::ok_with_state(
            self.request,
            ParsedPackage {
                previous_state: self.state,
                package
            }
        )
    }
}

// Simple field accessor
impl <S>AsPackage for ParsedPackage<S> {
    fn as_package(&self) -> &Package { &self.package }
}

// Propagate through previous state
impl <S: AsLocation> AsLocation for ParsedPackage<S> {
    fn as_location(&self) -> &DeployLocation { self.previous_state.as_location() }
}


pub struct ParsedLocation<S> {
    pub previous_state: S,
    pub location: DeployLocation
}

impl <S, R>Operation<S, R> where Self: AsLocationPath {
    pub fn parse_location_lockfile<F: ReadOnlyFileSystem, P: Parser>(self, filesystem: &F, parser: &P) -> Result<Operation<ParsedLocation<S>, R>> {
        let lockfile_path = self.to_location_lockfile_path();
        let lockfile_content = filesystem.read_to_string(lockfile_path)?;
        Self::ok_with_state(
            self.request,
            ParsedLocation {
                previous_state: self.state,
                location: parser.parse(lockfile_content)?
            }
        )
    }
}

impl <S>AsLocation for ParsedLocation<S> {
    fn as_location(&self) -> &DeployLocation { &self.location }
}

impl <S: AsPackage>AsPackage for ParsedLocation<S> {
    fn as_package(&self) -> &Package { self.previous_state.as_package() }
}


pub struct MatchingChecksum<S> { pub previous_state: S }

impl <S, R>Operation<S, R> where Self: AsPackagePath + AsPackage {
    pub fn validate_package_checksum<F: ReadOnlyFileSystem, D: Digester>(self, filesystem: &F, digester: &D) -> Result<Operation<MatchingChecksum<S>, R>> {
        let package_path = self.as_package_path();
        let digest = digester.generate_checksum(filesystem.open_read(&package_path)?)?;
        if digest == *self.as_package().as_checksum() {
            Self::ok_with_state(self.request, MatchingChecksum { previous_state: self.state })
        } else {
            Err(
                Error::PackageChecksumDoNotMatch {
                    package_path: package_path.to_path_buf(),
                    package_id: self.as_package().as_identifier().to_string(),
                    package_checksum: self.as_package().as_checksum().to_string()
                }
            )
        }
    }
}

impl <S: AsPackage>AsPackage for MatchingChecksum<S> {
    fn as_package(&self) -> &Package { self.previous_state.as_package() }
}

impl <S: AsLocation>AsLocation for MatchingChecksum<S> {
    fn as_location(&self) -> &DeployLocation { self.previous_state.as_location() }
}


pub struct PersistedDeployLocation<P> { pub previous_state: P }

impl <S, R> Operation<S, R> where Self: AsLocationPath + AsRef<DeployLocation> {
    pub fn persist_location_lockfile<F: FileSystem, Sr: Serializer> (self, filesystem: &F, serializer: &Sr) -> Result<Operation<PersistedDeployLocation<S>, R>> {
        let lockfile_location = self.to_location_lockfile_path();
        let location = self.as_ref();
        let deploy_location_file_content = serializer.serialize(location)?;
        filesystem.write_all(lockfile_location, deploy_location_file_content.as_bytes())?;
        Self::ok_with_state(self.request, PersistedDeployLocation { previous_state: self.state })
    }
}
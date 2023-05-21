use std::path::PathBuf;

use crate::{domain::{DeployLocation, Package, Deployment}, port::{Parser, ReadOnlyFileSystem, Digester}, Error, Result, path::Absolute};

use super::{AsPackagePath, AsLocation, AsPackage, AsPathLocation, Operation, AsChecksum};


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

impl <S, R>Operation<S, R> where Self: AsPathLocation {
    pub fn parse_location_lockfile<F: ReadOnlyFileSystem, P: Parser>(self, filesystem: &F, parser: &P) -> Result<Operation<ParsedLocation<S>, R>> {
        let lockfile_path = self.to_lockfile_location();
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


pub struct NotYetDeployed<S> { pub previous_state: S }

impl <S, R>Operation<S, R> where Self: AsPackage + AsLocation {
    pub fn probe_package_not_deployed_in_location(self) -> Result<Operation<NotYetDeployed<S>, R>> {
        if self.as_location().is_checksum_deployed(self.as_package().as_checksum()) {
            Err(Error::PackageAlreadyDeployedInLocation(self.as_package().as_identifier().to_string()))
        } else {
            Self::ok_with_state(self.request, NotYetDeployed { previous_state: self.state })
        }
    }
}

impl <S: AsPackage>AsPackage for NotYetDeployed<S> {
    fn as_package(&self) -> &Package { self.previous_state.as_package() }
}

impl <S: AsLocation>AsLocation for NotYetDeployed<S> {
    fn as_location(&self) -> &DeployLocation { self.previous_state.as_location() }
}

pub struct AlreadyDeployed<S> { pub previous_state: S, pub existing_deployment: Deployment }

impl <S, R>Operation<S, R> where Self: AsChecksum + AsLocation {
    pub fn probe_package_already_deployed_in_location(self) -> Result<Operation<AlreadyDeployed<S>, R>> {
        if let Some(existing_deployment) = self.as_location().get_deployment(self.as_checksum()).cloned() {
            Self::ok_with_state(self.request, AlreadyDeployed { previous_state: self.state, existing_deployment })
        } else {
            Err(Error::PackageNotYetDeployedInLocation(self.as_checksum().to_string()))
        }
    }
}

impl <S: AsPackage>AsPackage for AlreadyDeployed<S> {
    fn as_package(&self) -> &Package { self.previous_state.as_package() }
}

impl <S: AsLocation>AsLocation for AlreadyDeployed<S> {
    fn as_location(&self) -> &DeployLocation { self.previous_state.as_location() }
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


pub struct DeploymentPath<S> { pub previous_state: S, pub deployment_path: Absolute<PathBuf> }

impl <S, R> Operation<S, R> where Self: AsChecksum + AsPathLocation {
    pub fn guess_deployment_path(self) -> Operation<DeploymentPath<S>, R> {
        let checksum_string = self.as_checksum().to_string();
        let deployment_path = self.as_path_location().join(checksum_string);
        Self::with_state(
            self.request,
            DeploymentPath {
                previous_state: self.state,
                deployment_path
            }
        )
    }
}

impl <S: AsPackage>AsPackage for DeploymentPath<S> {
    fn as_package(&self) -> &Package { self.previous_state.as_package() }
}

impl <S: AsLocation>AsLocation for DeploymentPath<S> {
    fn as_location(&self) -> &DeployLocation { self.previous_state.as_location() }
}
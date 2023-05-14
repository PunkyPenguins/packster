#![allow(dead_code)]
mod pack;
use std::path::{Path, PathBuf};

pub use pack::*;

mod init_location;
pub use init_location::*;

mod deploy;
pub use deploy::*;

use crate::{path::Absolute, LOCKFILE_NAME, Error, port::{ReadOnlyFileSystem, Parser, Digester}, domain::{DeployLocation, Package}, Result};

pub struct Operation<S, R>{
    request: R,
    state: S
}

pub struct New;

//TODO doing rollback operation using same states newtype struct would be super easy and meaningful and stylish cause each state would be described as reversible, or not, or partially, or with different methods :O
//TODO R&D : consider replace Operation by a State Trait to simplify architecture and state machine building flexibility
impl <S, R>Operation<S, R> {
    /*Create a new operation */
    pub fn new(request: R, state: S) -> Self {
        Operation { request, state }
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_request(&self) -> &R {
        &self.request
    }

    /*Return same operation ( same request ) with a new state */
    pub (crate) fn with_state<N>(request: R, state: N) -> Operation<N, R> {
        Operation { state, request }
    }

    /*Return a new operation ( new request ) but the same state */
    pub (crate) fn with_request<N>(request: N, state: S) -> Operation<S, N> {
        Operation { request, state }
    }

    pub (crate) fn ok_with_state<N>(request: R, state: N) -> Result<Operation<N, R>> {
        Ok(Self::with_state(request, state))
    }
}

//---------------------------------------------------------------------------------

pub trait AsPackagePath {
    fn as_package_path(&self) -> Absolute<&Path>;
}

pub trait AsPackage {
    fn as_package(&self) -> &Package;
}

pub struct ParsedPackage<S> {
    pub state: S,
    pub package: Package
}

//Example about how to factorize common behaviors such as parsing package from path, providing an extra level of indirection
impl <S, R: AsPackagePath>Operation<S, R> {
    pub fn parse_package_path(self) -> Result<Operation<ParsedPackage<S>, R>> {
        let package = Package::from_path(self.request.as_package_path()); //TODO try ?
        Self::ok_with_state(
            self.request,
            ParsedPackage { state: self.state, package }
        )
    }
}

impl <S>AsPackage for ParsedPackage<S> {
    fn as_package(&self) -> &Package { &self.package }
}

impl <S>AsLocation for ParsedPackage<ParsedLocation<S>> {
    fn as_location(&self) -> &DeployLocation { &self.state.location }
}

//---------------------------------------------------------------------------------

pub trait AsPathLocation {
    fn as_path_location(&self) -> Absolute<&Path>;
    fn to_lockfile_location(&self) -> Absolute<PathBuf> {
        self.as_path_location().join(LOCKFILE_NAME)
    }
}

pub trait AsLocation {
    fn as_location(&self) -> &DeployLocation;
}

pub struct ParsedLocation<S> {
    pub state: S,
    pub location: DeployLocation
}

impl <S, R: AsPathLocation>Operation<S, R> {
    pub fn parse_location_lockfile<F: ReadOnlyFileSystem, P: Parser>(self, filesystem: &F, parser: &P) -> Result<Operation<ParsedLocation<S>, R>> {
        let lockfile_path = self.request.to_lockfile_location();
        let lockfile_content = filesystem.read_to_string(lockfile_path)?;
        Self::ok_with_state(
            self.request,
            ParsedLocation {
                state: self.state,
                location: parser.parse(lockfile_content)?
            }
        )
    }
}

impl <S>AsLocation for ParsedLocation<S> {
    fn as_location(&self) -> &DeployLocation { &self.location }
}

impl <S>AsPackage for ParsedLocation<ParsedPackage<S>> {
    fn as_package(&self) -> &Package { &self.state.package }
}

//---------------------------------------------------------------------------------

pub struct NotYetDeployed<S> { pub not_yet_deployed: S }

impl <S: AsPackage + AsLocation, R>Operation<S, R> {
    pub fn probe_package_not_deployed_in_location(self) -> Result<Operation<NotYetDeployed<S>, R>> {
        if self.state.as_location().is_checksum_deployed(self.state.as_package().as_checksum()) {
            Err(Error::PackageAlreadyDeployedInLocation(self.state.as_package().as_identifier().to_string()))
        } else {
            Self::ok_with_state(self.request, NotYetDeployed { not_yet_deployed: self.state })
        }
    }
}

impl <S: AsPackage>AsPackage for NotYetDeployed<S> {
    fn as_package(&self) -> &Package {
        self.not_yet_deployed.as_package()
    }
}

impl <S: AsLocation>AsLocation for NotYetDeployed<S> {
    fn as_location(&self) -> &DeployLocation {
        self.not_yet_deployed.as_location()
    }
}

//---------------------------------------------------------------------------------

pub struct MatchingChecksum<S> { pub matching_checksum: S }

impl <S: AsPackage, R: AsPackagePath>Operation<S, R> {
    pub fn validate_package_checksum<F: ReadOnlyFileSystem, D: Digester>(self, filesystem: &F, digester: &D) -> Result<Operation<MatchingChecksum<S>, R>> {
        let package_path = self.request.as_package_path();
        let digest = digester.generate_checksum(filesystem.open_read(&package_path)?)?;
        if digest == *self.state.as_package().as_checksum() {
            Self::ok_with_state(self.request, MatchingChecksum { matching_checksum: self.state })
        } else {
            Err(
                Error::PackageChecksumDoNotMatch {
                    package_path: package_path.as_ref().to_path_buf(),
                    package_id: self.state.as_package().as_identifier().to_string(),
                    package_checksum: self.state.as_package().as_checksum().to_string()
                }
            )
        }
    }
}

impl <S: AsPackage>AsPackage for MatchingChecksum<S> {
    fn as_package(&self) -> &Package {
        self.matching_checksum.as_package()
    }
}

use std::path::PathBuf;

use crate::{domain::{DeployLocation, Package, Deployment}, Error, Result, path::Absolute};

use super::{ AsLocation, AsPackage, AsLocationPath, Operation, AsChecksum};

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


pub struct DeploymentPath<S> { pub previous_state: S, pub deployment_path: Absolute<PathBuf> }

impl <S, R> Operation<S, R> where Self: AsChecksum + AsLocationPath {
    pub fn guess_deployment_path(self) -> Operation<DeploymentPath<S>, R> {
        let checksum_string = self.as_checksum().to_string();
        let deployment_path = self.as_location_path().join(checksum_string);
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
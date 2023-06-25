use std::path::{Path, PathBuf};

use crate::{port::{FileSystem, Archiver}, domain::{Package, Deployment, DeployLocation, Checksum}, Result, path::Absolute};

use super::{
    Operation,
    New,
    AsPackagePath,
    AsLocationPath,
    ParsedPackage,
    ParsedLocation,
    NotYetDeployed,
    MatchingChecksum,
    DeploymentPath,
    AsChecksum,
    AsPackage,
    UpdatedDeployLocation, PersistedDeployLocation
};

pub struct DeployRequest {
    package_path: Absolute<PathBuf>,
    location_path: Absolute<PathBuf>
}

impl DeployRequest {
    pub fn new(package_path: Absolute<PathBuf>, location_path: Absolute<PathBuf>) -> Self {
        DeployRequest { package_path, location_path }
    }
}


pub type DeployOperation<S> = Operation<S, DeployRequest>;

impl <S>AsPackagePath for DeployOperation<S> {
    fn as_package_path(&self) -> Absolute<&Path> { self.request.package_path.as_absolute_path() }
}

impl <S>AsLocationPath for DeployOperation<S> {
    fn as_location_path(&self) -> Absolute<&Path> { self.request.location_path.as_absolute_path() }
}

impl <S: AsPackage>AsChecksum for DeployOperation<S> {
    fn as_checksum(&self) -> &Checksum { self.state.as_package().as_checksum() }
}

pub type DeployValidState = DeploymentPath<MatchingChecksum<NotYetDeployed<ParsedLocation<ParsedPackage<New>>>>>;

impl AsMut<DeployLocation> for DeployValidState {
    fn as_mut(&mut self) -> &mut DeployLocation {
        &mut self.previous_state.previous_state.previous_state.location
    }
}

pub struct ExtractedPackage {
    previous_state: DeployValidState
}

impl DeployOperation<DeployValidState> {
    pub fn extract_package<F: FileSystem, A: Archiver>(self, filesystem: &F, archiver: &A) -> Result<DeployOperation<ExtractedPackage>> {
        archiver.extract(
            filesystem,
            self.state.deployment_path.as_absolute_path(),
            self.request.package_path.as_absolute_path()
        )?;
        Self::ok_with_state(self.request, ExtractedPackage { previous_state: self.state })
    }
}

pub struct DeployedDeployment {
    previous_state: ExtractedPackage,
    deployment: Deployment
}

impl DeployOperation<ExtractedPackage> {
    fn as_mut_location(&mut self) -> &mut DeployLocation {
        &mut self.state.previous_state.previous_state.previous_state.previous_state.location
    }

    pub fn add_deployment_to_location(mut self) -> DeployOperation<UpdatedDeployLocation<DeployedDeployment>> {
        let package = self.state.previous_state.as_package();
        let deployment: Deployment = Deployment::new(package.clone());

        let location = self.as_mut_location();
        location.add_deployment(deployment.clone());

        Self::with_state(
            self.request,
            UpdatedDeployLocation {
                previous_state: DeployedDeployment { previous_state: self.state, deployment }
            }
        )
    }
}

impl AsRef<DeployLocation> for DeployOperation<UpdatedDeployLocation<DeployedDeployment>> {
    fn as_ref(&self) -> &DeployLocation {
        &self.state.previous_state.previous_state.previous_state.previous_state.previous_state.previous_state.location
    }
}

impl DeployOperation<PersistedDeployLocation<DeployedDeployment>> {
    pub fn as_deploy_path(&self) -> Absolute<&Path> { self.state.previous_state.previous_state.previous_state.deployment_path.as_absolute_path() }
    pub fn as_deployment(&self) -> &Deployment { &self.state.previous_state.deployment }
    pub fn as_package(&self) -> &Package { self.state.previous_state.previous_state.previous_state.as_package() }
}
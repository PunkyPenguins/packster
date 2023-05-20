use std::path::{Path, PathBuf};

use crate::{port::{FileSystem, Archiver, Serializer}, domain::{Package, Deployment, DeployLocation, Checksum}, Result, path::Absolute};

use super::{Operation, New, AsPackagePath, AsPathLocation,  ParsedPackage, ParsedLocation, NotYetDeployed, MatchingChecksum, DeploymentPath, AsChecksum};

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

impl <S>AsPathLocation for DeployOperation<S> {
    fn as_path_location(&self) -> Absolute<&Path> { self.request.location_path.as_absolute_path() }
}

impl AsChecksum for DeployOperation<MatchingChecksum<NotYetDeployed<ParsedLocation<ParsedPackage<New>>>>> {
    fn as_checksum(&self) -> &Checksum {
        self.state.previous_state.previous_state.previous_state.package.as_checksum()
    }
}

pub type ValidState = DeploymentPath<MatchingChecksum<NotYetDeployed<ParsedLocation<ParsedPackage<New>>>>>;
pub struct ValidStateWrapper(ValidState);

impl From<ValidState> for ValidStateWrapper {
    fn from(value: ValidState) -> Self { ValidStateWrapper(value) }
}

impl ValidStateWrapper {
    fn as_deployment_path(&self) -> Absolute<&Path> { self.0.deployment_path.as_absolute_path() }
    fn as_package(&self) -> &Package { &self.0.previous_state.previous_state.previous_state.previous_state.package }
    fn as_mut_location(&mut self) -> &mut DeployLocation { &mut self.0.previous_state.previous_state.previous_state.location }
    fn as_location(&self) -> &DeployLocation { &self.0.previous_state.previous_state.previous_state.location }
}

pub struct ExtractedPackage {
    previous_state: ValidStateWrapper
}

//An emerging good practise here : keep read operations as generic as possible and write operation as specific as possible
impl DeployOperation<ValidStateWrapper> {
    pub fn extract_package<F: FileSystem, A: Archiver>(self, filesystem: &F, archiver: &A) -> Result<DeployOperation<ExtractedPackage>> {
        archiver.extract(
            filesystem,
            self.state.as_deployment_path(),
            self.request.package_path.as_absolute_path()
        )?;
        Self::ok_with_state(self.request, ExtractedPackage { previous_state: self.state })
    }
}

pub struct LockfileUpdated {
    previous_state: ExtractedPackage,
    deployment: Deployment
}

impl DeployOperation<ExtractedPackage> {
    pub fn update_location_lockfile<F: FileSystem, Sr: Serializer>(mut self, filesystem: &F, serializer: &Sr) -> Result<DeployOperation<LockfileUpdated>> {
        let package = self.state.previous_state.as_package();
        let deployment: Deployment = Deployment::new(package.as_checksum().clone());

        let location = &mut self.state.previous_state.as_mut_location();
        location.add_deployment(deployment.clone());

        let deploy_location_file_content = serializer.serialize(&location)?;
        let lockfile_location = self.to_lockfile_location();
        filesystem.write_all(lockfile_location, deploy_location_file_content.as_bytes())?;

        Self::ok_with_state(self.request, LockfileUpdated { previous_state: self.state, deployment })
    }
}

impl DeployOperation<LockfileUpdated> {
    pub fn as_deploy_path(&self) -> Absolute<&Path> { self.state.previous_state.previous_state.as_deployment_path() }
    pub fn as_location(&self) -> &DeployLocation { self.state.previous_state.previous_state.as_location() }
    pub fn as_deployment(&self) -> &Deployment { &self.state.deployment }
    pub fn as_package(&self) -> &Package { self.state.previous_state.previous_state.as_package() }
}
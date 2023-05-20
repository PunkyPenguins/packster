use std::path::{PathBuf, Path};

use crate::{domain::{Checksum, Deployment}, path::Absolute, port::{FileSystem, Serializer}, Result};

use super::{Operation, AsPathLocation, AsChecksum, AlreadyDeployed, ParsedLocation, New, DeploymentPath};

pub struct UndeployRequest {
    checksum: Checksum,
    location_path: Absolute<PathBuf>
}

impl UndeployRequest {
    pub fn new(checksum: Checksum, location_path: Absolute<PathBuf>) -> Self {
        UndeployRequest { checksum, location_path }
    }
}

pub type UndeployOperation<S> = Operation<S, UndeployRequest>;

impl <S>AsChecksum for UndeployOperation<S> {
    fn as_checksum(&self) -> &Checksum { &self.request.checksum }
}

impl <S>AsPathLocation for UndeployOperation<S> {
    fn as_path_location(&self) -> Absolute<&Path> { self.request.location_path.as_absolute_path() }
}

pub type ValidState = DeploymentPath<AlreadyDeployed<ParsedLocation<New>>>;

pub struct LockfileUpdated {
    previous_state: ValidState
}

impl UndeployOperation<ValidState> {
    pub fn update_location_lockfile<F: FileSystem, Sr: Serializer>(mut self, filesystem: &F, serializer: &Sr) -> Result<UndeployOperation<LockfileUpdated>> {
        let checksum = self.as_checksum().clone();
        let location = &mut self.state.previous_state.previous_state.location;
        location.remove_deployment(&checksum);

        let deploy_location_file_content = serializer.serialize(&location)?;
        let lockfile_location = self.to_lockfile_location();
        filesystem.write_all(lockfile_location, deploy_location_file_content.as_bytes())?;

        Self::ok_with_state(self.request, LockfileUpdated { previous_state: self.state })
    }
}

pub struct DeploymentDirectoryDeleted {
    previous_state: LockfileUpdated
}

impl UndeployOperation<LockfileUpdated> {
    pub fn delete_deployment_directory<F: FileSystem>(self, filesystem: &F) -> Result<UndeployOperation<DeploymentDirectoryDeleted>> {
        filesystem.remove_dir_all(&self.state.previous_state.deployment_path)?;
        Self::ok_with_state(self.request, DeploymentDirectoryDeleted { previous_state: self.state })
    }
}

impl UndeployOperation<DeploymentDirectoryDeleted> {
    pub fn as_undeployed_deployment(&self) -> &Deployment { &self.state.previous_state.previous_state.previous_state.existing_deployment }
}
use std::path::{PathBuf, Path};

use crate::{domain::{Checksum, Deployment, DeployLocation}, path::Absolute, port::FileSystem, Result};

use super::{Operation, AsLocationPath, AsChecksum, AlreadyDeployed, ParsedLocation, New, DeploymentPath, UpdatedDeployLocation, PersistedDeployLocation};

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

impl <S>AsLocationPath for UndeployOperation<S> {
    fn as_location_path(&self) -> Absolute<&Path> { self.request.location_path.as_absolute_path() }
}

pub type UndeployValidState = DeploymentPath<AlreadyDeployed<ParsedLocation<New>>>;

impl UndeployOperation<UndeployValidState> {
    fn as_mut_location(&mut self) -> &mut DeployLocation {
        &mut self.state.previous_state.previous_state.location
    }

    pub fn remove_deployment_from_location(mut self) -> UndeployOperation<UpdatedDeployLocation<UndeployValidState>> {
        let checksum = self.as_checksum().clone();
        let location = self.as_mut_location();
        location.remove_deployment(&checksum);

        Self::with_state(self.request, UpdatedDeployLocation { previous_state: self.state })
    }
}

impl AsRef<DeployLocation> for UndeployOperation<UpdatedDeployLocation<UndeployValidState>> {
    fn as_ref(&self) -> &DeployLocation {
        &self.state.previous_state.previous_state.previous_state.location
    }
}

pub struct DeploymentDirectoryDeleted {
    previous_state: PersistedDeployLocation<UndeployValidState>
}

impl UndeployOperation<PersistedDeployLocation<UndeployValidState>> {
    pub fn delete_deployment_directory<F: FileSystem>(self, filesystem: &F) -> Result<UndeployOperation<DeploymentDirectoryDeleted>> {
        filesystem.remove_dir_all(&self.state.previous_state.deployment_path)?;
        Self::ok_with_state(self.request, DeploymentDirectoryDeleted { previous_state: self.state })
    }
}

impl UndeployOperation<DeploymentDirectoryDeleted> {
    pub fn as_undeployed_deployment(&self) -> &Deployment { &self.state.previous_state.previous_state.previous_state.existing_deployment }
}
use std::path::{PathBuf, Path};

use crate::{Result, Absolute, FileSystem, Error, domain::DeployLocation, Parser, Serializer, LOCKFILE_NAME};
use super::{Operation, New};

pub struct InitLocationRequest {
    location_directory: Absolute<PathBuf>
}

impl InitLocationRequest {
    pub fn new(location_directory: Absolute<PathBuf>) -> Self {
        InitLocationRequest { location_directory }
    }

    pub fn as_location_directory(&self) -> Absolute<&Path> {
        self.location_directory.as_absolute_path()
    }
}

pub type InitLocationOperation<S> = Operation<S, InitLocationRequest>;

pub struct ExistingDeployLocation(DeployLocation);

impl InitLocationOperation<New> {
    pub fn initialize_lockfile<F: FileSystem, S: Serializer + Parser >(self, filesystem: &F, serde: &S) -> Result<InitLocationOperation<ExistingDeployLocation>> {
        let lockfile_path = self.request.location_directory.as_ref().join(LOCKFILE_NAME);
        if filesystem.is_file(&lockfile_path) {
             Ok(Self::with_state(self.request, ExistingDeployLocation(serde.parse(filesystem.read_to_string(lockfile_path)?)?)))
        } else if filesystem.is_directory(&lockfile_path) {
            Err(Error::LocationManifestPathIsNotAFile(lockfile_path))
        } else {
            let location_path = self.request.location_directory.as_ref();
            if ! filesystem.is_directory(location_path) {
                filesystem.create_dir(location_path)?
            }

            let deploy_location = DeployLocation::default();
            filesystem.write_all(&lockfile_path, serde.serialize(&deploy_location)?.as_bytes())?;
            Ok(Self::with_state(self.request, ExistingDeployLocation(deploy_location)))
        }
    }
}

//TODO register in a user config file ?
//TODO handle set as default if the first location created ?

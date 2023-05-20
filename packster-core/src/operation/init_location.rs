use std::path::{PathBuf, Path};

use crate::{Result, Error, path::Absolute, port::{ FileSystem, Serializer }, domain::DeployLocation};
use super::{Operation, New, AsPathLocation};

pub struct InitLocationRequest {
    location_directory: Absolute<PathBuf>
}

impl InitLocationRequest {
    pub fn new(location_directory: Absolute<PathBuf>) -> Self {
        InitLocationRequest { location_directory }
    }
}

impl AsPathLocation for InitLocationRequest {
    fn as_path_location(&self) -> Absolute<&Path> {
        self.location_directory.as_absolute_path()
    }
}

pub type InitLocationOperation<S> = Operation<S, InitLocationRequest>;


pub struct LocationInitialized;

impl InitLocationOperation<New> {
    pub fn initialize_lockfile<F: FileSystem, S: Serializer>(self, filesystem: &F, serializer: &S) -> Result<InitLocationOperation<LocationInitialized>> {
        let lockfile_path = self.request.to_lockfile_location();

        ensure_that_no_lockfile_is_present(&lockfile_path, filesystem)?;

        let is_free_slot = ! filesystem.is_directory(&self.request.location_directory);
        if is_free_slot {
            filesystem.create_dir(&self.request.location_directory)?
        }

        let deploy_location = DeployLocation::default();
        let deploy_location_file_content = serializer.serialize(&deploy_location)?;
        filesystem.write_all(&lockfile_path, deploy_location_file_content.as_bytes())?;
        Self::ok_with_state(self.request, LocationInitialized)
    }
}


pub fn ensure_that_no_lockfile_is_present<F: FileSystem, P: AsRef<Path>>(path: P, filesystem: &F) -> Result<()> {
    if filesystem.exists(&path) {
         return Err(Error::AlreadyPresentLockfile(path.as_ref().to_path_buf()));
    }
    Ok(())
}

//TODO register in a user config file ?
//TODO handle set as default if the first location created ?

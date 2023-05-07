use std::path::{PathBuf, Path};

use crate::{Result, path::Absolute, port::{ FileSystem, Serializer}, domain::{DeployLocation}};
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

//TODO proper error from filesystem if : lockfile is not an existing directory, and if location directory is a file
impl InitLocationOperation<New> {
    pub fn initialize_lockfile<F: FileSystem, S: Serializer>(self, filesystem: &F, serializer: &S) -> Result<InitLocationOperation<LocationInitialized>> {
        let lockfile_path = self.request.to_lockfile_location();

        abort_if_something_already_present(&lockfile_path, filesystem)?;

        let location_path = self.request.location_directory.as_ref();
        let is_free_slot = ! filesystem.is_directory(location_path);
        if is_free_slot {
            filesystem.create_dir(location_path)?
        }

        let deploy_location = DeployLocation::default();
        let deploy_location_file_content = serializer.serialize(&deploy_location)?;
        filesystem.write_all(&lockfile_path, deploy_location_file_content.as_bytes())?;
        Self::ok_with_state(self.request, LocationInitialized)
    }
}

pub fn abort_if_something_already_present<F: FileSystem, P: AsRef<Path>>(_path: P, _filesystem: &F) -> Result<()> {
    println!("TODO check if the location is not already present");
    Ok(())
}

//TODO register in a user config file ?
//TODO handle set as default if the first location created ?

use std::path::{PathBuf, Path};

use crate::{Result, path::Absolute, port::{ FileSystem, Parser, Serializer, ReadOnlyFileSystem, Archiver }, Error, domain::{DeployLocation, Deployment},  LOCKFILE_NAME};
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


pub struct LocationInitialized;

//TODO proper error from filesystem if : lockfile is not an existing directory, and if location directory is a file
impl InitLocationOperation<New> {
    pub fn initialize_lockfile<F: FileSystem, S: Serializer>(self, filesystem: &F, serializer: &S) -> Result<InitLocationOperation<LocationInitialized>> {
        let lockfile_path = self.request.location_directory.as_ref().join(LOCKFILE_NAME);       

        abort_if_something_already_present(&lockfile_path, filesystem)?;

        let location_path = self.request.location_directory.as_ref();
        let is_free_slot = ! filesystem.is_directory(location_path);
        if is_free_slot {
            filesystem.create_dir(location_path)?
        }

        let deploy_location = DeployLocation::default();
        let deploy_location_file_content = serializer.serialize(&deploy_location)?;
        filesystem.write_all(&lockfile_path, deploy_location_file_content.as_bytes())?;
        Ok(Self::with_state(self.request, LocationInitialized))
    }
}

pub fn abort_if_something_already_present<F: FileSystem, P: AsRef<Path>>(path_buff: P, filesystem: &F) -> Result<()> {
    //TODO check if the location is not already present        
    Ok(())
}

//TODO register in a user config file ?
//TODO handle set as default if the first location created ?


// commande install(path du package, path de location)

// extraire la checksum du package

// vérifier que le package est déjà déployé dans la location sinon erreur
// extraire le package dans la location


pub fn extract_checksum<P: AsRef<Path>>(path: P) -> Result<String> { //TODO Type Checksum
    let filename = path.as_ref().file_stem();

    match filename {
        None => panic!("No file stem in path"),
        Some(filename) => {
            let filename = filename.to_str().unwrap();
            let package_id_version_checksum_packster_hash = filename.split("_").collect::<Vec<&str>>();
            let checksum = package_id_version_checksum_packster_hash.get(2).unwrap();
            Ok(checksum.to_string())
        }
    }
}

pub struct PackagedNotYetInstalled;



pub fn install_package_to_location<P: AsRef<Path>, L: AsRef<Path>, F: FileSystem, A: Archiver, S: Serializer>(path_package: P, path_location: L, filesystem: &F, archiver: &A, serializer: &S) -> Result<()> {
    // get checksum from package
    let checksum = extract_checksum(path_package.as_ref())?;
    // create deploy path from checksum to deploy package 
    let deploy_path = path_location.as_ref().join(&checksum);
    // install package in location
    archiver.extract(filesystem, path_package.as_ref(), deploy_path)?;
    // get lockfile path
    let lockfile_path = path_location.as_ref().join(LOCKFILE_NAME); 
    // append deploy location to lockfile    
    
    // todo : check if package is already installed in location
    // todo : if yes, abort
    // todo : if no, install package in location
    let mut deploy_location = DeployLocation::default();
    let deployment: Deployment = Deployment::new(checksum);
    deploy_location.add_deployment(deployment.clone());
    
    let deploy_location_file_content = serializer.serialize(&deploy_location)?;
    
    filesystem.write_all(lockfile_path, deploy_location_file_content.as_bytes())?;   
    
    // read lockfile to string
    // parse lockfile
    // mutate lockfile ( add a deployment checksum )
    // serialize and write mutated lockfile     

    // path_package : C:\\Downloads\static-package-a_0.0.1_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad_f61f10025ad.packster
    // path_location : C:\\my-location

    // deploy_path : C:\\my-location\ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad

    Ok(())   
}


#[cfg(test)]
mod test {
    use super::*;
}

// let deploy_location = DeployLocation::default();

// // Check if package is already installed in location
// if package_is_installed_in_location(&deploy_location) {
//     // If yes, abort
//     println!("Package is already installed in location. Aborting.");
//     return;
// }

// // If no, install package in location
// if install_package_in_location(&deploy_location) {
//     println!("Package installed successfully in location.");
// } else {
//     println!("Package installation failed.");
// }


// fn package_is_installed_in_location(location: &DeployLocation) -> bool {
//     // TODO: Implement logic to check if package is installed in the specified location
//     // For now, we'll assume the package is never installed and always return false
//     false
// }


// fn package_is_installed_in_location(location: &DeployLocation) -> bool {
//     // Check if any deployments exist in the location
//     for deployment in location.as_slice() {
//         // TODO: Implement logic to check if the package is installed in this deployment
//         // For now, we'll assume the package is never installed and continue iterating
//         // through the deployments.
//     }
//     // If we've iterated through all the deployments and haven't found the package, it's not installed.
//     false
// }
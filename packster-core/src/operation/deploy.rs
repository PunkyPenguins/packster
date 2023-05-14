use std::path::{Path, PathBuf};

use crate::{port::{FileSystem, Archiver, Serializer}, domain::{Package, DeployLocation, Deployment}, Result, path::Absolute};

use super::{Operation, New, AsPackagePath, AsPathLocation, AsPackage, ParsedPackage, ParsedLocation, NotYetDeployed, MatchingChecksum};

pub struct DeployRequest {
    path_package: Absolute<PathBuf>,
    path_location: Absolute<PathBuf>
}

impl DeployRequest {
    pub fn new(path_package: Absolute<PathBuf>, path_location: Absolute<PathBuf>) -> Self {
        DeployRequest { path_package, path_location }
    }
}


impl AsPackagePath for DeployRequest {
    fn as_package_path(&self) -> Absolute<&Path> {
        self.path_package.as_absolute_path()
    }
}

impl AsPathLocation for DeployRequest {
    fn as_path_location(&self) -> Absolute<&Path> {
        self.path_location.as_absolute_path()
    }
}

pub type DeployOperation<S> = Operation<S, DeployRequest>;

type ValidState = MatchingChecksum<NotYetDeployed<ParsedLocation<ParsedPackage<New>>>>;

fn into_package_and_location(valid_package: ValidState) -> (Package, DeployLocation) {
    (
        valid_package.matching_checksum.not_yet_deployed.state.package,
        valid_package.matching_checksum.not_yet_deployed.location
    )
}

pub struct ExtractedPackage {
    pub valid_package: ValidState,
    pub deploy_path: Absolute<PathBuf>
}

//An emerging good practise here : keep read operations as generic as possible and write operation as specific as possible
impl DeployOperation<ValidState> {
    pub fn extract_package<F: FileSystem, A: Archiver>(self, filesystem: &F, archiver: &A) -> Result<DeployOperation<ExtractedPackage>> {
        let checksum = &self.state.as_package().as_checksum().to_string();
        let deploy_path = self.request.path_location.join(checksum);
        archiver.extract(
            filesystem,
            self.request.path_package.as_absolute_path(),
            deploy_path.as_absolute_path()
        )?;
        Self::ok_with_state(self.request, ExtractedPackage { valid_package: self.state, deploy_path })
    }
}

pub struct DeployedPackage {
    pub package: Package,
    pub deploy_path: Absolute<PathBuf>
}

impl DeployOperation<ExtractedPackage> {
    pub fn update_location_lockfile<F: FileSystem, Sr: Serializer>(self, filesystem: &F, serializer: &Sr) -> Result<DeployOperation<DeployedPackage>> {
        let (package, mut location) = into_package_and_location(self.state.valid_package);
        let deployment: Deployment = Deployment::new(package.as_checksum().clone());
        location.add_deployment(deployment);
        let deploy_location_file_content = serializer.serialize(&location)?;
        filesystem.write_all(self.request.to_lockfile_location(), deploy_location_file_content.as_bytes())?;
        Self::ok_with_state(self.request, DeployedPackage { package, deploy_path: self.state.deploy_path })
    }
}
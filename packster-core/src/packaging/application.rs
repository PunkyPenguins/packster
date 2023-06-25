#![allow(dead_code)]
use std::path::{Path, PathBuf};

mod generic;
pub use generic::*;

mod pack;
pub use pack::*;

mod init_location;
pub use init_location::*;

mod deploy;
pub use deploy::*;

mod undeploy;
pub use undeploy::*;

mod show_location;
pub use show_location::*;

use crate::application::{path::Absolute, operation::Operation};

use super::{domain::{Package, DeployLocation}, LOCKFILE_NAME};


pub trait AsPackagePath {
    fn as_package_path(&self) -> Absolute<&Path>;
}

pub trait AsPackage {
    fn as_package(&self) -> &Package;
}

// Forward to all operations containing state that implement this trait
impl <S: AsPackage, R>AsPackage for Operation<S, R> {
    fn as_package(&self) -> &Package { self.as_state().as_package() }
}

pub trait AsLocationPath {
    fn as_location_path(&self) -> Absolute<&Path>;
    fn to_location_lockfile_path(&self) -> Absolute<PathBuf> {
        self.as_location_path().join(LOCKFILE_NAME)
    }
}

pub trait AsLocation {
    fn as_location(&self) -> &DeployLocation;
}

impl <S: AsLocation, R>AsLocation for Operation<S, R> {
    fn as_location(&self) -> &DeployLocation { self.as_state().as_location() }
}
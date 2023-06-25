#![allow(dead_code)]
use std::path::{Path, PathBuf};

mod generic_domain;
pub use generic_domain::*;

mod generic_application;
pub use generic_application::*;

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

use crate::{path::Absolute, LOCKFILE_NAME, entity::{DeployLocation, Package, Checksum}, Result};

pub struct Operation<S, R>{
    request: R,
    state: S
}

pub struct New;

impl <R> Operation<New, R> {
    pub fn new(request: R) -> Self { Operation { request, state: New } }
}

//TODO doing rollback operation using same states newtype struct would be super easy and meaningful and stylish cause each state would be described as reversible, or not, or partially, or with different methods :O
impl <S, R>Operation<S, R> {
    /*Create a new operation */
    pub fn from(request: R, state: S) -> Self { Operation { request, state } }

    pub fn get_state(&self) -> &S { &self.state }
    pub fn get_request(&self) -> &R { &self.request }

    /*Return same operation ( same request ) with a new state */
    pub (crate) fn with_state<N>(request: R, state: N) -> Operation<N, R> {
        Operation { state, request }
    }

    /*Return a new operation ( new request ) but the same state */
    pub (crate) fn with_request<N>(request: N, state: S) -> Operation<S, N> {
        Operation { request, state }
    }

    pub fn into_state<N: From<S>>(self) -> Operation<N, R> {
        Self::with_state(self.request, self.state.into())
    }

    pub fn into_request<N: From<R>>(self) -> Operation<S, N> {
        Self::with_request(self.request.into(), self.state)
    }

    pub (crate) fn ok_with_state<N>(request: R, state: N) -> Result<Operation<N, R>> {
        Ok(Self::with_state(request, state))
    }
}

pub trait AsPackagePath {
    fn as_package_path(&self) -> Absolute<&Path>;
}

pub trait AsPackage {
    fn as_package(&self) -> &Package;
}

// Forward to all operations containing state that implement this trait
impl <S: AsPackage, R>AsPackage for Operation<S, R> {
    fn as_package(&self) -> &Package { self.state.as_package() }
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
    fn as_location(&self) -> &DeployLocation { self.state.as_location() }
}


pub trait AsChecksum {
    fn as_checksum(&self) -> &Checksum;
}
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

use crate::{path::Absolute, LOCKFILE_NAME, domain::{DeployLocation, Package, Checksum}, Result};

pub struct Operation<S, R>{
    request: R,
    state: S
}

pub struct New;

impl <R> Operation<New, R> {
    pub fn new(request: R) -> Self { Operation { request, state: New } }
}

//TODO doing rollback operation using same states newtype struct would be super easy and meaningful and stylish cause each state would be described as reversible, or not, or partially, or with different methods :O
//TODO R&D : consider replace Operation by a State Trait to simplify architecture and state machine building flexibility
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

pub trait AsPathLocation {
    fn as_path_location(&self) -> Absolute<&Path>;
    fn to_lockfile_location(&self) -> Absolute<PathBuf> {
        self.as_path_location().join(LOCKFILE_NAME)
    }
}

pub trait AsLocation {
    fn as_location(&self) -> &DeployLocation;
}

pub trait AsChecksum {
    fn as_checksum(&self) -> &Checksum;
}

impl <S: AsPackage>AsPackage for MatchingChecksum<S> {
    fn as_package(&self) -> &Package {
        self.previous_state.as_package()
    }
}
#![allow(dead_code)]
mod pack;
pub use pack::*;

mod init_location;
pub use init_location::*;

pub struct Operation<S, R>{
    request: R,
    state: S
}

pub struct New;

//TODO doing rollback operation using same states newtype struct would be super easy and meaningful and stylish cause each state would be described as reversible, or not, or partially, or with different methods :O
impl <S, R>Operation<S, R> {
    /*Create a new operation */
    pub fn new(request: R, state: S) -> Self {
        Operation { request, state }
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn get_request(&self) -> &R {
        &self.request
    }

    /*Return same operation ( same request ) with a new state */
    pub (crate) fn with_state<N>(request: R, state: N) -> Operation<N, R> {
        Operation { state, request }
    }

    /*Return a new operation ( new request ) but the same state */
    pub (crate) fn with_request<N>(request: N, state: S) -> Operation<S, N> {
        Operation { request, state }
    }
}


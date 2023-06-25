use crate::{Result, domain::entity::Checksum};

pub struct Operation<S, R>{
    pub request: R,
    pub state: S
}

pub struct New;

impl <R> Operation<New, R> {
    pub fn new(request: R) -> Self { Operation { request, state: New } }
}

//TODO doing rollback operation using same states newtype struct would be super easy and meaningful and stylish cause each state would be described as reversible, or not, or partially, or with different methods :O
impl <S, R>Operation<S, R> {
    pub fn as_state(&self) -> &S { &self.state }
    pub fn as_mut_state(&mut self) -> &mut S { &mut self.state }
    pub fn as_request(&self) -> &R { &self.request }

    /*Return same operation ( same request ) with a new state */
    pub (crate) fn with_state<N>(request: R, state: N) -> Operation<N, R> {
        Operation { state, request }
    }

    // /*Return a new operation ( new request ) but the same state */
    // pub (crate) fn with_request<N>(request: N, state: S) -> Operation<S, N> {
    //     Operation { request, state }
    // }

    // pub fn into_state<N: From<S>>(self) -> Operation<N, R> {
    //     Self::with_state(self.get_request(), self.get_state().into())
    // }

    // pub fn into_request<N: From<R>>(self) -> Operation<S, N> {
    //     Self::with_request(self.get_request().into(), self.get_state())
    // }

    pub (crate) fn ok_with_state<N>(request: R, state: N) -> Result<Operation<N, R>> {
        Ok(Self::with_state(request, state))
    }
}

impl <S, R>From<(S, R)> for Operation<S, R> {
    fn from(value: (S, R)) -> Self {
        let (state, request) = value;
        Operation { state, request }
    }
}

pub trait AsChecksum {
    fn as_checksum(&self) -> &Checksum;
}
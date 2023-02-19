mod pack;
pub use pack::*;

pub struct Operation<S, R>{
    request: R,
    state: S
}

pub struct New;


impl <S, R>Operation<S, R> {
    pub fn new(request: R, state: S) -> Self {
        Operation { request, state }
    }

    pub (crate) fn with_state<N>(request: R, state: N) -> Operation<N, R> {
        Operation { state, request }
    }
}


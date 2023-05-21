use std::path::{PathBuf, Path};

use crate::path::Absolute;

use super::{AsPathLocation, Operation};

pub struct ShowLocationRequest {
    location_directory: Absolute<PathBuf>
}

impl ShowLocationRequest {
    pub fn new(location_directory: Absolute<PathBuf>) -> Self {
        ShowLocationRequest { location_directory }
    }
}

impl <S>AsPathLocation for Operation<S, ShowLocationRequest> {
    fn as_path_location(&self) -> Absolute<&Path> { self.request.location_directory.as_absolute_path() }
}
use std::path::{PathBuf, Path};
use crate::application::path::Absolute;
use super::{AsLocationPath, Operation};

pub struct ShowLocationRequest {
    location_directory: Absolute<PathBuf>
}

impl ShowLocationRequest {
    pub fn new(location_directory: Absolute<PathBuf>) -> Self {
        ShowLocationRequest { location_directory }
    }
}

impl <S>AsLocationPath for Operation<S, ShowLocationRequest> {
    fn as_location_path(&self) -> Absolute<&Path> { self.as_request().location_directory.as_absolute_path() }
}
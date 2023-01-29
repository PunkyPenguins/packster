use std::path::{Path, PathBuf};
use crate::{Result, port::Archiver, essential::port::FileSystem};

pub struct DirectoryArchiver {
    extension: String
}

impl Default for DirectoryArchiver {
    fn default() -> Self {
        DirectoryArchiver {
            extension: String::from("packster")
        }
    }
}

impl Archiver for DirectoryArchiver {
    fn create_archive<F: FileSystem, P: AsRef<Path>>(&self, filesystem: &mut F, portage_path: P, destination_path: P) -> Result<PathBuf> {
        todo!()
    }
}
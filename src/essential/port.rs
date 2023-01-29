use std::path::Path;
use crate::Result;

pub trait FileSystem {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn write_all<P: AsRef<Path>>(&mut self, path: P, buf: &[u8]) -> Result<()>;
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool;
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String>;
}

pub trait Packager {}
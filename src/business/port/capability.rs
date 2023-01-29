use std::{path::{Path, PathBuf}, marker::PhantomData};
use crate::Result;

pub trait TmpDir {
    fn path(&self) -> &Path;
    fn close(self) -> Result<()>;
}

pub trait ReadOnlyFileSystem : Sync + Send {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool;
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String>;
}

pub trait FileSystem : ReadOnlyFileSystem {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, buf: B) -> Result<()>;
    fn rename<P: AsRef<Path>>(&mut self, source: P, destination: P) -> Result<String>;
    fn create_tmp_dir<S: AsRef<str>>(&mut self, prefix: S) -> Result<Box<dyn TmpDir>>;
}

pub trait Archiver : Sync + Send {
    fn create_archive<F: FileSystem, P: AsRef<Path>>(&self, filesystem: &mut F, portage_path: P, destination_path: P) -> Result<PathBuf>;
}

pub trait Digester : Sync + Send {
    fn generate_checksum<F: ReadOnlyFileSystem, P: AsRef<Path>>(&self, filesystem: &F, file_path: P) -> Result<String>;
}
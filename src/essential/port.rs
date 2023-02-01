use std::{path::{Path, PathBuf}, io::{Read, Write}, fmt::Display};
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
    fn open_read<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Read>>;
}

pub trait FileSystem : ReadOnlyFileSystem {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;
    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, buf: B) -> Result<()>;
    fn rename<P: AsRef<Path>>(&mut self, source: P, destination: P) -> Result<()>;
    fn create_tmp_dir<S: AsRef<str>>(&mut self, prefix: S) -> Result<Box<dyn TmpDir>>;
    fn append<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, buf: B) -> Result<usize>;
    fn open_write<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<Box<dyn Write + 'a>>;
}

pub trait Archiver : Sync + Send {
    fn archive<P: AsRef<Path>, W: Write>(&self, path: P, write: W) -> Result<()>;
    fn compress<R: Read, W: Write>(&self, reader: R, write: W) -> Result<()>;
    //TODO expand archive => We want opposite features to be bound in the same trait
}

pub trait Digester : Sync + Send + Display {
    fn generate_checksum<R: Read>(&self, reader: R) -> Result<Vec<u8>>;
    //TODO verify checksup => We want opposite features to be bound in the same trait
}

pub trait PortageManifest : Sync + Send {
    fn as_identifier(&self) -> &str;
    fn as_version(&self) -> &str;
}

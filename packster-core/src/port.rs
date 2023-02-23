use std::{path::Path, io::{Read, Write}, fmt::Display};
use serde::de::DeserializeOwned;

use crate::{Result, path::NormalizedPath};

#[derive(Debug)]
pub struct DirEntry {
    path: NormalizedPath,
    size: u64
}

impl DirEntry {
    pub fn new(path: &Path, size: u64) -> Self { DirEntry { path: NormalizedPath::from(path), size } }
    pub fn as_path(&self) -> &Path { &self.path }
    pub fn as_normalized_path(&self) -> &NormalizedPath { &self.path }
    pub fn size(&self) -> u64 { self.size }
}

pub trait ReadOnlyFileSystem : Sync + Send {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool;
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String>;
    fn open_read<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Read + Send + Sync>>;
    fn walk<'a>(&'a self, target_path: &'a Path) -> Box<dyn Iterator<Item = Result<DirEntry>> + 'a>;
    fn file_size<P: AsRef<Path>>(&self, path: P) -> Result<u64>;
}

pub trait FileSystem : ReadOnlyFileSystem {
    fn create<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()>;
    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<()>;
    fn rename<P: AsRef<Path>>(&self, source: P, destination: P) -> Result<()>;
    fn append<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<usize>;
    fn open_write<'a, P: AsRef<Path>>(&'a self, path: P) -> Result<Box<dyn Write + Send + Sync + 'a>>;
}

pub trait Archiver : Sync + Send + Display {
    fn archive<F: FileSystem, P: AsRef<Path>>(&self, filesystem: &F, project_path: P, archive_path: P) -> Result<()>;
    // fn unarchive<F: FileSystem, P: AsRef<Path>>(&self, filesystem: &F, expand_path: P, archive_path: P) -> Result<()>;
}

pub trait Digester : Sync + Send + Display {
    fn generate_checksum<R: Read>(&self, reader: R) -> Result<Vec<u8>>;
    // fn verify_checksum<R: Read>(&self, reader: R, checksum: &[u8]) -> bool;
}

pub trait IdentifierGenerator : Sync + Send {
    fn generate_identifier<S: AsRef<str>>(&self, name: S) -> String;
}

pub trait Parser {
    fn parse<S: AsRef<str>, T: DeserializeOwned>(&self, s: S) -> Result<T>;
}
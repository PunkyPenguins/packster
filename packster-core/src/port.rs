use std::{path::Path, io::{Read, Write}};
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::{Result, path::{Absolute, NormalizedPathBuf}};

pub trait PathExt {
    fn is_ancestor_of<P: AsRef<Path>>(&self, child_path: P) -> bool;
    fn to_normalized_path(&self) -> NormalizedPathBuf;
}

#[derive(Debug)]
pub struct DirEntry {
    path: Absolute<NormalizedPathBuf>,
    size: u64
}

impl DirEntry {
    pub fn new(path: Absolute<NormalizedPathBuf>, size: u64) -> Self { DirEntry { path, size } }
    pub fn as_path(&self) -> &Path { self.path.as_ref() }
    pub fn as_absolute_path(&self) -> Absolute<&Path> { self.path.as_absolute_path() }
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

pub trait Archiver : Sync + Send {
    fn archive<F: FileSystem, P1: AsRef<Path>, P2: AsRef<Path>>(&self, filesystem: &F, project_path: Absolute<P1>, archive_path: Absolute<P2>) -> Result<()>;
    // fn unarchive<F: FileSystem, P: AsRef<Path>>(&self, filesystem: &F, expand_path: P, archive_path: P) -> Result<()>;
}

pub trait Digester : Sync + Send {
    fn generate_checksum<R: Read>(&self, reader: R) -> Result<Vec<u8>>;
    // fn verify_checksum<R: Read>(&self, reader: R, checksum: &[u8]) -> bool;
}

pub trait UniqueIdentifierGenerator : Sync + Send {
    fn generate_identifier(&self) -> String;
}

pub trait Parser {
    fn parse<S: AsRef<str>, T: DeserializeOwned>(&self, s: S) -> Result<T>;
}

pub trait Serializer {
    fn serialize<T: Serialize>(&self, value: &T) -> Result<String>;
}
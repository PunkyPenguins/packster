use std::{path::{Path, PathBuf}, io::{Read, Write}, fs::{self, File}};

use walkdir::WalkDir;

use crate::{ Result, essential::port::{IReadOnlyFileSystem, IFileSystem, DirEntry} };

pub struct StdFileSystem;

impl IReadOnlyFileSystem for StdFileSystem {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_file()
    }

    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_dir()
    }

    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }

    fn open_read<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Read>> {
        Ok(Box::new(File::open(path.as_ref())?))
    }

    fn walk<'a>(&'a self, target_path: &'a Path) -> Box<dyn Iterator<Item = Result<DirEntry>> + 'a> {
        Box::new(
            WalkDir::new(target_path)
                .into_iter()
                .map(|entry|
                    entry.and_then(|entry|
                        entry.metadata().map(|metadata|
                            DirEntry::new(
                                entry.into_path().to_path_buf(),
                                metadata.len()
                            )
                        )
                    ).map_err(|e| e.into())
                )
        )
    }

    fn file_size<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        Ok(path.as_ref().metadata()?.len())
    }
}

impl IFileSystem for StdFileSystem {
    fn create<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        todo!()
    }

    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        todo!()
    }

    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<()> {
        todo!()
    }

    fn rename<P: AsRef<Path>>(&self, source: P, destination: P) -> Result<()> {
        todo!()
    }

    fn append<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<usize> {
        todo!()
    }

    fn open_write<'a, P: AsRef<Path>>(&'a self, path: P) -> Result<Box<dyn Write + 'a>> {
        todo!()
    }
}
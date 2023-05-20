use std::{
    path::Path,
    io::{Read, Write},
    fs::{self, File}
};
use walkdir::WalkDir;

use packster_core::{port::{ReadOnlyFileSystem, FileSystem, DirEntry, PathExt }, path::Absolute};
use crate::{Result, Error};

pub struct StdFileSystem;

//TODO add some logging and integration tests
impl ReadOnlyFileSystem for StdFileSystem {
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
        Ok(fs::read_to_string(path).map_err(Error::from)?)
    }

    fn open_read<P: AsRef<Path>>(&self, path: P) -> packster_core::Result<Box<dyn Read + Send + Sync>> {
        Ok(Box::new(File::open(path).map_err(Error::from)?))
    }

    fn walk<'a>(&'a self, target_path: &'a Path) -> Box<dyn Iterator<Item = Result<DirEntry>> + 'a> {
        Box::new(
            WalkDir::new(target_path)
                .into_iter()
                .map(|entry|
                    entry
                    .and_then(|entry|
                        entry.metadata()
                            .map(|metadata|
                                (metadata.len(), entry.path().to_normalized_path())
                            )
                    )
                    .map_err(|e| Error::from(e).into())
                    .and_then(|(len, normalized_path)|
                        Absolute::try_absolute(normalized_path)
                            .map(|absolute_path|
                                 DirEntry::new(
                                    absolute_path,
                                    len
                                )
                            )
                    )
                )
        )
    }

    fn file_size<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        Ok(path.as_ref().metadata().map_err(Error::from)?.len())
    }
}

impl FileSystem for StdFileSystem {
    fn create<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        File::create(path).map_err(Error::from)?;
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(path).map_err(Error::from).map_err(Error::from)?;
        Ok(())
    }

    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<()> {
        fs::write(path, buf).map_err(Error::from)?;
        Ok(())
    }

    fn rename<P: AsRef<Path>>(&self, source: P, destination: P) -> Result<()> {
        fs::rename(source, destination).map_err(Error::from)?;
        Ok(())
    }

    fn append<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<usize> {
        Ok(File::open(path).map_err(Error::from)?.write(buf.as_ref()).map_err(Error::from)?)
    }

    fn open_write<'a, P: AsRef<Path>>(&'a self, path: P) -> packster_core::Result<Box<dyn Write + Send + Sync + 'a>> {
        Ok(Box::new(File::create(path).map_err(Error::from)?))
    }

    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> packster_core::Result<()> {
        Ok(fs::remove_dir_all(path).map_err(Error::from)?)
    }
}
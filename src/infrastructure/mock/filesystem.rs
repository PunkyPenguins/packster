use std::path::Path;
use crate::{ Result, port::{FileSystem, ReadOnlyFileSystem, TmpDir} };

pub struct FileSystemMock;

impl Default for FileSystemMock {
    fn default() -> Self {
        todo!()
    }
}

impl ReadOnlyFileSystem for FileSystemMock {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool { todo!() }
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool { todo!() }
    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool { todo!() }
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> { todo!() }
}

impl FileSystem for FileSystemMock {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<()> { todo!() }
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> { todo!() }
    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, buf: B) -> Result<()> { todo!() }
    fn rename<P: AsRef<Path>>(&mut self, source: P, destination: P) -> Result<String> { todo!() }
    fn create_tmp_dir<S: AsRef<str>>(&mut self, prefix: S) -> Result<Box<dyn TmpDir>> { todo!() }
}

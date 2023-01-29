use std::path::Path;
use crate::{ Result, port::FileSystem };

pub struct FileSystemMock;

impl Default for FileSystemMock {
    fn default() -> Self {
        todo!()
    }
}

impl FileSystem for FileSystemMock {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<()> { todo!() }
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> { todo!() }
    fn write_all<P: AsRef<Path>>(&mut self, path: P, buf: &[u8]) -> Result<()> { todo!() }
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool { todo!() }
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool { todo!() }
    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool { todo!() }
    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> { todo!() }
}

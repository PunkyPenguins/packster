use std::{
    collections::BTreeMap,
    sync::RwLock,
    path::{Path, PathBuf},
    io::{ self, Cursor, Read, Write }
};
use crate::{ Result, path::NormalizedPath, port::{FileSystem, ReadOnlyFileSystem, TmpDir} };


#[derive(Clone, Debug)]
pub enum Node {
    File(Vec<u8>),
    Directory
}

#[derive(Default)]
pub struct InMemoryFileSystem(RwLock<BTreeMap<NormalizedPath, Node>>);

impl ReadOnlyFileSystem for InMemoryFileSystem {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.read().unwrap().get(&NormalizedPath::from(path.as_ref())).is_some()
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.read().unwrap().get(&NormalizedPath::from(path.as_ref())).filter(|node| matches!(node, Node::File(_))).is_some()
    }

    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.read().unwrap().get(&NormalizedPath::from(path.as_ref())).filter(|node| matches!(node, Node::Directory)).is_some()
    }

    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        if ! self.exists(path.as_ref()) { panic!("Path not found ! {:?}", path.as_ref()); }
        if ! self.is_file(path.as_ref()) { panic!("Path is not a file ! {:?}", path.as_ref()); }

        self.0.read().unwrap()
            .get(&NormalizedPath::from(path.as_ref()))
            .map(|node| match node {
                Node::File(content) => Ok(String::from_utf8(content.to_vec()).unwrap()),
                _ => { panic!("Path is not a file {:?}", path.as_ref()); }
            }).unwrap()
    }

    fn open_read<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Read>> {
        if ! self.exists(path.as_ref()) { panic!("Path not found ! {:?}", path.as_ref()); }
        if ! self.is_file(path.as_ref()) { panic!("Path is not a file ! {:?}", path.as_ref()); }

        self.0.read().unwrap()
            .get(&NormalizedPath::from(path.as_ref()))
            .map(|node| match node {
                Node::File(content) => Ok(Box::new(Cursor::new(content.to_vec())) as Box<dyn Read>),
                _ => { panic!("Path is not a file {:?}", path.as_ref()); }
            }).unwrap()
    }
}

impl FileSystem for InMemoryFileSystem {
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.0.write().unwrap().insert(NormalizedPath::from(path.as_ref()), Node::File(Vec::new()));
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        if self.exists(path.as_ref()) { panic!("Path already exists ! {:?}", path.as_ref()); }
        if ! self.is_directory(path.as_ref()) { panic!("Path is not a directory ! {:?}", path.as_ref()); }

        self.0.write().unwrap().insert(NormalizedPath::from(path.as_ref()), Node::Directory);
        Ok(())
    }

    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, buf: B) -> Result<()> {
        self.0.write().unwrap().insert(NormalizedPath::from(path.as_ref()), Node::File(buf.as_ref().to_vec()));
        Ok(())
    }

    fn rename<P: AsRef<Path>>(&mut self, source: P, destination: P) -> Result<()> {
        if ! self.exists(source.as_ref()) { panic!("Path not found {:?}", source.as_ref()); }
        let destination_path = NormalizedPath::from(source.as_ref());
        let node = { self.0.write().unwrap().remove(&destination_path).unwrap() };

        self.0.write().unwrap().insert(destination_path, node).unwrap();
        Ok(())
    }

    fn create_tmp_dir<S: AsRef<str>>(&mut self, prefix: S) -> Result<Box<dyn TmpDir>> {
        todo!()
    }

    fn append<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, buf: B) -> Result<usize> {
        let len = buf.as_ref().len();
        self.0.write()
            .unwrap()
            .entry(NormalizedPath::from(path.as_ref()))
            .and_modify(|node|
                if let Node::File(content) = node {
                    content.extend(buf.as_ref())
                }  else {
                    panic!("Path is not a file {:?}", path.as_ref());
                }
            )
            .or_insert( Node::File(buf.as_ref().to_vec()));

        Ok(len)
    }

    fn open_write<'a, P: AsRef<Path>>(&'a mut self, path: P) -> Result<Box<dyn Write + 'a>> {
        if ! self.exists(path.as_ref()) { panic!("Path not found ! {:?}", path.as_ref()); }
        if ! self.is_file(path.as_ref()) { panic!("Path is not a file ! {:?}", path.as_ref()); }

        Ok(
            Box::new(
                FileMock {
                    fs: self,
                    path: path.as_ref().to_path_buf()
                }
            )
        )
    }
}

pub struct FileMock<'a>{
    fs: &'a mut InMemoryFileSystem,
    path: PathBuf
}

impl <'a>Write for FileMock<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Result::Ok(self.fs.append(&self.path, buf).unwrap())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::Result::Ok(())
    }
}
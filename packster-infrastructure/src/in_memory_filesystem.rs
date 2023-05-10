use std::{
    collections::BTreeMap,
    sync::RwLock,
    path::{Path, PathBuf},
    io::{self, Cursor, Read, Write}
};

use packster_core::{
    port::{
        PathExt,
        FileSystem,
        ReadOnlyFileSystem,
        DirEntry,
        Archiver
    },
    path::{ NormalizedPathBuf, Absolute }
};
use crate::{Result, Error};


#[derive(Clone, Debug)]
pub enum Node {
    File(Vec<u8>),
    Directory
}

/**
 * InMemoryFileSystem is not prod ready for the following reasons :
 * - it doesn't handle errors properly ( a lot of unwrap )
 * - it assumes paths are absolute
 * - it propably doesn't have all the wanted behaviours for a virtual in-memory file system
 */
#[derive(Default, Debug)]
pub struct InMemoryFileSystem(RwLock<BTreeMap<NormalizedPathBuf, Node>>);

impl ReadOnlyFileSystem for InMemoryFileSystem {
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if path == Path::new("") || path == Path::new("//") || path == Path::new("C:") {
            true
        } else {
            self.0.read().unwrap().get(&NormalizedPathBuf::from(path)).is_some()
        }
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.read().unwrap().get(&NormalizedPathBuf::from(path.as_ref())).filter(|node| matches!(node, Node::File(_))).is_some()
    }

    fn is_directory<P: AsRef<Path>>(&self, path: P) -> bool {
        let path = path.as_ref();
        if path == Path::new("") || path == Path::new("//") || path == Path::new("C:") {
            true
        } else {
            self.0.read().unwrap().get(&NormalizedPathBuf::from(path)).filter(|node| matches!(node, Node::Directory)).is_some()
        }
    }

    fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        if ! self.exists(path.as_ref()) { panic!("read_to_string: Path not found ! {:?}", path.as_ref()); }
        if ! self.is_file(path.as_ref()) { panic!("read_to_string! Path is not a file ! {:?}", path.as_ref()); }

        self.0.read().unwrap()
            .get(&NormalizedPathBuf::from(path.as_ref()))
            .map(|node| match node {
                Node::File(content) => Ok(String::from_utf8(content.to_vec()).unwrap()),
                _ => { panic!("Path is not a file {:?}", path.as_ref()); }
            }).unwrap()
    }

    fn open_read<P: AsRef<Path>>(&self, path: P) -> packster_core::Result<Box<dyn Read + Send + Sync>> {
        if ! self.exists(path.as_ref()) { panic!("open_read: Path not found ! {:?}", path.as_ref()); }
        if ! self.is_file(path.as_ref()) { panic!("open_read: Path is not a file ! {:?}", path.as_ref()); }

        self.0.read().unwrap()
            .get(&NormalizedPathBuf::from(path.as_ref()))
            .map(|node| match node {
                Node::File(content) => Ok(Box::new(Cursor::new(content.to_vec())) as Box<dyn Read + Send + Sync>),
                _ => { panic!("Path is not a file {:?}", path.as_ref()); }
            }).unwrap()
    }

    fn walk<'a>(&'a self, target_path: &'a Path) -> Box<dyn Iterator<Item = Result<DirEntry>> + 'a> {
        let normalized_target_path = NormalizedPathBuf::from(target_path);
        let buf : Vec<_> = self.0.read()
            .unwrap()
            .iter()
            .filter(move |(node_path, _)|
                normalized_target_path.as_ref().is_ancestor_of(*node_path)
            ).map(|(node_path, _)|
                self.file_size(node_path)
                    .map(|size|
                        DirEntry::new(Absolute::assume_absolute(node_path.clone()), size)
                    )
            ).collect();

        Box::new(buf.into_iter())
    }

    fn file_size<P: AsRef<Path>>(&self, path: P) -> Result<u64> {
        let mut buffer = Vec::new();
        if self.is_directory(&path) {
            Ok(0)
        } else {
            self.open_read(&path)?.read_to_end(&mut buffer).map_err(Error::from)?;
            Ok(buffer.len() as u64)
        }
    }
}

impl FileSystem for InMemoryFileSystem {
    fn create<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if self.exists(path.as_ref()) { panic!("create: Path already exists ! {:?}", path.as_ref()); }
        if let Some(parent_path) = path.as_ref().parent() {
            if ! self.is_directory(parent_path) { panic!("create: Parent is not a directory ! {parent_path:?}"); }
        }

        self.0.write().unwrap().insert(NormalizedPathBuf::from(path.as_ref()), Node::File(Vec::new()));
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if self.exists(path.as_ref()) { panic!("create_dir: Path already exists ! {:?}", path.as_ref()); }
        if let Some(parent_path) = path.as_ref().parent() {
            if ! self.is_directory(parent_path) { panic!("create_dir: Parent is not a directory ! {parent_path:?}"); }
        }

        self.0.write().unwrap().insert(NormalizedPathBuf::from(path.as_ref()), Node::Directory);
        Ok(())
    }

    fn write_all<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<()> {
        if let Some(parent_path) = path.as_ref().parent() {
            if ! self.is_directory(parent_path) { panic!("write_all: Parent is not a directory ! {parent_path:?}"); }
        }

        self.0.write().unwrap().insert(NormalizedPathBuf::from(path.as_ref()), Node::File(buf.as_ref().to_vec()));
        Ok(())
    }

    fn rename<P: AsRef<Path>>(&self, source: P, destination: P) -> Result<()> {
        if ! self.exists(source.as_ref()) { panic!("rename: Path not found {:?}", source.as_ref()); }
        let source_path = NormalizedPathBuf::from(source.as_ref());
        let destination_path = NormalizedPathBuf::from(destination.as_ref());
        let node = { self.0.write().unwrap().remove(&source_path).unwrap() };

        self.0.write().unwrap().insert(destination_path, node);
        Ok(())
    }

    fn append<P: AsRef<Path>, B: AsRef<[u8]>>(&self, path: P, buf: B) -> Result<usize> {
        if let Some(parent_path) = path.as_ref().parent() {
            if ! self.is_directory(parent_path) { panic!("append: Parent is not a directory ! {parent_path:?}"); }
        }

        let len = buf.as_ref().len();
        self.0.write()
            .unwrap()
            .entry(NormalizedPathBuf::from(path.as_ref()))
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

    fn open_write<'a, P: AsRef<Path>>(&'a self, path: P) -> packster_core::Result<Box<dyn Write + Send + Sync + 'a>> {
        if self.is_directory(path.as_ref()) { panic!("open_write: Path is not a file ! {:?}", path.as_ref()); }

        Ok(
            Box::new(
                InMemoryFile {
                    fs: self,
                    path: path.as_ref().to_path_buf()
                }
            )
        )
    }
}

pub struct InMemoryFile<'a>{
    fs: &'a InMemoryFileSystem,
    path: PathBuf
}

impl <'a>Write for InMemoryFile<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Result::Ok(self.fs.append(&self.path, buf).unwrap())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::Result::Ok(())
    }
}

impl Archiver for InMemoryFileSystem {
    fn archive<F: FileSystem, P1: AsRef<Path>, P2: AsRef<Path>>(&self, filesystem: &F, project_path: Absolute<P1>, archive_path: Absolute<P2>) -> Result<()> {
        filesystem.create(archive_path.as_ref())?;

        for found_entry_result in filesystem.walk(project_path.as_ref()) {
            let found_entry = found_entry_result?;
            let absolute_path = found_entry.as_absolute_path();
            let relative_path = absolute_path.try_to_relative(&project_path)?;

            if filesystem.is_file(found_entry.as_path()) {
                let mut reader = filesystem.open_read(found_entry.as_path())?;
                let mut writer = self.open_write(&relative_path)?;
                io::copy(&mut reader, &mut writer).map_err(Error::from)?;
            } else if filesystem.is_directory(found_entry.as_path()) {
                self.create_dir(&relative_path)?;
            }
        }

        Ok(())
    }

    fn extract<F: FileSystem, P1: AsRef<Path>, P2: AsRef<Path>>(&self, filesystem: &F, expand_path: Absolute<P1>, archive_path: Absolute<P2>) -> Result<()> {
        unimplemented!()
    }
}

pub struct InMemoryDirEntry<'a> {
    fs: &'a InMemoryFileSystem,
    path: PathBuf
}
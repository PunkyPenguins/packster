use std::{path::{PathBuf, Path, Component}, ops::Deref};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct NormalizedPath(PathBuf);

impl From<&Path> for NormalizedPath {
    fn from(path: &Path) -> Self {
        NormalizedPath(normalize_path(path))
    }
}

impl Deref for NormalizedPath {
    type Target = Path;

    fn deref(&self) -> &Path {
        self.0.deref()
    }
}

pub fn normalize_path<P: AsRef<Path>>(path_ref: P) -> PathBuf {
    let path = path_ref.as_ref();

    let mut buffer = PathBuf::new();
    let mut level = 0;
    for component in path.components() {
        if matches!(component, Component::Normal(_)) { level += 1; }
        match component {
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => buffer.push(component),
            Component::CurDir => {},
            Component::ParentDir => if level > 0 {
                buffer.pop();
                level -= 1;
            } else {
                buffer.push(component)
            },
        };
    }

    buffer
}

#[test]
pub fn test_normalize_path_handle_different_separators(){
    let path = PathBuf::from("C:\\this/is\\a/test/of/inconstistant\\separators");
    assert_eq!(PathBuf::from("C:\\this\\is\\a\\test\\of\\inconstistant\\separators"), normalize_path(path))
}
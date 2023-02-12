use std::{path::{PathBuf, Path, Component}, ops::Deref};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NormalizedPath(PathBuf);


impl NormalizedPath {
    pub fn is_ancestor_of<P: AsRef<Path>>(&self, child_path: P) -> bool {
        child_path.as_ref()
            .ancestors()
            .map(|ancestor| NormalizedPath(ancestor.to_path_buf()))
            .any(|ancestor| { ancestor == *self })
    }

    pub fn to_relative_path<P: AsRef<Path>>(&self, base: P) -> Self {
        NormalizedPath(
            self.0.strip_prefix(base.as_ref())
                .unwrap_or_else(|_| &self.0)
                .to_path_buf()
        )
    }
}

impl From<&Path> for NormalizedPath {
    fn from(path: &Path) -> Self {
        NormalizedPath(normalize_path(path))
    }
}


impl AsRef<Path> for NormalizedPath {
    fn as_ref(&self) -> &Path {
        &self.0
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
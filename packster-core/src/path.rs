use std::{path::{PathBuf, Path, Component}, ffi::OsStr};

use serde::{Serialize, Deserialize};

use crate::{Result, Error, port::PathExt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct NormalizedPathBuf(PathBuf);

impl From<&Path> for NormalizedPathBuf {
    fn from(path: &Path) -> Self { NormalizedPathBuf(normalize_path(path)) }
}

impl AsRef<Path> for NormalizedPathBuf {
    fn as_ref(&self) -> &Path { &self.0 }
}

pub fn normalize_path<P: AsRef<Path>>(path_ref: P) -> PathBuf { //TODO MORE UT
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Deserialize, Serialize)]
pub struct Absolute<T: AsRef<Path>>(T);

impl <T: AsRef<Path>>Absolute<T> {
    pub fn assume_absolute(path: T) -> Self {
        Absolute(path)
    }

    pub fn try_absolute(path: T) -> Result<Self> {
        if path.as_ref().is_relative() {
            Err(Error::PathIsRelative(path.as_ref().to_path_buf()))
        } else {
            Ok(Absolute(path))
        }
    }

    pub fn try_to_relative<P: AsRef<Path>>(&self, base: &Absolute<P>) -> Result<Relative<&Path>> {
        Ok(
            Relative(
                self.0.as_ref().strip_prefix(base.as_ref())
                    .map_err(|_| Error::BaseNotInPath { base: base.to_absolute_path(), path: self.to_absolute_path() })?
            )
        )
    }

    pub fn as_absolute_path(&self) -> Absolute<&Path> {
        Absolute(self.0.as_ref())
    }

    pub fn to_absolute_path(&self) -> Absolute<PathBuf> {
        Absolute(self.0.as_ref().to_path_buf())
    }

    pub fn to_normalized_absolute_path(&self) -> Absolute<NormalizedPathBuf> {
        Absolute(self.0.as_ref().to_normalized_path())
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> Absolute<PathBuf> {
        Absolute(self.0.as_ref().join(path))
    }

    pub fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> Absolute<PathBuf> {
        Absolute(self.0.as_ref().with_extension(extension))
    }

    pub fn with_file_name<S: AsRef<OsStr>>(&self, filename: S) -> Absolute<PathBuf> {
        Absolute(self.0.as_ref().with_file_name(filename))
    }
}

impl <T: AsRef<Path>>AsRef<Path> for Absolute<T> {
    fn as_ref(&self) -> &Path { self.0.as_ref() }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Relative<T: AsRef<Path>>(T);

impl <T: AsRef<Path>>AsRef<Path> for Relative<T> {
    fn as_ref(&self) -> &Path { self.0.as_ref() }
}

impl <T: AsRef<Path>>Relative<T> {
    pub fn assume_relative(path: T) -> Self {
       Relative(path)
    }

    pub fn try_relative(path: T) -> Result<Self> {
        if path.as_ref().is_relative() {
            Ok(Relative(path))
        } else {
            Err(Error::PathIsRelative(path.as_ref().to_path_buf()))
        }
    }

    pub fn to_absolute<P: AsRef<Path>>(&self, base: &Absolute<P>) -> Absolute<PathBuf> {
        base.join(self.0.as_ref())
    }
}

impl PathExt for &Path {
    fn is_ancestor_of<P: AsRef<Path>>(&self, child_path: P) -> bool {
        if child_path.as_ref() == *self { return false; }
        child_path.as_ref()
            .ancestors()
            .any(|ancestor| { ancestor.to_normalized_path() == self.to_normalized_path() })
    }

    fn to_normalized_path(&self) -> NormalizedPathBuf {
        NormalizedPathBuf::from(*self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(not(unix))]
    pub fn test_normalize_path_handle_different_separators(){
        assert_eq!(
            normalize_path("C:\\this/is\\a/test/of/inconstistant\\separators"),
            PathBuf::from("C:\\this\\is\\a\\test\\of\\inconstistant\\separators")
        );
    }

    #[test]
    pub fn test_is_ancestor_of_same_path () {
        assert!(! Path::new("same/path").is_ancestor_of("same/path"));
        assert!(! Path::new("same\\path").is_ancestor_of("same/path"));

        assert!(! Path::new("/same/path").is_ancestor_of("/same/path"));
        assert!(! Path::new("\\same\\path").is_ancestor_of("/same/path"));

        assert!(! Path::new("C:\\same\\path").is_ancestor_of("C:\\same\\path"));
        assert!(! Path::new("C:\\same\\path").is_ancestor_of("C:/same/path"));
    }

    #[test]
    pub fn test_is_ancestor_of_relative_path() {
        assert!(Path::new("is/a").is_ancestor_of("is/a/relative/path"));
        assert!(! Path::new("be/a").is_ancestor_of("is/a/relative/path"));

        assert!(Path::new("is\\a").is_ancestor_of("is\\a/relative\\path"));
        assert!(! Path::new("be\\a").is_ancestor_of("is/a\\relative\\path"));
    }

    #[test]
    pub fn test_is_ancestor_of_absolute_path() {
        assert!(Path::new("/is/an").is_ancestor_of("/is/an/absolute/path"));
        assert!(! Path::new("/be/an").is_ancestor_of("/is/an/absolute/path"));
    }

    #[test]
    fn test_normalize_path_solve_one_and_two_dots () {
        assert_eq!(
            normalize_path("relative/./eaten/../path/depth/./.."),
            PathBuf::from("relative/path")
        );
    }

    #[test]
    fn test_normalize_path_solve_only_defined_levels () {
        assert_eq!(
            normalize_path("../../relative/../.."),
            PathBuf::from("../../..")
        );
    }

    #[test]
    fn test_normalize_path_preserve_root () {
        assert_eq!(
            normalize_path("/relative/./eaten/../path/depth/./.."),
            PathBuf::from("/relative/path")
        );
    }

    #[test]
    #[cfg(not(unix))]
    fn test_normalize_preserve_prefix () {
        assert_eq!(
            normalize_path("C:\\relative\\.\\eaten\\..\\path\\depth\\.\\.."),
            PathBuf::from("C:\\relative\\path")
        );
    }
}

use std::{path::{PathBuf, Path, Component}};

use serde::{Serialize, Deserialize};

use crate::{Result, Error, PathExt};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct NormalizedPath(PathBuf);

impl From<&Path> for NormalizedPath {
    fn from(path: &Path) -> Self { NormalizedPath(normalize_path(path)) }
}

impl AsRef<Path> for NormalizedPath {
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
pub struct AbsolutePath(PathBuf);

impl AbsolutePath {
    pub fn assume_absolute<P: AsRef<Path>>(path: P) -> Self {
        AbsolutePath(path.as_ref().to_path_buf())
    }

    pub fn try_to_relative(&self, base: &AbsolutePath) -> Result<RelativePath> {
        Ok(
            RelativePath::assume_relative(
                self.0.strip_prefix(base)
                    .map_err(|_| Error::BaseNotInPath { base: base.clone(), path: self.clone() })?
            )
        )
    }
}

impl TryFrom<&Path> for AbsolutePath {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self> {
        if path.is_relative() {
            Err(Error::PathIsRelative(path.to_path_buf()))
        } else {
            Ok(AbsolutePath::assume_absolute(path))
        }
     }
}

impl AsRef<Path> for AbsolutePath {
    fn as_ref(&self) -> &Path { &self.0 }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct RelativePath(PathBuf);

impl TryFrom<&Path> for RelativePath {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self> {
        if path.is_absolute() {
            Err(Error::PathIsAbsolute(path.to_path_buf()))
        } else {
            Ok(RelativePath::assume_relative(path))
        }
    }
}

impl <'a>AsRef<Path> for &'a RelativePath {
    fn as_ref(&self) -> &Path { &self.0 }
}

impl RelativePath {
    pub fn assume_relative<P: AsRef<Path>>(path: P) -> Self {
       RelativePath(path.as_ref().to_path_buf())
    }
}

impl PathExt for &Path {
    fn is_ancestor_of<P: AsRef<Path>>(&self, child_path: P) -> bool {
        if child_path.as_ref() == *self { return false; }
        child_path.as_ref()
            .ancestors()
            .any(|ancestor| { ancestor.to_normalized_path() == self.to_normalized_path() })
    }

    fn to_normalized_path(&self) -> NormalizedPath {
        NormalizedPath::from(*self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
    fn test_normalize_preserve_prefix () {
        assert_eq!(
            normalize_path("C:\\relative\\.\\eaten\\..\\path\\depth\\.\\.."),
            PathBuf::from("C:\\relative\\path")
        );
    }
}

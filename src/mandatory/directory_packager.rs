use crate::port::Packager;

pub struct DirectoryPackager {
    extension: String
}

impl Default for DirectoryPackager {
    fn default() -> Self {
        DirectoryPackager {
            extension: String::from("packster")
        }
    }
}

impl Packager for DirectoryPackager {}
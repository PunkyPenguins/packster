use std::{io::{empty, self}, path::Path};
use flate2::{write::GzEncoder, Compression, read::GzDecoder};
use tar::{Header, Builder, EntryType, Archive};
use packster_core::{Error as CoreError, port::{FileSystem, Archiver}, path::Absolute};
use crate::{Result, Error};

#[derive(Default)]
pub struct TarballArchiver;

//TODO add some logging and integration tests
//Note : this implementation does not covers: symlinks, hardlinks, access rights, owners, created / modified time.
impl Archiver for TarballArchiver {
    fn archive<F: FileSystem, P1: AsRef<Path>, P2: AsRef<Path>>(&self, filesystem: &F, project_path: Absolute<P1>, archive_path: Absolute<P2>) -> Result<()> {
        let writer = filesystem.open_write(archive_path)?;
        let encoder = GzEncoder::new(writer, Compression::default());
        let mut tar_builder = Builder::new(encoder);

        for found_entry_result in filesystem.walk(project_path.as_ref()) { //TODO optimize with rayon since fs support sending Send + Sync descriptors
            let found_entry = found_entry_result?;
            if found_entry.as_path() == project_path.as_ref() {
                continue;
            }
            let found_absolute_path = found_entry.as_absolute_path();
            let found_relative_path = found_absolute_path.try_to_relative(&project_path)?;

            let mut header = Header::new_gnu();

            if filesystem.is_file(found_entry.as_path()) {
                let size = found_entry.size();
                header.set_entry_type(EntryType::Regular);
                header.set_size(size);
                header.set_cksum();

                let reader = filesystem.open_read(found_entry.as_path())?;
                tar_builder.append_data(&mut header, &found_relative_path, reader).map_err(Error::from)?;
            } else if filesystem.is_directory(found_entry.as_path()) {
                header.set_entry_type(EntryType::Directory);
                header.set_size(0);
                tar_builder.append_data(&mut header, &found_relative_path, empty()).map_err(Error::from)?;
            }
        }

        Ok(())
    }

    fn extract<F: FileSystem, P1: AsRef<Path>, P2: AsRef<Path>>(&self, filesystem: &F, expand_path: Absolute<P1>, archive_path: Absolute<P2>) -> Result<()> {
        let reader = filesystem.open_read(archive_path)?;
        let decoder = GzDecoder::new(reader);
         let mut archive = Archive::new(decoder);

        let mut directories = Vec::new();
        for entry in archive.entries().map_err(Error::from)? {
            let mut node = entry.map_err(Error::from)?;
            match node.header().entry_type() {
                EntryType::Directory => directories.push(node),
                _ => {
                    let relative_file_path = node.path().map_err(Error::from)?;
                    let absolute_file_path = expand_path.join(relative_file_path);
                    if filesystem.exists(&absolute_file_path) {
                        return Err(CoreError::NodeAlreadyExists(absolute_file_path.into()))
                    }
                    if let Some(parent_absolute_path) = absolute_file_path.as_ref().parent() {
                        filesystem.create_dir_recursively(parent_absolute_path)?;
                    }
                    let mut writer = filesystem.open_write(absolute_file_path)?;
                    io::copy(&mut node, &mut writer).map_err(Error::from)?;
                }
            }
        }

        for directory in directories {
            let relative_directory_path = directory.path().map_err(Error::from)?;
            let absolute_directory_path = expand_path.join(relative_directory_path);
            filesystem.create_dir_recursively(absolute_directory_path)?;
        }

        Ok(())
    }
}

//TODO test extract and archive to/from InMemoryFileSystem

#[cfg(test)]
mod test {
    use packster_core::port::ReadOnlyFileSystem;

    use crate::InMemoryFileSystem;

    use super::*;

    #[test]
    fn test_archive_unarchive_reciprocal() -> Result<()> {
        let archiver = TarballArchiver;
        let filesystem = InMemoryFileSystem::default();

        filesystem.create_dir_recursively("/my/a_directory/a_subdirectory")?;
        filesystem.open_write("/my/a_first_file.txt")?
            .write_all(b"Hello world from atop").unwrap();
        filesystem.open_write("/my/a_directory/a_second_file.txt")?
            .write_all(b"Hello world from bottom").unwrap();
        filesystem.open_write("/punk_file.txt")?
            .write_all(b"I shall not be archived !").unwrap();

        archiver.archive(
            &filesystem,
            Absolute::assume_absolute("/my"),
            Absolute::assume_absolute("/my_archive.tar")
        )?;

        assert!(filesystem.is_file("/my_archive.tar"));
        assert!(filesystem.file_size("/my_archive.tar")? > 0);

        archiver.extract(
            &filesystem,
            Absolute::assume_absolute("/my_extracted"),
            Absolute::assume_absolute("/my_archive.tar")
        )?;

        assert!(filesystem.is_file("/my_extracted/a_first_file.txt"));
        assert_eq!(filesystem.read_to_string("/my_extracted/a_first_file.txt")?, "Hello world from atop");

        assert!(filesystem.is_file("/my_extracted/a_directory/a_second_file.txt"));
        assert_eq!(filesystem.read_to_string("/my_extracted/a_directory/a_second_file.txt")?, "Hello world from bottom");

        assert!(filesystem.is_directory("/my_extracted/a_directory"));

        assert!(!filesystem.is_file("/my_extracted/punk_file.txt"));

        Ok(())
    }
}
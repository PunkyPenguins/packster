use std::{io::empty, path::Path};
use flate2::{write::GzEncoder, Compression};
use tar::{Header, Builder, EntryType};
use packster_core::{port::{FileSystem, Archiver}, path::Absolute};
use crate::{Result, Error};

#[derive(Default)]
pub struct TarballArchiver;

//TODO add some logging and integration tests
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
                tar_builder.append_data(&mut header, &found_relative_path, empty()).map_err(Error::from)?;
            }
        }

        Ok(())
    }

    fn extract<F: FileSystem, P1: AsRef<Path>, P2: AsRef<Path>>(&self, filesystem: &F, expand_path: P1, archive_path: P2) -> Result<()> {
        todo!()
    }
}
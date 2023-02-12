use std::{io::empty, path::Path, fmt};
use flate2::{write::GzEncoder, Compression};
use tar::{Header, Builder, EntryType};
use packster_core::port::{IFileSystem, IArchiver};
use crate::{Result, Error};

#[derive(Default)]
pub struct TarballArchiver;

impl IArchiver for TarballArchiver {
    fn archive<F: IFileSystem, P: AsRef<Path>>(&self, filesystem: &F, project_path: P, archive_path: P) -> Result<()> {
        let writer = filesystem.open_write(archive_path)?;
        let encoder = GzEncoder::new(writer, Compression::default());
        let mut tar_builder = Builder::new(encoder);
        for found_entry_result in filesystem.walk(project_path.as_ref()) {
            let found_entry = found_entry_result?;
            let found_relative_path = found_entry.as_normalized_path().to_relative_path(project_path.as_ref());

            let mut header = Header::new_gnu();

            if filesystem.is_file(found_entry.as_path()) {
                let size = found_entry.size();
                header.set_entry_type(EntryType::Regular);
                header.set_size(size as u64);
                header.set_cksum();

                let reader = filesystem.open_read(&found_relative_path)?;
                tar_builder.append_data(&mut header, &found_relative_path, reader).map_err(Error::from)?;
            } else if filesystem.is_directory(found_entry.as_path()) {
                header.set_entry_type(EntryType::Directory);
                tar_builder.append_data(&mut header, found_relative_path, empty()).map_err(Error::from)?;
            }
        }

        Ok(())
    }
}


impl fmt::Display for TarballArchiver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tar.gz")
    }
}
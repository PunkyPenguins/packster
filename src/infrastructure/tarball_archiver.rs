use std::{io::empty, path::Path, fmt};

use crate::{Result, port::IArchiver, essential::port::IFileSystem};
use flate2::{write::GzEncoder, Compression};
use tar::{ Header, Builder, EntryType };

#[derive(Default)]
pub struct TarballArchiver;

impl IArchiver for TarballArchiver {
    fn archive<F: IFileSystem, P: AsRef<Path>>(&self, filesystem: &F, project_path: P, archive_path: P) -> Result<()> {
        let writer = filesystem.open_write(archive_path)?;
        let encoder = GzEncoder::new(writer, Compression::default());
        let mut tar_builder = Builder::new(encoder);
        for found_entry_result in filesystem.walk(project_path.as_ref()) {
            let found_entry = found_entry_result?;
            let mut header = Header::new_gnu();

            if filesystem.is_file(found_entry.as_path()) {
                let size = found_entry.size();
                header.set_entry_type(EntryType::Regular);
                header.set_size(size as u64);
                header.set_cksum();

                let reader = filesystem.open_read(found_entry.as_path())?;
                tar_builder.append_data(&mut header, found_entry.as_path(), reader)?; //TODO rework path to be always relative to project path
            } else if filesystem.is_directory(found_entry.as_path()) {
                header.set_entry_type(EntryType::Directory);
                tar_builder.append_data(&mut header, found_entry.as_path(), empty())?;
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
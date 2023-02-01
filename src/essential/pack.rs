use std::path::{PathBuf, Path};

use crate::{
    Result,
    Error,
    port::{ FileSystem, Archiver, Digester, PortageManifest }
};
use uniqueid::{IdentifierBuilder, IdentifierType};

pub fn generate_unique_id(name: &str) -> String { //TODO to infra
   let mut builder = IdentifierBuilder::default();

   builder.name(name);
   builder.add(IdentifierType::CPU);
   builder.add(IdentifierType::RAM);
   builder.add(IdentifierType::DISK);

   let identifier = builder.build();

   identifier.to_string(true)
}

pub struct PackCommand {
    portage_manifest_path: PathBuf,
    destination_directory_path: PathBuf
}

impl PackCommand {
    fn as_portage_manifest_path(&self) -> &Path { &self.portage_manifest_path }
    fn as_destination_directory_path(&self) -> &Path { &self.destination_directory_path }
}

impl PackCommand {
    fn new<P: AsRef<Path>>(portage_manifest_path: P, destination_directory_path: P) -> Self {
        PackCommand {
            portage_manifest_path: portage_manifest_path.as_ref().to_path_buf(),
            destination_directory_path: destination_directory_path.as_ref().to_path_buf()
        }
    }
}

//TODO implement transactions to rollback on error and guarantee state even on update
fn pack<F: FileSystem, A: Archiver, D: Digester>(
    filesystem: &mut F,
    archiver: &A,
    digester: &D,
    command: PackCommand
) -> Result<()> {
    let raw_manifest_string = filesystem.read_to_string(command.as_portage_manifest_path())?;
    if ! filesystem.exists(command.as_portage_manifest_path()) { return Err(Error::ManifestPathDoesNotExist(command.as_portage_manifest_path().to_path_buf())) }
    if filesystem.is_directory(command.as_portage_manifest_path()) { return Err(Error::ManifesPathIsADirectory(command.as_portage_manifest_path().to_path_buf())) }

    // Parse manifest
    let portage_path = command.as_portage_manifest_path().parent().ok_or_else(|| Error::ManifesPathIsADirectory(command.as_portage_manifest_path().to_path_buf()))?;
    let portage_manifest = <dyn PortageManifest>::parse(raw_manifest_string)?; //TODO validate the dto values with rules like no underscore in business in identifier

    let tmp_archive_path = command.as_destination_directory_path()
        .join(generate_unique_id(portage_manifest.as_identifier()));


    // Create archive
    // archiver.archive(tmp_archive_path, );

    // Compress archive
    let reader = filesystem.open_read(portage_path)?;
    let writer = filesystem.open_write(&tmp_archive_path)?;
    archiver.compress(reader, writer)?;

    // Generate Checksum
    let reader = filesystem.open_read(&tmp_archive_path)?; //TODO performance optimization : do read + hash + archive + copy in the same stream ?
    let checksum = format!("{:x?}", digester.generate_checksum(reader)?);

    // Normalize filename
    let final_archive_name = format!("{}_{}_{}_{}.packster", portage_manifest.as_identifier(), portage_manifest.as_version(), digester, checksum);
    let final_archive_path = tmp_archive_path.with_file_name(&final_archive_name);

    filesystem.rename(tmp_archive_path, final_archive_path)?;

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use crate::{
        port::ReadOnlyFileSystem,
        infrastructure::{ Compressor, FileDigester, InMemoryFileSystem }
    };

    #[test]
    fn test_static_packing() -> Result<()> {
        let mut filesystem = InMemoryFileSystem::default();
        filesystem.create("portage/hello_world.txt")?;
        filesystem.write_all("portage/hello_world.txt", b"Hello world !")?;
        filesystem.create("portage/packster.toml")?;

        let manifest = indoc!{r#"
            identifier = "static-package-a"
            version = "0.0.1"
        "#};

        filesystem.write_all("portage/packster.toml", manifest.as_bytes())?;

        let archiver = Compressor::default(); //Make CompressorMock implements FileSystem (utilises Mock) so we can test file content easily
        //TODO let digester = DigesterMock::new("whatever");
        let digester = FileDigester::default();
        pack(&mut filesystem, &archiver, &digester, PackCommand::new("portage/packster.toml", "repo"))?;

        assert!(filesystem.exists("repo/static-package-a_0.0.1_whatever.packster"));
        assert!(filesystem.is_file("repo/static-package-a_0.0.1_whatever.packster"));

        //TODO extract and see what's in the archive ! ? mock compressor & digester ?

        assert_eq!("Hello world !", &filesystem.read_to_string("repo/static-package-a_0.0.1_.packster/hello_world.txt")?);
        Ok(())
    }
}


// - package pack - create a package
// given a package source directory path ( fail if path does not exists or is not a directory )
// given a destination path
// reads packster-manifest.toml ( fail if file not exists or parsing fail )
// execute "any" and pack handlers with an executor ( powershell by example ) IF ANY
// create an archive of the directory in a tmp path
// compute a checksum
// move to path & rename archive with checksum in filename
// delete local location
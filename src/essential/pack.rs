use std::path::{PathBuf, Path};

use crate::{
    Result, Error,
    port::{ FileSystem, Archiver, Digester, DtoParser, PackageManifest, PackageManifestContract }
};


pub struct PackCommand {
    portage_manifest_path: PathBuf,
    destination_directory_path: PathBuf
}

impl PackCommand {
    fn as_portage_manifest_path(&self) -> &Path {
        &self.portage_manifest_path
    }

    fn as_destination_directory_path(&self) -> &Path {
        &self.destination_directory_path
    }
}

impl PackCommand {
    fn new<P: AsRef<Path>>(portage_path: P, destination_directory_path: P) -> Self {
        PackCommand {
            portage_manifest_path: portage_path.as_ref().to_path_buf(),
            destination_directory_path: destination_directory_path.as_ref().to_path_buf()
        }
    }
}

fn pack<F: FileSystem, A: Archiver, D: Digester>(
    filesystem: &mut F,
    archiver: &A,
    digester: &D,
    command: PackCommand
) -> Result<()> {
    let raw_manifest_string = filesystem.read_to_string(command.as_portage_manifest_path())?;
    if ! filesystem.exists(command.as_portage_manifest_path()) { return Err(Error::ManifestPathDoesNotExist(command.as_portage_manifest_path().to_path_buf())) }
    if filesystem.is_directory(command.as_portage_manifest_path()) { return Err(Error::ManifesPathIsADirectory(command.as_portage_manifest_path().to_path_buf())) }

    let portage_path = command.as_portage_manifest_path().parent().ok_or_else(|| Error::ManifesPathIsADirectory(command.as_portage_manifest_path().to_path_buf()))?;
    let portage_manifest = DtoParser::<PackageManifest>::parse(raw_manifest_string)?; //TODO validate the dto values with rules like no underscore

    let tmp_archive_path = archiver.create_archive(filesystem, portage_path, command.as_destination_directory_path())?;
    let checksum = digester.generate_checksum(filesystem, &tmp_archive_path)?;
    let final_archive_path = tmp_archive_path.with_file_name(format!("{}_{}_{}.packster", portage_manifest.as_identifier(), portage_manifest.as_version(), checksum));

    filesystem.rename(tmp_archive_path, final_archive_path)?;
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use crate::{
        port::{
            ReadOnlyFileSystem
        },
        mandatory::{
            mock::FileSystemMock,
            DirectoryArchiver, DirectoryDigester
        }
    };


    #[test]
    fn test_static_packing() -> Result<()> {
        let mut filesystem = FileSystemMock::default();
        filesystem.create("portage/hello_world.txt")?;
        filesystem.write_all("portage/hello_world.txt", b"Hello world !")?;
        filesystem.create("portage/packster.toml")?;

        let manifest = indoc!{r#"
            identifier = "static-package-a"
            version = "0.0.1"
        "#};

        filesystem.write_all("portage/packster.toml", manifest.as_bytes())?;
        let archiver = DirectoryArchiver::default();
        let digester = DirectoryDigester::default();
        pack(&mut filesystem, &archiver, &digester, PackCommand::new("portage/packster.toml", "repo"))?;

        assert!(filesystem.exists("repo/static-package-a_0.0.1_.packster"));
        assert!(filesystem.is_directory("repo/static-package-a_0.0.1_.packster"));

        assert!(filesystem.exists("repo/static-package-a_0.0.1_.packster/hello_world.txt"));
        assert!(filesystem.is_file("repo/static-package-a_0.0.1_.packster/hello_world.txt"));

        assert_eq!("Hello world !", &filesystem.read_to_string("repo/static-package-a_0.0.1_.packster/hello_world.txt")?);
        Ok(())
    }
}
use std::path::{PathBuf, Path};
use uniqueid::{IdentifierBuilder, IdentifierType};
use packster_core::{IFileSystem, IArchiver, IDigester, IProjectManifest};
use crate::{Result,Error, project_manifest};

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
    project_manifest_path: PathBuf,
    destination_directory_path: PathBuf
}

impl PackCommand {
    fn as_project_manifest_path(&self) -> &Path { &self.project_manifest_path }
    fn as_destination_directory_path(&self) -> &Path { &self.destination_directory_path }
}

impl PackCommand {
    fn new<P: AsRef<Path>>(project_manifest_path: P, destination_directory_path: P) -> Self {
        PackCommand {
            project_manifest_path: project_manifest_path.as_ref().to_path_buf(),
            destination_directory_path: destination_directory_path.as_ref().to_path_buf()
        }
    }
}

const PACKAGE_EXENSION : &'static str = "packster";

//TODO implement transactions to rollback on error and guarantee state even on update
fn pack<F: IFileSystem, A: IArchiver, D: IDigester>(
    filesystem: &F,
    archiver: &A,
    digester: &D,
    command: PackCommand
) -> Result<()> {
    if ! filesystem.exists(command.as_project_manifest_path()) { return Err(Error::ManifestPathDoesNotExist(command.as_project_manifest_path().to_path_buf()).into()) }
    if filesystem.is_directory(command.as_project_manifest_path()) { return Err(Error::ManifesPathIsADirectory(command.as_project_manifest_path().to_path_buf()).into()) }

    let raw_manifest_string = filesystem.read_to_string(command.as_project_manifest_path())?;

    // Parse manifest
    let project_path = command.as_project_manifest_path().parent().ok_or_else(|| Error::ManifesPathIsADirectory(command.as_project_manifest_path().to_path_buf()))?;
    let project_manifest = project_manifest::parse(raw_manifest_string)?; //TODO validate the dto values with rules like no underscore in business in identifier

    let tmp_archive_path = command.as_destination_directory_path()
        .join(generate_unique_id(project_manifest.as_identifier()))
        .with_extension(PACKAGE_EXENSION);

    //TODO trigger build event scripts + dependencies, etc ... profile management, etc ...

    // Create archive
    archiver.archive(filesystem, project_path, &tmp_archive_path)?;//TODO add metadata to manifest ? build timestamp, git commit ?

    // Generate Checksum
    let reader = filesystem.open_read(&tmp_archive_path)?; //TODO performance optimization : do read + hash + archive + copy in the same stream ?
    let checksum = hex::encode(digester.generate_checksum(reader)?);

    // Normalize filename
    let final_archive_name = format!(
        "{}_{}_{}_{}.{}.{}",
        project_manifest.as_identifier(),
        project_manifest.as_version(),
        digester,
        checksum,
        archiver,
        PACKAGE_EXENSION
    );
    let final_archive_path = tmp_archive_path.with_file_name(&final_archive_name);

    filesystem.rename(tmp_archive_path, final_archive_path)?;

    Ok(())
}


#[cfg(test)]
mod test {
    use std::{io::Read, fmt};

    use super::*;
    use indoc::indoc;
    use packster_core::IReadOnlyFileSystem;
    use packster_infrastructure::InMemoryFileSystem;


    #[test]
    fn test_static_packing() -> Result<()> {
        pub struct DigesterMock;

        impl IDigester for DigesterMock {
            fn generate_checksum<R: Read>(&self, _: R) -> Result<Vec<u8>> {
                Ok(hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap())
            }
        }

        impl fmt::Display for DigesterMock {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "mock")
            }
        }


        let mut filesystem = InMemoryFileSystem::default();
        filesystem.create_dir("project")?;
        filesystem.create_dir("repo")?;
        filesystem.write_all("project/hello_world.txt", b"Hello world !")?;
        filesystem.create("project/packster.toml")?;


        let manifest = indoc!{r#"
            identifier = "static-package-a"
            version = "0.0.1"
        "#};

        filesystem.write_all("project/packster.toml", manifest.as_bytes())?;

        let filesystem_as_archiver = InMemoryFileSystem::default();
        pack(&mut filesystem, &filesystem_as_archiver, &DigesterMock, PackCommand::new("project/packster.toml", "repo"))?;

        assert!(filesystem.exists("repo/static-package-a_0.0.1_mock_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.mock.packster"));
        assert!(filesystem.is_file("repo/static-package-a_0.0.1_mock_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.mock.packster"));

        assert!(filesystem_as_archiver.is_file("packster.toml"));
        assert!(filesystem_as_archiver.is_file("hello_world.txt"));
        assert_eq!(filesystem_as_archiver.read_to_string("hello_world.txt")?, String::from("Hello world !"));

        Ok(())
    }
}
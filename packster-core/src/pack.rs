use crate::{Result,Error, PACKAGE_EXENSION, IdentifierGenerator, Parser, PackCommand, FileSystem, Archiver, Digester, entity::Project};

//TODO distinguish I/O ( filesystem, id generator, data-sources ) and transformations ( parser, archiver, digester ) ?
//TODO avoid onion, keep direct core / outward separation

//TODO implement transactions to rollback on error and guarantee state even on update
//TODO all that should definitely end into Core
fn pack<F: FileSystem, A: Archiver, D: Digester, I: IdentifierGenerator, P: Parser, C: PackCommand>(
    filesystem: &F,
    archiver: &A,
    digester: &D,
    identifier_generator: &I,
    parser: &P,
    command: &C,
) -> Result<()> {
    if ! filesystem.exists(command.as_project_manifest_path()) { return Err(Error::ManifestPathDoesNotExist(command.as_project_manifest_path().to_path_buf())) }
    if filesystem.is_directory(command.as_project_manifest_path()) { return Err(Error::ManifesPathIsADirectory(command.as_project_manifest_path().to_path_buf())) }

    let raw_manifest_string = filesystem.read_to_string(command.as_project_manifest_path())?;

    // Parse manifest
    let project_path = command.as_project_manifest_path().parent().ok_or_else(|| Error::ManifesPathIsADirectory(command.as_project_manifest_path().to_path_buf()))?;
    let project : Project = parser.parse(raw_manifest_string)?; //TODO validate the dto values with rules like no underscore in business in identifier

    let tmp_archive_path = command.as_destination_directory_path()
        .join(identifier_generator.generate_identifier(project.as_identifier()))
        .with_extension(PACKAGE_EXENSION);

    //TODO trigger build event scripts + dependencies, etc ... profile management, etc ...

    // Create archive
    archiver.archive(filesystem, project_path, &tmp_archive_path)?;//TODO add metadata to manifest ? build timestamp, git commit ?

    // Generate Checksum
    let reader = filesystem.open_read(&tmp_archive_path)?;
    let checksum = hex::encode(digester.generate_checksum(reader)?);

    // Normalize filename
    let final_archive_name = format!( //TODO to core ?
        "{}_{}_{}_{}.{}.{}",
        project.as_identifier(),
        project.as_version(),
        digester,
        checksum,
        archiver,
        PACKAGE_EXENSION
    );
    let final_archive_path = tmp_archive_path.with_file_name(final_archive_name);

    filesystem.rename(tmp_archive_path, final_archive_path)?;

    Ok(())

    // Pack::new().read_project(filesystem).archive(filesystem, archiver).checksum(digest).finalize(filesystem) => () State pattern ?
}


#[cfg(test)]
mod test {
    use std::{io::Read, fmt};

    use super::*;
    use indoc::indoc;

    //TODO problem here : either we move in_memory_filesystem in core, either we put such test outside core, either we put all this into another crate
    // use crate::port::ReadOnlyFileSystem;
    // use packster_infrastructure::InMemoryFileSystem;
    // TODO : assume that test to be integration and move it to a dedicated test package

    // #[test]
    // fn test_static_packing() -> Result<()> {
    //     pub struct DigesterMock;

    //     impl Digester for DigesterMock {
    //         fn generate_checksum<R: Read>(&self, _: R) -> Result<Vec<u8>> {
    //             Ok(hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap())
    //         }
    //     }

    //     impl fmt::Display for DigesterMock {
    //         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //             write!(f, "mock")
    //         }
    //     }

    //     let filesystem = InMemoryFileSystem::default();
    //     filesystem.create_dir("project")?;
    //     filesystem.create_dir("repo")?;
    //     filesystem.write_all("project/hello_world.txt", b"Hello world !")?;
    //     filesystem.create("project/packster.toml")?;


    //     let manifest = indoc!{r#"
    //         identifier = "static-package-a"
    //         version = "0.0.1"
    //     "#};

    //     filesystem.write_all("project/packster.toml", manifest.as_bytes())?;

    //     let filesystem_as_archiver = InMemoryFileSystem::default();
    //     pack(&filesystem, &filesystem_as_archiver, &DigesterMock, PackCommand::new("project/packster.toml", "repo"))?;

    //     assert!(filesystem.exists("repo/static-package-a_0.0.1_mock_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.mock.packster"));
    //     assert!(filesystem.is_file("repo/static-package-a_0.0.1_mock_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.mock.packster"));

    //     assert!(filesystem_as_archiver.is_file("packster.toml"));
    //     assert!(filesystem_as_archiver.is_file("hello_world.txt"));
    //     assert_eq!(filesystem_as_archiver.read_to_string("hello_world.txt")?, String::from("Hello world !"));

    //     Ok(())
    // }
}
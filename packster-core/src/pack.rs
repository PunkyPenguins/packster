use std::path::{PathBuf, Path};

use crate::{Result,Error, PACKAGE_EXENSION, IdentifierGenerator, Parser, FileSystem, Archiver, Digester, entity::Project};

pub struct PackCommand {
    project_manifest_path: PathBuf,
    destination_directory_path: PathBuf
}

impl PackCommand {
    pub fn new<P: AsRef<Path>>(project_manifest_path: P, destination_directory_path: P) -> Self {
        PackCommand {
            project_manifest_path: project_manifest_path.as_ref().to_path_buf(),
            destination_directory_path: destination_directory_path.as_ref().to_path_buf()
        }
    }

    pub fn as_project_manifest_path(&self) -> &Path {
        &self.project_manifest_path
    }

    pub fn as_destination_directory_path(&self) -> &Path {
        &self.destination_directory_path
    }
}

//TODO distinguish I/O ( filesystem, id generator, data-sources ) and transformations ( parser, archiver, digester ) ?
//TODO avoid onion, keep direct core / outward separation

//TODO implement transactions to rollback on error and guarantee state even on update
//TODO all that should definitely end into Core
pub fn pack<F: FileSystem, A: Archiver, D: Digester, I: IdentifierGenerator, P: Parser>(
    filesystem: &F,
    archiver: &A,
    digester: &D,
    identifier_generator: &I,
    parser: &P,
    command: &PackCommand,
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


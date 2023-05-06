use std::path::PathBuf;

use crate::{
    Result,
    port::{ReadOnlyFileSystem, Parser, FileSystem, Archiver, Digester, UniqueIdentifierGenerator},
    path::Absolute,
    domain::{Project, Package, Version},
    PACKAGE_EXTENSION,
};

use super::{Operation, New};

pub struct PackRequest {
    project_workspace: Absolute<PathBuf>,
    package_output_directory: Absolute<PathBuf>,
}

impl PackRequest {
    pub fn new(project_workspace: Absolute<PathBuf>, package_output_directory: Absolute<PathBuf>) -> Self {
        PackRequest { project_workspace, package_output_directory }
    }
}

pub type PackOperation<S> = Operation<S, PackRequest>;

impl PackOperation<New> {
    pub fn parse_project<F: ReadOnlyFileSystem, P: Parser>(self, filesystem: &F, parser: &P) -> Result<PackOperation<Project>> {
        let manifest_path = self.request.project_workspace.as_ref().join("packster.toml");

        let raw_manifest_string = filesystem.read_to_string(manifest_path)?;
        Ok(Self::with_state(self.request, parser.parse(raw_manifest_string)?))
    }
}

pub struct IdentifiedProject {
    pub project: Project,
    pub identifier: String
}

impl PackOperation<Project> {
    pub fn generate_unique_identity<I: UniqueIdentifierGenerator>(self, identifier_generator: &I) -> PackOperation<IdentifiedProject> {
        Self::with_state(
            self.request,
            IdentifiedProject {
                identifier: identifier_generator.generate_identifier(),
                project: self.state
            }
        )
    }
}

pub struct ArchivedProject {
    pub project: Project,
    pub archive_path: Absolute<PathBuf>
}

impl PackOperation<IdentifiedProject> {
    pub fn archive<F: FileSystem, A: Archiver>(self, filesystem: &F, archiver: &A) -> Result<PackOperation<ArchivedProject>> {
        let archive_path = self.request.package_output_directory
            .join(self.state.identifier)
            .with_extension(PACKAGE_EXTENSION);

        archiver.archive(
            filesystem,
            self.request.project_workspace.as_absolute_path(),
            archive_path.as_absolute_path()
        )?;

        Ok(
            Self::with_state(
                self.request,
                ArchivedProject {
                    project: self.state.project,
                    archive_path
                }
            )
        )
    }
}

pub struct DigestedArchivedProject {
    pub archived: ArchivedProject,
    pub digest: Vec<u8>
}

impl PackOperation<ArchivedProject> {
    pub fn digest<F: ReadOnlyFileSystem, D: Digester>(self, filesystem: &F, digester: &D) -> Result<PackOperation<DigestedArchivedProject>> {
        let digest = digester.generate_checksum(filesystem.open_read(&self.state.archive_path)?)?;
        Ok(
            Self::with_state(
                self.request,
                DigestedArchivedProject {
                    archived: self.state,
                    digest
                }
            )
        )
    }
}

impl PackOperation<DigestedArchivedProject> {
    pub fn finalize<F: FileSystem>(self, filesystem: &F, packster_version: &str) -> Result<PackOperation<Package>> {
        let DigestedArchivedProject {
            digest,
            archived: ArchivedProject {
                project,
                archive_path,
            }
        } = self.state;

        let package = Package::new(project, digest, Version::new(packster_version));
        let final_archive_path = archive_path.with_file_name(package.to_file_name());

        filesystem.rename(archive_path, final_archive_path)?;
        Ok(Self::with_state(self.request, package))
    }
}
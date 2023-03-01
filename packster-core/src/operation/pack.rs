use std::path::{PathBuf, Path};

use crate::{ReadOnlyFileSystem, Parser, Project, FileSystem, Archiver, Digester, IdentifierGenerator, Result, domain::Package};

use super::{Operation, New};

//Project -> Package

pub struct PackRequest {
    project_workspace: PathBuf,
    package_output_directory: PathBuf,
}

impl PackRequest {
    pub fn new<P: AsRef<Path>>(project_workspace: P, package_output_directory: P) -> Self {
        PackRequest {
            project_workspace: project_workspace.as_ref().to_owned(),
            package_output_directory: package_output_directory.as_ref().to_owned()
        }
    }
}

pub type PackOperation<S> = Operation<S, PackRequest>;

impl PackOperation<New> {
    pub fn parse_project<F: ReadOnlyFileSystem, P: Parser>(self, filesystem: &F, parser: &P) -> Result<PackOperation<Project>> {
        let manifest_path = self.request.project_workspace.join("packster.toml");

        let raw_manifest_string = filesystem.read_to_string(manifest_path)?;
        Ok(Self::with_state(self.request, parser.parse(raw_manifest_string)?))
    }
}

pub struct IdentifiedProject {
    pub project: Project,
    pub identifier: String
}

impl PackOperation<Project> {
    pub fn generate_unique_identity<I: IdentifierGenerator>(self, identifier_generator: &I) -> PackOperation<IdentifiedProject> {
        Self::with_state(
            self.request,
            IdentifiedProject {
                identifier: identifier_generator.generate_identifier(self.state.as_identifier()),
                project: self.state
            }
        )
    }
}

pub struct ArchivedProject {
    pub project: Project,
    pub archive_path: PathBuf,
    pub archiver_representation: String
}

impl PackOperation<IdentifiedProject> {
    pub fn archive<F: FileSystem, A: Archiver>(self, filesystem: &F, archiver: &A) -> Result<PackOperation<ArchivedProject>> {
        let archive_path = self.request.package_output_directory
            .join(self.state.identifier)
            .with_extension("packster");

        archiver.archive(filesystem, &self.request.project_workspace, &archive_path)?;
        let state = ArchivedProject {
            project: self.state.project,
            archiver_representation: archiver.to_string(),
            archive_path
        };
        Ok(Self::with_state(self.request, state))
    }
}

pub struct DigestedArchivedProject {
    pub archived: ArchivedProject,
    pub digest: Vec<u8>,
    pub digester_representation: String
}

impl PackOperation<ArchivedProject> {
    pub fn digest<F: ReadOnlyFileSystem, D: Digester>(self, filesystem: &F, digester: &D) -> Result<PackOperation<DigestedArchivedProject>> {
        let digest = digester.generate_checksum(filesystem.open_read(&self.state.archive_path)?)?;
        let state = DigestedArchivedProject {
            archived: self.state,
            digest,
            digester_representation: digester.to_string()
        };
        Ok(Self::with_state(self.request, state))
    }
}

impl PackOperation<DigestedArchivedProject> {
    pub fn finalize<F: FileSystem>(self, filesystem: &F) -> Result<Package> {
        let DigestedArchivedProject {
            archived: ArchivedProject {
                project,
                archive_path,
                archiver_representation
            },
            digest,
            digester_representation
        } = self.state;

        let package = Package::new(project, digester_representation, archiver_representation, digest);
        let final_archive_path = archive_path.with_file_name(package.file_name());

        filesystem.rename(archive_path, final_archive_path)?;
        Ok(package)
    }
}
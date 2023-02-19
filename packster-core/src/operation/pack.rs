use std::path::{PathBuf, Path};

use crate::{ReadOnlyFileSystem, Parser, Project, FileSystem, Archiver, Digester, IdentifierGenerator, Result};

use super::{Operation, New};

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

pub struct Identified {
    pub project: Project,
    pub identifier: String
}

impl PackOperation<Project> {
    pub fn generate_unique_identity<I: IdentifierGenerator>(self, identifier_generator: &I) -> PackOperation<Identified> {
        Self::with_state(
            self.request,
            Identified {
                identifier: identifier_generator.generate_identifier(self.state.as_identifier()),
                project: self.state
            }
        )
    }
}

pub struct Archived {
    pub project: Project,
    pub archive_path: PathBuf,
    pub archiver_representation: String
}

impl PackOperation<Identified> {
    pub fn archive<F: FileSystem, A: Archiver>(self, filesystem: &F, archiver: &A) -> Result<PackOperation<Archived>> {
        let archive_path = self.request.package_output_directory
            .join(self.state.identifier)
            .with_extension("packster");

        archiver.archive(filesystem, &self.request.project_workspace, &archive_path)?;
        let state = Archived {
            project: self.state.project,
            archiver_representation: archiver.to_string(),
            archive_path
        };
        Ok(Self::with_state(self.request, state))
    }
}

pub struct Digested {
    pub archived: Archived,
    pub digest: String,
    pub digester_representation: String
}

impl PackOperation<Archived> {
    pub fn digest<F: ReadOnlyFileSystem, D: Digester>(self, filesystem: &F, digester: &D) -> Result<PackOperation<Digested>> {
        let digest_bytes = digester.generate_checksum(filesystem.open_read(&self.state.archive_path)?)?;
        let digest = hex::encode(digest_bytes);
        let state = Digested {
            archived: self.state,
            digest,
            digester_representation: digester.to_string()
        };
        Ok(Self::with_state(self.request, state))
    }
}

pub struct Package; //TODO FROM DOMAIN !!

impl PackOperation<Digested> {
    pub fn finalized<F: FileSystem>(self, filesystem: &F) -> Result<Package> {
        let final_archive_name = format!(
            "{}_{}_{}_{}.{}.{}",
            self.state.archived.project.as_identifier(),
            self.state.archived.project.as_version(),
            self.state.digester_representation,
            self.state.digest,
            self.state.archived.archiver_representation,
            "packster"
        );
        let final_archive_path = self.state.archived.archive_path.with_file_name(final_archive_name);

        filesystem.rename(self.state.archived.archive_path, final_archive_path)?;
        Ok(Package)
    }
}
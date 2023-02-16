draw bounded context
detect my actors
understand types and states transform

## TypeStates

type PositiveNumber = uint
type String64 = str

type AbsolutePath
type Directory = AbsolutePath
type File = AbsolutePath

type ValidCharacter = Numeric | LowercaseLetter | Dash
type ProjectName = ValidString<ValidCharacter, String64>

type VersionRequirement = ... ( delegate to rust-semver library )
type Version = {
    major: PositiveNumber,
    minor: PositiveNumber,
    patch: PositiveNumber
}

type ProjectWorkspace = Directory
type ProjectManifest = {
    name: ProjectName,
    version: Version
}

type Project = {
    project_manifest: ProjectManifest,
    workspace: ProjectWorkspace
}

type Digester = Sha256 | Sha512
type Checksum = { digester: Digester, value: bytes }
type VerifiedArchive = File
type Package = {
    project_manifest: ProjectManifest,
    checksum: Checksum,
    archive: VerifiedArchive
}

type DeploymentIdentifier = {
    name: ProjectName,
    version: Version,
    constraint: VersionRequirement
}

type InstallLocation = Directory
type Deployment = {
    install_location: InstallLocation,
    directory: Directory,
    identifier: DeploymentIdentifier,
    project_manifest: ProjectManifest
}

type InstalledDeployment = Deployment
type UninstalledDeployment = UninstalledDeployment

type Bundle = Vec<VersionRequirement>

fn verify_archive_integrity = (File * Checksum) -> VerifiedArchive
fn install_package = (Package * InstallLocation) -> &[InstalledDeployment]
fn pack_project = Project -> Package
fn list_installed = InstallLocation -> &[InstalledDeployment]
fn uninstall_deployment = (DeploymentIdentifier * InstallLocation) -> UninstalledDeployment

fn install_bundle = (Bundle * InstallLocation) -> &[InstalledDeployment]

## Later

Dependency
Parameter / Feature ?
DefaultInstallLocation
Resource
    Environment
    ExecutableInPath
    SharedDirectory
ResourceRequirement
Handler / Executor / Runtime ?
License

Source = Vec<Package>

publish = (Package * Source) -> Source
unpublish = (ProjectName * Source) -> Source
fn install = (ProjectName * VersionRequirement * InstallLocation) -> &[InstalledDeployment]

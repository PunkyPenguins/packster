draw bounded context
detect my actors
understand types and states transform

## TypeStates

type PositiveNumber = uint
type String64 = str

type AbsolutePath
type File = AbsolutePath
type Directory = AbsolutePath
type ExistingDirectory = Directory

type ValidCharacter = Numeric | LowercaseLetter | Dash
type ProjectName = 64CharacterLong<StringWith<ValidCharacter>>

type VersionRequirement = ... ( delegate to rust-semver library )
type Version = {
    major: PositiveNumber,
    minor: PositiveNumber,
    patch: PositiveNumber
}

type ProjectWorkspace = ExistingDirectory
type ProjectIdentifier  = {
    name: ProjectName,
    version: Version
}

type Project = {
    identifier: ProjectIdentifier,
    workspace: ProjectWorkspace
}

type Digest = Sha256 | Sha512
type Checksum = { digester: Digest, value: bytes }
type Archive = File
type VerifiedArchive = File
type PackageIdentifier = Checksum
type Package = {
    identifier: PackageIdentifier,
    project_identifier: ProjectIdentifier,
    archive: VerifiedArchive,
}

type DeploymentIdentifier = PackageIdentifier
type Deployment = {
    identifier: DeploymentIdentifier,
    project_identifier: ProjectIdentifier,
    directory: ExistingDirectory,
    constraint: MatchingVersionRequirement
}

type ContainerDirectory = ExistingDirectory
type Container = {
    directory: ContainerDirectory,
    deployment_list: Vec<Deployment>
}

type Bundle = Vec<VersionRequirement>

fn verify_directory_exists = (Directory * FileSystem) -> ExistingDirectory
fn match_version_requirement = (VersionRequirement * Version) -> MatchingVersionRequirement
fn verify_archive_integrity = (Archive * Checksum) -> VerifiedArchive
fn pack_project = Project -> Package
fn deploy_package = (Package * ContainerDirectory) -> Container
fn list_deployed = ContainerDirectory -> Container
fn undeploy = (DeploymentIdentifier * ContainerDirectory) -> Container

fn install_bundle = (Bundle * Container) -> Container


/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/location_manifest.json (lock file)

Project -> Package -> Deployment

## Later

Dependency
Parameter / Feature ?
DefaultInstallLocation
Resource
    Container
    ExecutableInPath
    SharedDirectory
ResourceRequirement
Handler / Executor / Runtime ?
License

Source = Vec<Package>

publish = (Package * Source) -> Source
unpublish = (ProjectName * Source) -> Source
fn install = (ProjectName * VersionRequirement * InstallLocation) -> &[InstalledDeployment]

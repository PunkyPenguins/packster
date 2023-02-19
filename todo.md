## todo

- stop round tripping & get hands on
- start a cli package
    - with a pack & install command
- create a core/domain
    - essential complexity
    - Project / Version / Package concepts
- create a core/operations
    - required complexity
    - pack / install operations
        - use pattern build to split out super complicated pack signature
            Package::builder(pack_command).read_project(filesystem).archive(filesystem, archiver).checksum(digest).rename(filesystem).build()
            Builder / TypeState / Command
            or NewType ?
            type Package = Digested<Archived<Project>>

    - File / Directory / FileSystem concepts


- consider using binary header for archive so filename doesn't matter ( only human readable amenity )




## TypeStates

type PositiveNumber = uint
type String64 = str

type AbsolutePath = PathBuf
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

type ProjectIdentifier  = {
    name: ProjectName,
    version: Version
}

type Asset = File
type Project = {
    identifier: ProjectIdentifier,
    assets: Vec<Asset>
}

type Digest = Sha256 | Sha512
type Checksum = { digester: Digest, value: bytes }
type Archive = File
type PackageIdentifier = Checksum
type Package = {
    identifier: PackageIdentifier,
    project_identifier: ProjectIdentifier,
    archive: Archive,
}

type DeploymentIdentifier = PackageIdentifier
type Deployment = {
    identifier: DeploymentIdentifier,
    project_identifier: ProjectIdentifier,
    directory: Directory,
    constraint: MatchingVersionRequirement
}

type ContainerDirectory = ExistingDirectory
type Container = {
    directory: ContainerDirectory,
    deployment_list: Vec<Deployment>
}

type Bundle = Vec<VersionRequirement>


fn verify_directory_exists = Directory -> ExistingDirectory
fn match_version_requirement = (VersionRequirement * Version) -> MatchingVersionRequirement
fn pack_project = Project -> Package
fn deploy_package = (Package * ContainerDirectory) -> Container
fn list_deployed = ContainerDirectory -> Container
fn undeploy = (DeploymentIdentifier * ContainerDirectory) -> Container

fn install_bundle = (Bundle * Container) -> Container


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

## draft



/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/7cedFe/deployment_manifest.json
/usr/bin/location/location_manifest.json (lock file)

Project -> Package -> Deployment

draw bounded context
detect my actors
    my actors are developers :
        - trying to distribute theirs software, content or library
        - trying to deploy software content or library on their computer
understand types and states transform




struct PackCommand<S> {
    project_manifest_path: PathBuf,
    package_path: PathBuf,
    state: S
}


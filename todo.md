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

type Project = {
    name: ProjectName,
    version: Version
}

type Digest = Sha256 | Sha512
type Digest = { digester: Digest, value: bytes }

type Archiver = TarGz | Lzma
type Archive = { archiver: Archiver, file: File }

type PackageIdentifier = Digest
type Package = {
    identifier: PackageIdentifier,
    project: Project,
    archive: Archive,
}

type Deployment = {
    package: Package,
    project: Project,
    directory: Directory,
    constraint: MatchingVersionRequirement
}

type LocationDirectory = ExistingDirectory
type Location = {
    directory: LocationDirectory,
    deployment_list: Vec<Deployment>
}

type Bundle = Vec<VersionRequirement>


fn verify_directory_exists = Directory -> ExistingDirectory
fn match_version_requirement = (VersionRequirement * Version) -> MatchingVersionRequirement
fn pack_project = Project -> Package

fn deploy_package = (Package * Location) -> Location

fn list_deployed = LocationDirectory -> Location
fn undeploy = (DeploymentIdentifier * LocationDirectory) -> Location

fn install_bundle = (Bundle * Location) -> Location



## Later

Dependency
Parameter / Feature ?
DefaultInstallLocation
Resource
    Location
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



verifiy_project_manifest_path = PathBuf -> Result<PathBuf, Error>

packster.exe location create
Create DeploymentLocation
- creer un dossier ( vide )
- creer un lockfile.json ( avec une liste de déploiement vide )


packster.exe package install
"Install"
Verify package integrity (checksum)
Verify that the package is not already present in the location <= TODO test
Unarchive


## Prochaine session

Dadou code la création de Location et je le supervise



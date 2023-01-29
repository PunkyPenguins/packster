
use std::{path::{Path, PathBuf}, collections::HashSet};

pub enum PacksterError {}
type Result<T> = std::result::Result<T, PacksterError>;
trait FileSystem {}
trait PackageSource {} //search / liste

#[derive(PartialEq, Eq)]
struct ValidIdentifier(String);

struct Dependency {
    identifier: ValidIdentifier,
    version_constraint: VersionConstraint
}

#[derive(Default)]
struct Bundle(Vec<Dependency>);

struct Location {
    identifier: ValidIdentifier,
    bundle: Bundle,
    system_resources: Vec<Resource>,
    deploy_path: PathBuf,
    is_system: bool
}

pub enum Event {
    Pack,
    Install,
    Upgrade,
    Uninstall,
    Any
}

impl Default for Event {
    fn default() -> Self {
        Self::Any
    }
}

struct Executor(Resource);

struct Handler {
    executor: Executor,
    event: Event,
    script: PathBuf
}

struct Parameter {
    key: String,
    default: String,
    ask_user: bool,
    overrideable: bool,
    event: Event
}

struct Version(String);

struct Author(String);

struct License(String);

struct PackageManifest {
    identifier: ValidIdentifier,
    version: Version,
    description: Option<String>,
    author: Option<Author>,
    license: Option<License>,
    parameters: HashSet<Parameter>,
    handles: HashSet<Handler>,
    resources: HashSet<Resource>,
    dependencies: Bundle
}

#[cfg(test)]
impl Default for License {
    fn default() -> Self {
        todo!()
    }
}
#[cfg(test)]
impl Default for Version {
    fn default() -> Self {
        todo!()
    }
}

#[cfg(test)]
impl Default for Author {
    fn default() -> Self {
        todo!()
    }
}

struct PackageBase {
    manifest: PackageManifest,
    path: PathBuf
}

#[derive(PartialEq, Eq)]
enum Resource {
    Environment(String),
    ExecutableInPath(String),
    SharedDirectory(PathBuf),
    TcpNetPort(u16),
    Executor(ValidIdentifier)
}

enum VersionConstraint {
    Latest,
    StrictSemver(String)
}

struct Package;
struct Deployment;


fn pack<F: FileSystem, R: PackageSource>(fs: F, package_source: PackageBase, repository: R) { todo!() }

fn deploy_file<F: FileSystem>(fs: F, location: Location, package_file: &Path) { todo!() }

fn install_location_on_system() { todo!() }

fn uninstall_location_on_system() { todo!() }

fn install_location_on_session() { todo!() }

fn uninstall_location_on_session() { todo!() }

fn deploy_bundle() { todo!() }

fn deploy_dependency(){ todo!() }

fn clean_bundle() { todo!() }

fn clean_dependency() { todo!() }

fn add_source() { todo!() }

fn remove_source() { todo!() }


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test(){
        assert!(false)
    }
}



pub struct PortageManifestDtoBuilder {
    identifier: Option<String>,
    version: Option<String>
}

impl PortageManifestDtoBuilder {
    fn with_identifier<S: AsRef<str>>(self, s: S) -> Self {
        self.identifier = Some(s.as_ref().to_string());
        self
    }

    fn with_version<S: AsRef<str>>(self, s: S) -> Self {
        self.version = Some(s.as_ref().to_string());
        self
    }

    fn build(self) -> DtoResult<PortageManifestDto> {
        let identifier = self.identifier.ok_or_else(|| DtoError::MissingMandatoryField{ entity_name: "PortageManifest", field_name: "identifier" })?;
        let version = self.version.ok_or_else(|| DtoError::MissingMandatoryField{ entity_name: "PortageManifest", field_name: "identifier" })?;

        Ok(PortageManifestDto { identifier, version })
    }
}

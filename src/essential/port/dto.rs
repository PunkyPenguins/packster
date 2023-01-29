pub struct PackageManifest;

pub trait PackageManifestContract {
    fn as_identifier(&self) -> &str;
    fn as_version(&self) -> &str;
}

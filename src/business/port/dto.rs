pub trait PortageManifest {
    fn as_identifier(&self) -> &str;
    fn as_version(&self) -> &str;
}

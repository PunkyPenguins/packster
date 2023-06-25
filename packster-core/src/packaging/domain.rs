mod generic;
pub use generic::*;

mod entity;
pub use entity::*;


pub trait AsPackage {
    fn as_package(&self) -> &Package;
}

pub trait AsLocation {
    fn as_location(&self) -> &DeployLocation;
}

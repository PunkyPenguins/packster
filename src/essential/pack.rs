use std::path::{PathBuf, Path};

use crate::{port::{FileSystem, Packager}, Result};

pub struct PackCommand {
    portage_path: PathBuf,
    destination_directory_path: PathBuf
}

impl PackCommand {
    fn new<P: AsRef<Path>>(portage_path: P, destination_directory_path: P) -> Self {
        PackCommand {
            portage_path: portage_path.as_ref().to_path_buf(),
            destination_directory_path: destination_directory_path.as_ref().to_path_buf()
        }
    }
}

fn pack<F: FileSystem, P: Packager>(
    filesystem: &mut F,
    packager: &mut P,
    command: PackCommand
) -> Result<()> {
    todo!()
}



#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use crate::mandatory::{
        mock::FileSystemMock,
        DirectoryPackager
    };

    #[test]
    fn test_static_packing() -> Result<()> {
        let mut filesystem = FileSystemMock::default();
        filesystem.create("portage/hello_world.txt")?;
        filesystem.write_all("portage/hello_world.txt", b"Hello world !")?;
        filesystem.create("portage/packster.toml")?;
        filesystem.write_all("portage/packster.toml", indoc! { r#"
            identifier = "static-package-a"
            version = "0.0.1"
        "# }.as_bytes())?;

        let mut packager = DirectoryPackager::default();
        pack(&mut filesystem, &mut packager, PackCommand::new("portage", "repo"))?;

        assert!(filesystem.exists("repo/static-package-a_0.0.1_.packster"));
        assert!(filesystem.is_directory("repo/static-package-a_0.0.1_.packster"));

        assert!(filesystem.exists("repo/static-package-a_0.0.1_.packster/hello_world.txt"));
        assert!(filesystem.is_file("repo/static-package-a_0.0.1_.packster/hello_world.txt"));

        assert_eq!("Hello world !", &filesystem.read_to_string("repo/static-package-a_0.0.1_.packster/hello_world.txt")?);
        Ok(())
    }
}
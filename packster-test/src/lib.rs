#[cfg(test)]
mod test {
    use std::{io::Read, path::{PathBuf, Path}};
    use indoc::indoc;

    use packster_core::{
        port::{
            Digester,
            ReadOnlyFileSystem,
            FileSystem,
            UniqueIdentifierGenerator,
        },
        Result,
        operation::{PackRequest, Operation, New, InitLocationRequest},
        path::Absolute, LOCKFILE_NAME
    };
    use packster_infrastructure::{
        InMemoryFileSystem,
        Toml,
        Json
    };

    #[test]
    fn test_static_packing() -> Result<()> {
        pub struct DigesterMock;

        impl Digester for DigesterMock {
            fn generate_checksum<R: Read>(&self, _: R) -> Result<Vec<u8>> {
                Ok(hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap())
            }
        }

        pub struct UniqueIdentifierGeneratorMock;

        impl UniqueIdentifierGenerator for UniqueIdentifierGeneratorMock {
            fn generate_identifier(&self) -> String {
                String::from("123456")
            }
        }

        let filesystem = InMemoryFileSystem::default();
        filesystem.create_dir("/project")?;
        filesystem.create_dir("/repo")?;
        filesystem.write_all("/project/hello_world.txt", b"Hello world !")?;
        filesystem.create("/project/packster.toml")?;


        let manifest = indoc!{r#"
            identifier = "static-package-a"
            version = "0.0.1"
        "#};

        filesystem.write_all("/project/packster.toml", manifest.as_bytes())?;

        const APP_VERSION : &str = "0.1.4";

        let filesystem_as_archiver = InMemoryFileSystem::default();
        let project_workspace = Absolute::assume_absolute(PathBuf::from("/project"));
        let output_directory = Absolute::assume_absolute(PathBuf::from("/repo"));
        let request = PackRequest::new(project_workspace, output_directory);
        Operation::new(request,New)
            .parse_project(&filesystem, &Toml)?
            .generate_unique_identity(&UniqueIdentifierGeneratorMock)
            .archive(&filesystem, &filesystem_as_archiver)?
            .digest(&filesystem, &DigesterMock)?
            .finalize(&filesystem, APP_VERSION)?;

        let package_path = format!("/repo/static-package-a_0.0.1_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.{}.packster", hex::encode(APP_VERSION.as_bytes()));

        assert!(filesystem.exists(&package_path));
        assert!(filesystem.is_file(package_path));

        assert!(filesystem_as_archiver.is_file("packster.toml"));
        assert!(filesystem_as_archiver.is_file("hello_world.txt"));
        assert_eq!(filesystem_as_archiver.read_to_string("hello_world.txt")?, String::from("Hello world !"));

        Ok(())
    }

    #[test]
    fn test_init_location_initialization_case() -> Result<()> {
        let filesystem = InMemoryFileSystem::default();
        filesystem.create_dir("/my")?;

        let request = InitLocationRequest::new(Absolute::assume_absolute(PathBuf::from("/my/location")));
        Operation::new(request, New)
            .initialize_lockfile(&filesystem, &Json)?;

        let expected_lockfile_path = Path::new("/my/location").join(LOCKFILE_NAME);
        assert!(filesystem.exists(&expected_lockfile_path));

        let expected_lockfile_content = "{\"deployments\":[]}";
        assert_eq!(filesystem.read_to_string(expected_lockfile_path)?, expected_lockfile_content);

        Ok(())
    }

}
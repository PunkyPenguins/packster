#[cfg(test)]
mod test {
    use std::{io::Read, path::{PathBuf, Path}, str::FromStr };
    use indoc::indoc;
    use base64::{Engine as _, engine::general_purpose};

    use packster_core::{
        port::{
            Digester,
            ReadOnlyFileSystem,
            FileSystem,
            UniqueIdentifierGenerator,
        },
        Result,
        operation::{PackRequest, Operation, New, InitLocationRequest, DeployRequest},
        path::Absolute, LOCKFILE_NAME, domain::Checksum
    };

    use packster_infrastructure::{
        InMemoryFileSystem,
        Toml,
        Json, TarballArchiver, Sha2Digester
    };

    fn base64_decode(s: &str) -> Vec<u8> {
        general_purpose::STANDARD.decode(s).unwrap()
    }

    #[test]
    fn test_static_packing() -> Result<()> {
        pub struct DigesterMock;

        impl Digester for DigesterMock {
            fn generate_checksum<R: Read>(&self, _: R) -> Result<Checksum> {
                Ok(Checksum::from_str("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap())
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
        assert!(filesystem.is_file(&package_path));

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

    #[test]
    fn test_deployment_new_package() -> Result<()> {
        let filesystem = InMemoryFileSystem::default();
        filesystem.create_dir_recursively("/my/location")?;
        // Made out of "my-simple-package" stub
        let package_base64_str =  "H4sIAAAAAAAA/+3WsQ6CMBAG4JtNfIfKLrYiOLn7FqTqoY2FknIafXtlAQOuNjHct7RJx7//5XR+Mh6P5PwTQpO94T1TiYIUArg1pL0QMFG6z3+lc105uqDPC2MxpgfBT3WJr5NB/kptZQYSAph4/nu01onCu1IcHNH7WACbDh2o69/0/R/N/zRLt9z/AD76T67m8k9NrY/XhtDH79FvIayu8Vky2v82ivsfgjlhRaYw6MVOROVz2Ziytrhs/4U+YzSf3dE3xlXts4xlrCJgjDH2/15uvNI0ABIAAA==";
        let package_bytes = base64_decode(package_base64_str);
        filesystem.open_write("/my/my-simple-package_0.0.1_d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4.302e312e30.packster")?.write_all(&package_bytes).unwrap();

        let lockfile_path = Path::new("/my/location").join(LOCKFILE_NAME);
        let empty_lockfile_content = "{\"deployments\":[]}";
        filesystem.open_write(&lockfile_path)?.write_all(empty_lockfile_content.as_bytes()).unwrap();

        let request = DeployRequest::new(
            Absolute::assume_absolute(PathBuf::from("/my/my-simple-package_0.0.1_d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4.302e312e30.packster")),
            Absolute::assume_absolute(PathBuf::from("/my/location")),
        );

        Operation::new(request, New)
            .parse_package_path()?
            .parse_location_lockfile(&filesystem, &Json)?
            .probe_package_not_deployed_in_location()?
            .validate_package_checksum(&filesystem, &Sha2Digester::Sha256)?
            .extract_package(&filesystem, &TarballArchiver)?
            .update_location_lockfile(&filesystem, &Json)?;

        assert!(filesystem.exists("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4"));
        assert!(filesystem.is_directory("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4"));

        assert!(filesystem.exists("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/packster.toml"));
        assert!(filesystem.is_file("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/packster.toml"));

        assert!(filesystem.exists("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_file.txt"));
        assert!(filesystem.is_file("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_file.txt"));
        assert_eq!(filesystem.read_to_string("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_file.txt")?, "Hello from top !");

        assert!(filesystem.exists("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_directory"));
        assert!(filesystem.is_directory("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_directory"));

        assert!(filesystem.exists("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_directory/a_another_file.txt"));
        assert!(filesystem.is_file("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_directory/a_another_file.txt"));
        assert_eq!(filesystem.read_to_string("/my/location/d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4/a_directory/a_another_file.txt")?, "Hello from bottom !");

        let lockfile_content = filesystem.read_to_string(lockfile_path)?;
        assert_ne!(lockfile_content, empty_lockfile_content);
        assert!(lockfile_content.contains("d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4"));

        Ok(())
    }

}
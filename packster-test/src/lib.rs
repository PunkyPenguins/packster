

#[cfg(test)]
mod test {
    use std::{io::Read, fmt};
    use indoc::indoc;

    use packster_core::{
        Digester,
        ReadOnlyFileSystem,
        FileSystem,
        IdentifierGenerator,
        Result,
        operation::{PackRequest, Operation, New}
    };
    use packster_infrastructure::{
        InMemoryFileSystem,
        TomlParser
    };

    #[test]
    fn test_static_packing() -> Result<()> {
        pub struct DigesterMock;

        impl Digester for DigesterMock {
            fn generate_checksum<R: Read>(&self, _: R) -> Result<Vec<u8>> {
                Ok(hex::decode("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").unwrap())
            }
        }

        impl fmt::Display for DigesterMock {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "mock")
            }
        }

        pub struct IdentifierGeneratorMock;

        impl IdentifierGenerator for IdentifierGeneratorMock {
            fn generate_identifier<S: AsRef<str>>(&self, _name: S) -> String {
                String::from("123456")
            }
        }

        let filesystem = InMemoryFileSystem::default();
        filesystem.create_dir("project")?;
        filesystem.create_dir("repo")?;
        filesystem.write_all("project/hello_world.txt", b"Hello world !")?;
        filesystem.create("project/packster.toml")?;


        let manifest = indoc!{r#"
            identifier = "static-package-a"
            version = "0.0.1"
        "#};

        filesystem.write_all("project/packster.toml", manifest.as_bytes())?;

        let filesystem_as_archiver = InMemoryFileSystem::default();
        Operation::new(PackRequest::new("project", "repo"),New)
            .parse_project(&filesystem, &TomlParser)?
            .generate_unique_identity(&IdentifierGeneratorMock)
            .archive(&filesystem, &filesystem_as_archiver)?
            .digest(&filesystem, &DigesterMock)?
            .finalized(&filesystem)?;

        assert!(filesystem.exists("repo/static-package-a_0.0.1_mock_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.mock.packster"));
        assert!(filesystem.is_file("repo/static-package-a_0.0.1_mock_ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad.mock.packster"));

        assert!(filesystem_as_archiver.is_file("packster.toml"));
        assert!(filesystem_as_archiver.is_file("hello_world.txt"));
        assert_eq!(filesystem_as_archiver.read_to_string("hello_world.txt")?, String::from("Hello world !"));

        Ok(())
    }
}
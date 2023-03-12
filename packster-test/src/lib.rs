#[cfg(test)]
mod test {
    use std::io::Read;
    use indoc::indoc;

    use packster_core::{
        Digester,
        ReadOnlyFileSystem,
        FileSystem,
        UniqueIdentifierGenerator,
        Result,
        operation::{PackRequest, Operation, New},
        AbsolutePath
    };
    use packster_infrastructure::{
        InMemoryFileSystem,
        Toml
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
        let request = PackRequest::new(AbsolutePath::assume_absolute("/project"), AbsolutePath::assume_absolute("/repo"));
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
}
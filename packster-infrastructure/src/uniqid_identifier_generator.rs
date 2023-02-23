use packster_core::IdentifierGenerator;
use uniqueid::{IdentifierBuilder, IdentifierType};

pub struct UniqidIdentifierGenerator;

//TODO add some logging and integration tests
impl IdentifierGenerator for UniqidIdentifierGenerator {
    fn generate_identifier<S: AsRef<str>>(&self, name: S) -> String {
        let mut builder = IdentifierBuilder::default();

        builder.name(name.as_ref());
        builder.add(IdentifierType::CPU);
        builder.add(IdentifierType::RAM);
        builder.add(IdentifierType::DISK);

        let identifier = builder.build();

        identifier.to_string(true)
    }
}
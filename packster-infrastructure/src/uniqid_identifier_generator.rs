use packster_core::port::UniqueIdentifierGenerator;
use unique_id::Generator;
use unique_id::string::StringGenerator;


#[derive(Default)]
pub struct UniqidIdentifierGenerator(StringGenerator);

//TODO add some logging and integration tests
impl UniqueIdentifierGenerator for UniqidIdentifierGenerator {
    fn generate_identifier(&self) -> String {
        self.0.next_id()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_really_unique() {
        assert_ne!(
            UniqidIdentifierGenerator::default().generate_identifier(),
            UniqidIdentifierGenerator::default().generate_identifier()
        );
    }
}
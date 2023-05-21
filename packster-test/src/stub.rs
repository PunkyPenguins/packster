pub fn get_simple_package_bytes() -> Vec<u8> {
    include_bytes!("./stub/my-simple-package_0.0.1_d829752c10db8f7a98c939b5418beb0a360c6a6b818830e000f2c5a8dce35af4.302e312e30.packster.tar.gz").to_vec()
}
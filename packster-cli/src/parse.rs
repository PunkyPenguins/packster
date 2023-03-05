use std::path::Path;
use packster_core::AbsolutePath;

pub fn try_from_current_dir(path: &str) -> std::io::Result<AbsolutePath> {
    let path = Path::new(path);
    Ok(
        if path.is_relative() {
            AbsolutePath::assume_absolute(std::env::current_dir()?.join(path))
        } else {
            AbsolutePath::assume_absolute(path)
        }
    )
}
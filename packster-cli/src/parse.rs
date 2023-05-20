use std::{path::{Path, PathBuf}};
use packster_core::{path::Absolute};

pub fn try_from_current_dir(path: &str) -> std::io::Result<Absolute<PathBuf>> {
    let path = Path::new(path);
    Ok(
        if path.is_relative() {
            Absolute::assume_absolute(std::env::current_dir()?.join(path))
        } else {
            Absolute::assume_absolute(path.to_path_buf())
        }
    )
}
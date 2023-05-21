use std::path::PathBuf;

use clap::Args;
use packster_core::{operation::{DeployRequest}, path::Absolute};
use crate::parse::try_from_current_dir;

#[derive(Args)]
pub struct DeployFileCommand {
    #[arg(value_parser=try_from_current_dir)]
    pub location_directory: Absolute<PathBuf>,
    #[arg(value_parser=try_from_current_dir)]
    pub package_file: Absolute<PathBuf>,
}

impl From<DeployFileCommand> for DeployRequest {
    fn from(command: DeployFileCommand) -> DeployRequest {
        DeployRequest::new(command.package_file, command.location_directory)
    }
}
use std::path::PathBuf;

use clap::Args;
use packster_core::{operation::PackRequest, path::Absolute};
use crate::parse::try_from_current_dir;

#[derive(Args)]
pub struct PackCommand {
    #[arg(value_parser=try_from_current_dir)]
    pub project_workspace: Absolute<PathBuf>,

    #[arg(value_parser=try_from_current_dir)]
    pub package_output_directory: Absolute<PathBuf>
}

impl From<PackCommand> for PackRequest {
    fn from(pack_command: PackCommand) -> PackRequest {
        PackRequest::new(
            pack_command.project_workspace,
            pack_command.package_output_directory
        )
    }
}
use std::path::PathBuf;
use clap::Args;
use packster_core::operation::PackRequest;

#[derive(Args)]
pub struct PackCommand {
    #[arg(short='p', long)]
    pub project_workspace: PathBuf,

    #[arg(short='o', long)]
    pub package_output_directory: PathBuf
}

impl From<PackCommand> for PackRequest {
    fn from(pack_command: PackCommand) -> PackRequest {
        PackRequest::new(pack_command.project_workspace, pack_command.package_output_directory)
    }
}
use std::path::PathBuf;

use clap::Args;
use packster_core::{operation::InitLocationRequest, Absolute};
use crate::parse::try_from_current_dir;

#[derive(Args)]
pub struct InitLocationCommand {
    #[arg(short='l', long, value_parser=try_from_current_dir)]
    pub location_directory: Absolute<PathBuf>
}

impl From<InitLocationCommand> for InitLocationRequest {
    fn from(init_location_command: InitLocationCommand) -> InitLocationRequest {
        InitLocationRequest::new(init_location_command.location_directory)
    }
}
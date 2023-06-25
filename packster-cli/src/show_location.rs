use std::path::PathBuf;
use clap::Args;
use packster_core::{application::path::Absolute, packaging::application::ShowLocationRequest};
use crate::parse::try_from_current_dir;


#[derive(Args)]
pub struct ShowLocationCommand {
    #[arg(value_parser=try_from_current_dir)]
    pub location_directory: Absolute<PathBuf>,
}

impl From<ShowLocationCommand> for ShowLocationRequest {
    fn from(value: ShowLocationCommand) -> Self { ShowLocationRequest::new(value.location_directory) }
}
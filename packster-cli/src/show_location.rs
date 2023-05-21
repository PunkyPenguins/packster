use std::path::PathBuf;
use clap::Args;
use packster_core::path::Absolute;
use crate::parse::try_from_current_dir;


#[derive(Args)]
pub struct ShowLocationCommand {
    #[arg(short='l', long, value_parser=try_from_current_dir)]
    pub location_directory: Absolute<PathBuf>,
}

use std::{path::PathBuf, str::FromStr};

use clap::Args;
use packster_core::{operation::UndeployRequest, path::Absolute, domain::Checksum};
use crate::parse::try_from_current_dir;

#[derive(Args)]
pub struct UndeployCommand {
    #[arg(value_parser=try_from_current_dir)]
    pub location_directory: Absolute<PathBuf>,
    #[arg()]
    pub checksum: String,
}

impl From<UndeployCommand> for UndeployRequest {
    fn from(command: UndeployCommand) -> UndeployRequest {
        UndeployRequest::new(
            Checksum::from_str(&command.checksum).expect("Wrong checksum hexadecimal"),
            command.location_directory
        )
    }
}
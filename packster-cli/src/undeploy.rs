use std::{path::PathBuf, str::FromStr};

use clap::{Args};
use packster_core::{Result, Error, packaging::application::UndeployRequest, domain::entity::Checksum, application::path::Absolute};
use crate::parse::try_from_current_dir;

#[derive(Args)]
pub struct UndeployCommand {
    #[arg(value_parser=try_from_current_dir)]
    pub location_directory: Absolute<PathBuf>,
    #[arg()]
    pub checksum: String,
}

impl TryFrom<UndeployCommand> for UndeployRequest {
    type Error = Error;
    fn try_from(command: UndeployCommand) -> Result<UndeployRequest> {
        Ok(
            UndeployRequest::new(
                Checksum::from_str(&command.checksum)?,
                command.location_directory
            )
        )
    }
}
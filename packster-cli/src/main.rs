use std::println;

use clap::{Parser, Subcommand};
use packster_core::{operation::*, Result};
use packster_infrastructure::{
    StdFileSystem,
    Toml,
    UniqidIdentifierGenerator,
    Sha2Digester,
    TarballArchiver,
    Json
};

mod parse;
mod pack;
mod init_location;
mod deploy_file;
mod undeploy;

pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");


fn main() {
    std::process::exit(
        match CommandLine::parse().execute() {
            Ok(_) => 0,
            Err(error) => {
                eprintln!("{error}");
                1
            }
        }
    )
}

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    #[command(subcommand)]
    command: Option<Command>,
}

impl CommandLine {
    pub fn execute(self) -> Result<()> {
        if let Some(command) = self.command {
            match command {
                Command::Pack(pack_command) =>
                    Operation::new(pack_command.into())
                        .parse_project(&StdFileSystem, &Toml)?
                        .generate_unique_identity(&UniqidIdentifierGenerator::default())
                        .archive(&StdFileSystem, &TarballArchiver)?
                        .digest(&StdFileSystem, &Sha2Digester::Sha256)?
                        .finalize(&StdFileSystem, CRATE_VERSION)
                        .map(
                            |operation|
                                println!("Package created : {}", operation.get_state().to_file_name())
                        )?
                ,
                Command::InitLocation(init_location_command) =>
                    Operation::new(init_location_command.into())
                        .initialize_lockfile(&StdFileSystem, &Json)
                        .map(
                            |op|
                            println!("Empty deployment created at : {}", op.as_path_location().to_string_lossy())
                        )?
                ,
                Command::DeployFile(deploy_file_command) =>
                    Operation::new(DeployRequest::from(deploy_file_command))
                        .parse_package_path()?
                        .parse_location_lockfile(&StdFileSystem, &Json)?
                        .probe_package_not_deployed_in_location()?
                        .validate_package_checksum(&StdFileSystem, &Sha2Digester::Sha256)?
                        .guess_deployment_path()
                        .extract_package(&StdFileSystem, &TarballArchiver)?
                        .update_location_lockfile(&StdFileSystem, &Json)
                        .map(|operation|
                            println!(
                                "Package {} deployed in {}",
                                operation.as_package().as_identifier(),
                                operation.as_package_path().to_string_lossy()
                            )
                        )?
                ,
                Command::Undeploy(undeploy_command) =>
                    Operation::new(UndeployRequest::from(undeploy_command))
                        .parse_location_lockfile(&StdFileSystem, &Json)?
                        .probe_package_already_deployed_in_location()?
                        .guess_deployment_path()
                        .update_location_lockfile(&StdFileSystem, &Json)?
                        .delete_deployment_directory(&StdFileSystem)
                        .map(|operation|
                            println!(
                                "Deployment {} undeployed from location {}",
                                operation.as_checksum().to_string(),
                                operation.as_path_location().to_string_lossy()
                            )
                        )?
            };
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Command {
    Pack(pack::PackCommand),
    InitLocation(init_location::InitLocationCommand),
    DeployFile(deploy_file::DeployFileCommand),
    Undeploy(undeploy::UndeployCommand)
}

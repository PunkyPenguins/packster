#![cfg_attr(all(not(debug_assertions), not(test)), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(clippy::all)]

use clap::{Args, Parser, Subcommand};
use packster_core::{
    application::operation::{AsChecksum, Operation},
    packaging::application::*,
    packaging::domain::*,
    Result,
};
use packster_infrastructure::{ Json, Sha2Digester, StdFileSystem, TarballArchiver, Toml, UniqidIdentifierGenerator };

mod deploy_file;
mod init_location;
mod pack;
mod parse;
mod show_location;
mod undeploy;

pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    std::process::exit(match CommandLine::parse().execute() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    })
}

#[derive(Subcommand)]
enum ProjectCommand {
    Pack(pack::PackCommand),
}

#[derive(Args)]
struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectCommand,
}

#[derive(Subcommand)]
enum LocationCommand {
    Init(init_location::InitLocationCommand),
    Undeploy(undeploy::UndeployCommand),
    Show(show_location::ShowLocationCommand),
}

#[derive(Args)]
struct LocationArgs {
    #[command(subcommand)]
    command: LocationCommand,
}

#[derive(Subcommand)]
enum PackageCommand {
    Deploy(deploy_file::DeployFileCommand),
}

#[derive(Args)]
struct PackageArgs {
    #[command(subcommand)]
    command: PackageCommand,
}

#[derive(Subcommand)]
enum Scope {
    Project(ProjectArgs),
    Location(LocationArgs),
    Package(PackageArgs),
}

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    #[command(subcommand)]
    scope: Scope,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    CommandLine::command().debug_assert()
}

impl CommandLine {
    pub fn execute(self) -> Result<()> {
        match self.scope {
            Scope::Project(ProjectArgs { command }) => match command {
                ProjectCommand::Pack(pack_command) => Operation::new(pack_command.into())
                    .parse_project(&StdFileSystem, &Toml)?
                    .generate_unique_identity(&UniqidIdentifierGenerator::default())
                    .archive(&StdFileSystem, &TarballArchiver)?
                    .digest(&StdFileSystem, &Sha2Digester::Sha256)?
                    .finalize(&StdFileSystem, CRATE_VERSION)
                    .map(|operation| {
                        println!("Package created : {}", operation.as_state().to_file_name())
                    })?,
            },
            Scope::Location(LocationArgs { command }) => match command {
                LocationCommand::Init(init_location_command) => {
                    Operation::new(init_location_command.into())
                        .initialize_lockfile(&StdFileSystem, &Json)
                        .map(|op| {
                            println!(
                                "Empty deployment created at : {}",
                                op.as_location_path().to_string_lossy()
                            )
                        })?
                }
                LocationCommand::Show(show_location_command) => {
                    Operation::new(ShowLocationRequest::from(show_location_command))
                        .parse_location_lockfile(&StdFileSystem, &Json)
                        .map(|operation| {
                            if operation.as_location().iter().next().is_some() {
                                operation.as_location().iter().for_each(|deployment| {
                                    let package = deployment.as_ref();
                                    println!(
                                        "{} {} {}",
                                        package.as_identifier(),
                                        package.as_version(),
                                        package.as_checksum().to_string()
                                    )
                                })
                            } else {
                                print!("Location contains no deployments")
                            }
                        })?
                }
                LocationCommand::Undeploy(undeploy_command) => {
                    Operation::new(UndeployRequest::try_from(undeploy_command)?)
                        .parse_location_lockfile(&StdFileSystem, &Json)?
                        .probe_package_already_deployed_in_location()?
                        .guess_deployment_path()
                        .remove_deployment_from_location()
                        .persist_location_lockfile(&StdFileSystem, &Json)?
                        .delete_deployment_directory(&StdFileSystem)
                        .map(|operation| {
                            println!(
                                "Deployment {} undeployed from location {}",
                                operation.as_checksum().to_string(),
                                operation.as_location_path().to_string_lossy()
                            )
                        })?
                }
            },
            Scope::Package(PackageArgs { command }) => match command {
                PackageCommand::Deploy(deploy_file_command) => {
                    Operation::new(DeployRequest::from(deploy_file_command))
                        .parse_package_path()?
                        .parse_location_lockfile(&StdFileSystem, &Json)?
                        .probe_package_not_deployed_in_location()?
                        .validate_package_checksum(&StdFileSystem, &Sha2Digester::Sha256)?
                        .guess_deployment_path()
                        .extract_package(&StdFileSystem, &TarballArchiver)?
                        .add_deployment_to_location()
                        .persist_location_lockfile(&StdFileSystem, &Json)
                        .map(|operation| {
                            println!(
                                "Package {} deployed in {}",
                                operation.as_package().as_identifier(),
                                operation.as_package_path().to_string_lossy()
                            )
                        })?
                }
            },
        };

        Ok(())
    }
}

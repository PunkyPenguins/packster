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
                    Operation::new(pack_command.into(), New)
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
                    Operation::new(init_location_command.into(), New)
                        .initialize_lockfile(&StdFileSystem, &Json)
                        .map(
                            |op|
                            println!("Empty deployment created at : {}", op.get_request().as_path_location().as_ref().to_string_lossy())
                        )?,
                Command::DeployFile(deploy_file_command) =>
                    Operation::new(DeployRequest::from(deploy_file_command), New)
                        .parse_package_path()?
                        .parse_location_lockfile(&StdFileSystem, &Json)?
                        .probe_package_not_deployed_in_location()?
                        .validate_package_checksum(&StdFileSystem, &Sha2Digester::Sha256)?
                        .extract_package(&StdFileSystem, &TarballArchiver)?
                        .update_location_lockfile(&StdFileSystem, &Json)
                        .map(|operation| {
                            let state = operation.get_state();
                            println!("Package {} deployed in {}", state.package.as_identifier(), state.deploy_path.as_ref().to_string_lossy())
                        })?
            };
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Command {
    Pack(pack::PackCommand),
    InitLocation(init_location::InitLocationCommand),
    DeployFile(deploy_file::DeployFileCommand)
}

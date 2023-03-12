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
                Command::Pack(pack_command) => Operation::new(pack_command.into(), New)
                    .parse_project(&StdFileSystem, &Toml)?
                    .generate_unique_identity(&UniqidIdentifierGenerator::default())
                    .archive(&StdFileSystem, &TarballArchiver)?
                    .digest(&StdFileSystem, &Sha2Digester::Sha256)?
                    .finalize(&StdFileSystem, CRATE_VERSION)
                    .map(
                        |package|
                            println!("Package created : {}", package.get_state().file_name())
                    )?
                ,
                Command::InitLocation(init_location) => Operation::new(init_location.into(), New)
                    .initialize_lockfile(&StdFileSystem, &Json)
                    .map(
                        |op|
                        println!("Empty deployment created at : {}", op.get_request().as_location_directory().as_ref().to_string_lossy())
                    )?
            };
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Command {
    Pack(pack::PackCommand),
    InitLocation(init_location::InitLocationCommand)
}

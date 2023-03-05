use clap::{Parser, Subcommand};
use packster_core::{operation::*, Result};
use packster_infrastructure::{
    StdFileSystem,
    TomlParser,
    UniqidIdentifierGenerator,
    Sha2Digester,
    TarballArchiver
};

mod pack;
mod parse;

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
                    .parse_project(&StdFileSystem, &TomlParser)?
                    .generate_unique_identity(&UniqidIdentifierGenerator)
                    .archive(&StdFileSystem, &TarballArchiver)?
                    .digest(&StdFileSystem, &Sha2Digester::Sha256)?
                    .finalize(&StdFileSystem, CRATE_VERSION)?
            };
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Command {
    Pack(pack::PackCommand)
}

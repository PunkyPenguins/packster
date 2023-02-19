use clap::{Parser, Subcommand};
use packster_core::operation::*;
use packster_infrastructure::{
    StdFileSystem,
    TomlParser,
    UniqidIdentifierGenerator,
    Sha2Digester,
    TarballArchiver
};

mod pack;

fn main() {
    std::process::exit(CommandLine::parse().execute())
}

#[derive(Parser)]
#[command(author, version, about)]
struct CommandLine {
    #[command(subcommand)]
    command: Option<Command>,
}

impl CommandLine {
    pub fn execute(self) -> i32 {
        if let Some(command) = self.command {
            match command {
                Command::Pack(pack_command) => Operation::new(pack_command.into(), New)
                    .parse_project(&StdFileSystem, &TomlParser).unwrap() //TODO handle errors here
                    .generate_unique_identity(&UniqidIdentifierGenerator)
                    .archive(&StdFileSystem, &TarballArchiver).unwrap()
                    .digest(&StdFileSystem, &Sha2Digester::Sha256).unwrap()
                    .finalized(&StdFileSystem).unwrap()
            };
        }

        0
    }
}

#[derive(Subcommand)]
enum Command {
    Pack(pack::PackCommand)
}

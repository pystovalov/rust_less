mod cli;
mod docker;
use clap::Parser;
use cli::{Cli, Command, ListCommand};

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::List { list_command } => match list_command {
            ListCommand::Containers { all } => {
                if all {
                    println!("Listing all conteiner...");
                } else {
                    println!("Listing running containers")
                }
            }
        },
    }
}

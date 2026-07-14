use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mydocker")]
#[command(about = "A minimal Docker CLI in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    List {
        #[command(subcommand)]
        list_command: ListCommand,
    },
}

#[derive(Subcommand)]
pub enum ListCommand {
    Containers {
        #[arg(short, long)]
        all: bool,
    },
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "greeter")]
#[command(about="A simple CLI tool to greet a person",long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Hello {
        #[arg(short, long)]
        name: String,
    },
    Goodbye {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();
    match args.command {
        Command::Hello { name } => {
            println!("Hello, {}", name);
        }
        Command::Goodbye { name } => {
            println!("Goodbye, {}", name);
        }
    }
}

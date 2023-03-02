use clap::{Parser, Subcommand};
use mutils::dice::roll;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Roll {
        expression: String
    }
}

impl Command {
    fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            Command::Roll { expression } =>{
                let result = roll(expression)?;

                println!("Dice Roll: {result}")
            }
        }

        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command.execute() {
        Ok(_) => {},
        Err(e) => println!("Error: {e}"),
    }
}



use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds two numbers
    Add {
        /// The first number
        a: i32,
        /// The second number
        b: i32,
    },
    /// Subtracts two numbers
    Sub {
        /// The first number
        a: i32,
        /// The second number
        b: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { a, b } => {
            println!("{} + {} = {}", a, b, a + b);
        }
        Commands::Sub { a, b } => {
            println!("{} - {} = {}", a, b, a - b);
        }
    }
}

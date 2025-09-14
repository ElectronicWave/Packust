use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
}

fn main() {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Init {}) => {
            println!("Initializing...");
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}


mod vault;
mod cli;

use crate::cli::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { service, username, password} => {
            println!("add {}, {}, {:?}", service, username, password);
        }
        Commands::Get {service } => {
            println!("get {}", service);
        }
        Commands::List => {
            println!("used list arg");
        }
        Commands::Delete {service } => {
            println!("delete {}", service);
        }
    }
}

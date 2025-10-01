use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        service: String,
    
        #[arg(short, long)]
        username: String,

        #[arg(short, long)]
        password: Option<String> //promt later if not included
    },
    Get {
        service: String,
    },
    List,
    Delete {
        service: String,
    }

}

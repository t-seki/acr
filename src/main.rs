mod atcoder;
mod cli;
mod config;
mod error;
mod runner;
mod types;
mod workspace;

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            todo!("acrs init")
        }
        Command::Login => {
            todo!("acrs login")
        }
        Command::Logout => {
            todo!("acrs logout")
        }
        Command::Session => {
            todo!("acrs session")
        }
        Command::New { contest_id } => {
            todo!("acrs new {}", contest_id)
        }
        Command::Add { problem } => {
            todo!("acrs add {}", problem)
        }
        Command::Test => {
            todo!("acrs test")
        }
        Command::Submit => {
            todo!("acrs submit")
        }
        Command::Config { key, value } => {
            todo!("acrs config")
        }
    }
}

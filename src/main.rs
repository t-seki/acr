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
            use std::io::{self, Write};
            print!("Username: ");
            io::stdout().flush()?;
            let mut username = String::new();
            io::stdin().read_line(&mut username)?;
            let username = username.trim();
            print!("Password: ");
            io::stdout().flush()?;
            let password = rpassword::read_password()?;
            atcoder::auth::login(username, &password).await?;
            println!("ログインに成功しました。");
        }
        Command::Logout => {
            atcoder::auth::logout()?;
            println!("ログアウトしました。");
        }
        Command::Session => {
            if atcoder::auth::check_session().await? {
                println!("ログイン済みです。");
            } else {
                println!("ログインしていません。");
            }
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
        Command::Config { key: _, value: _ } => {
            todo!("acrs config")
        }
    }

    Ok(())
}

mod atcoder;
mod cli;
mod config;
mod error;
mod runner;
mod workspace;

use atcoder::AtCoderClient;
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
            print!("Username: ");
            let mut username = String::new();
            std::io::Write::flush(&mut std::io::stdout())?;
            std::io::stdin().read_line(&mut username)?;
            let username = username.trim();

            eprint!("Password: ");
            let password = rpassword::read_password()?;

            let client = AtCoderClient::new()?;
            let revel_session = client.login(username, &password).await?;

            config::session::save(&config::session::SessionConfig { revel_session })?;
            println!("Logged in as {}.", username);
            Ok(())
        }
        Command::Logout => {
            config::session::delete()?;
            println!("Logged out.");
            Ok(())
        }
        Command::Session => {
            let session = config::session::load()?;
            let client = AtCoderClient::with_session(&session.revel_session)?;
            match client.check_session().await? {
                Some(username) => println!("Logged in as {}.", username),
                None => println!("Session expired. Run `acrs login` again."),
            }
            Ok(())
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
}

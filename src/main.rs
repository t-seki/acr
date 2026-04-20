mod atcoder;
mod browser;
mod cli;
mod commands;
mod config;
mod error;
mod launcher;
mod runner;
mod workspace;

use clap::Parser;
use cli::{Cli, Command};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut cli = Cli::parse();
    cli.command.normalize();

    match cli.command {
        Command::Init => commands::init::execute(),
        Command::Login => commands::session::login().await,
        Command::Logout => commands::session::logout(),
        Command::Session => commands::session::check().await,
        Command::New {
            contest_id,
            problems,
            at,
        } => commands::new::execute(contest_id, problems, at).await,
        Command::Add { problems } => commands::add::execute(problems).await,
        Command::Update {
            args,
            tests,
            code,
            deps,
        } => commands::update::execute(args, tests, code, deps).await,
        Command::View { args } => commands::view::execute(args),
        Command::Test { problem } => commands::test::execute(problem).await,
        Command::Submit { problem, force } => commands::submit::execute(problem, force).await,
        Command::Virtual {
            contest_id,
            problems,
            at,
        } => commands::virtual_contest::execute(contest_id, problems, at).await,
        Command::Submissions { contest_id } => commands::submissions::execute(contest_id),
        Command::Config { key, value } => commands::config::execute(key, value),
    }
}

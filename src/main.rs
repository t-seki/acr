mod atcoder;
mod cli;
mod config;
mod error;
mod runner;
mod types;
mod workspace;

use anyhow::Context;
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
            use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

            // セッション確認
            let session = config::Session::load()?;
            let client = atcoder::client::AtCoderClient::with_session(&session.revel_session)?;

            // 問題一覧取得
            println!("コンテスト {} の問題一覧を取得中...", contest_id);
            let contest_info =
                atcoder::contest::fetch_problems(&client, &contest_id).await?;

            // ワークスペース作成
            let current_dir = std::env::current_dir()?;
            let contest_dir = workspace::generator::create_contest_workspace(
                &current_dir,
                &contest_id,
                &contest_info.problems,
            )?;

            // テストケースを並列取得
            let mp = MultiProgress::new();
            let style = ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")?;

            let mut handles = Vec::new();
            for problem in &contest_info.problems {
                let pb = mp.add(ProgressBar::new_spinner());
                pb.set_style(style.clone());
                pb.set_message(format!(
                    "問題 {} のテストケースを取得中...",
                    problem.alphabet
                ));

                let client = client.clone();
                let contest_id = contest_id.clone();
                let task_screen_name = problem.task_screen_name.clone();
                let alphabet = problem.alphabet.clone();
                let tests_dir = contest_dir.join(alphabet.to_lowercase()).join("tests");

                handles.push(tokio::spawn(async move {
                    let test_cases = atcoder::scraper::fetch_test_cases(
                        &client,
                        &contest_id,
                        &task_screen_name,
                    )
                    .await?;
                    workspace::testcase::save_test_cases(&tests_dir, &test_cases)?;
                    pb.finish_with_message(format!(
                        "問題 {}: {}件のテストケースを取得",
                        alphabet,
                        test_cases.len()
                    ));
                    Ok::<_, anyhow::Error>(())
                }));
            }

            for handle in handles {
                handle.await??;
            }

            // エディタ起動
            let config = config::GlobalConfig::load()?;
            let _ = std::process::Command::new(&config.editor)
                .arg(&contest_dir)
                .status();

            println!(
                "ワークスペースを作成しました: {}",
                contest_dir.display()
            );
        }
        Command::Add { problem } => {
            use indicatif::{ProgressBar, ProgressStyle};

            let current_dir = std::env::current_dir()?;
            // コンテストディレクトリの Cargo.toml からcontest_idを取得
            let ws_toml_path = current_dir.join("Cargo.toml");
            anyhow::ensure!(
                ws_toml_path.exists(),
                "コンテストディレクトリから実行してください。"
            );
            let ws_content = std::fs::read_to_string(&ws_toml_path)?;
            let ws_doc: toml::Table = toml::from_str(&ws_content)?;
            anyhow::ensure!(
                ws_doc.get("workspace").is_some(),
                "コンテストディレクトリから実行してください。"
            );

            // contest_id はディレクトリ名から推定
            let contest_id = current_dir
                .file_name()
                .context("ディレクトリ名の取得に失敗")?
                .to_string_lossy()
                .to_string();

            // セッション確認
            let session = config::Session::load()?;
            let client = atcoder::client::AtCoderClient::with_session(&session.revel_session)?;

            // 問題一覧から該当問題を探す
            let contest_info =
                atcoder::contest::fetch_problems(&client, &contest_id).await?;
            let target = contest_info
                .problems
                .iter()
                .find(|p| p.alphabet.to_lowercase() == problem.to_lowercase())
                .with_context(|| format!("問題 '{}' が見つかりません", problem))?;

            // 問題ディレクトリ作成
            let template = config::load_template()?;
            let problem_dir = workspace::generator::create_problem_dir(
                &current_dir,
                &contest_id,
                target,
                &template,
            )?;
            workspace::generator::add_member_to_workspace(
                &current_dir,
                &problem.to_lowercase(),
            )?;

            // テストケース取得
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")?,
            );
            pb.set_message(format!(
                "問題 {} のテストケースを取得中...",
                target.alphabet
            ));

            let test_cases = atcoder::scraper::fetch_test_cases(
                &client,
                &contest_id,
                &target.task_screen_name,
            )
            .await?;
            workspace::testcase::save_test_cases(
                &problem_dir.join("tests"),
                &test_cases,
            )?;

            pb.finish_with_message(format!(
                "問題 {}: {}件のテストケースを取得",
                target.alphabet,
                test_cases.len()
            ));

            println!(
                "問題 {} を追加しました: {}",
                problem,
                problem_dir.display()
            );
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

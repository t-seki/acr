use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "acr", about = "AtCoder CLI tool for Rust", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initial setup (interactive)
    Init,

    /// Login to AtCoder
    Login,

    /// Logout from AtCoder
    Logout,

    /// Check login status
    Session,

    /// Create contest workspace and open editor
    #[command(alias = "n")]
    New {
        /// Contest ID (e.g. abc001)
        contest_id: String,
        /// Problem identifiers (e.g. a b c). If omitted, all problems are set up.
        problems: Vec<String>,
    },

    /// Add a problem to the contest workspace
    Add {
        /// Problem identifiers (e.g. a b c). If omitted, all missing problems are added.
        problems: Vec<String>,
    },

    /// Update test cases and/or source code
    #[command(alias = "u")]
    Update {
        /// Contest ID (e.g. abc123) or problem identifier (e.g. a)
        target: Option<String>,
        /// Problem identifier when target is a contest ID (e.g. a)
        problem: Option<String>,
        /// Re-fetch sample test cases from AtCoder (default if no flags given)
        #[arg(short, long)]
        tests: bool,
        /// Regenerate src/main.rs from template
        #[arg(short, long)]
        code: bool,
        /// Update [dependencies] in Cargo.toml to the latest built-in list
        #[arg(short, long)]
        deps: bool,
    },

    /// Open problem page in browser
    ///
    /// From a problem directory: opens the problem page.
    ///
    /// From a contest directory: opens the task list,
    /// or a specific problem with PROBLEM arg.
    #[command(alias = "v")]
    View {
        /// Problem identifier (e.g. a, b, c)
        problem: Option<String>,
    },

    /// Run tests for the current problem
    #[command(alias = "t")]
    Test {
        /// Problem identifier (e.g. a, b, c)
        problem: Option<String>,
    },

    /// Test and submit the current problem
    #[command(alias = "s")]
    Submit {
        /// Problem identifier (e.g. a, b, c)
        problem: Option<String>,
        /// Submit even if tests fail
        #[arg(short, long)]
        force: bool,
    },

    /// Open my submissions page in browser
    Submissions {
        /// Contest ID (e.g. abc123). If omitted, detected from current directory.
        contest_id: Option<String>,
    },

    /// View or modify configuration
    Config {
        /// Configuration key
        key: Option<String>,
        /// Configuration value
        value: Option<String>,
    },
}

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
    },

    /// Add a problem to the contest workspace
    Add {
        /// Problem identifier (e.g. a, b, c)
        problem: String,
    },

    /// Fetch or re-fetch sample test cases
    Fetch {
        /// Problem identifier (e.g. a, b, c)
        problem: Option<String>,
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
    Submissions,

    /// View or modify configuration
    Config {
        /// Configuration key
        key: Option<String>,
        /// Configuration value
        value: Option<String>,
    },
}

use std::mem;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "acr", about = "AtCoder CLI tool for Rust", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, PartialEq)]
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
        /// Wait until the specified time to start (e.g. 21:00)
        #[arg(long, value_name = "HH:MM")]
        at: Option<String>,
    },

    /// Add a problem to the contest workspace
    Add {
        /// Problem identifiers (e.g. a b c). If omitted, all missing problems are added.
        problems: Vec<String>,
    },

    /// Update test cases and/or source code
    ///
    /// From a problem directory: updates the current problem.
    ///
    /// From a contest directory: updates all problems,
    /// or specific problems with ARGS (e.g. a b c).
    ///
    /// From outside: specify contest ID as first arg,
    /// optionally followed by problem names (e.g. abc123 a b).
    #[command(alias = "u")]
    Update {
        /// Problem names (in contest dir) or contest_id [problems...] (outside)
        args: Vec<String>,
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
    ///
    /// From outside: specify contest ID as first arg,
    /// optionally followed by a problem name (e.g. abc123 a).
    #[command(alias = "v")]
    View {
        /// Problem name (in contest dir) or contest_id [problem] (outside)
        args: Vec<String>,
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
        /// Open the submit page in browser instead of POSTing directly
        #[arg(long)]
        web: bool,
        /// Override the AtCoder language ID for this submission
        #[arg(long, value_name = "ID")]
        language_id: Option<String>,
    },

    /// Start a virtual contest participation
    Virtual {
        /// Contest ID (e.g. abc420)
        contest_id: String,
        /// Problem identifiers (e.g. a b c). If omitted, all problems are set up.
        problems: Vec<String>,
        /// Start time (e.g. 07:57). If omitted, auto-calculated (~2 min from now).
        #[arg(long, value_name = "HH:MM")]
        at: Option<String>,
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

fn strip_trailing_slash_mut(s: &mut String) {
    while s.ends_with('/') {
        s.pop();
    }
}

fn expand_slash_args(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .flat_map(|arg| {
            arg.trim_end_matches('/')
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

impl Command {
    /// Normalize CLI arguments: strip trailing slashes and expand path-style args.
    pub fn normalize(&mut self) {
        match self {
            Command::View { args } | Command::Update { args, .. } => {
                *args = expand_slash_args(mem::take(args));
            }
            Command::Add { problems } => {
                *problems = expand_slash_args(mem::take(problems));
            }
            Command::Test { problem } | Command::Submit { problem, .. } => {
                if let Some(p) = problem {
                    strip_trailing_slash_mut(p);
                }
            }
            Command::Submissions { contest_id: Some(id) } => {
                strip_trailing_slash_mut(id);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_slash_args_path_format() {
        assert_eq!(
            expand_slash_args(vec!["abc001/a/".to_string()]),
            vec!["abc001", "a"]
        );
    }

    #[test]
    fn test_expand_slash_args_trailing_slash() {
        assert_eq!(
            expand_slash_args(vec!["abc001/".to_string()]),
            vec!["abc001"]
        );
    }

    #[test]
    fn test_expand_slash_args_no_slash() {
        assert_eq!(
            expand_slash_args(vec!["abc001".to_string(), "a".to_string()]),
            vec!["abc001", "a"]
        );
    }

    #[test]
    fn test_expand_slash_args_mixed() {
        assert_eq!(
            expand_slash_args(vec!["abc001/a".to_string(), "b".to_string()]),
            vec!["abc001", "a", "b"]
        );
    }

    #[test]
    fn test_expand_slash_args_empty() {
        assert_eq!(expand_slash_args(vec![]), Vec::<String>::new());
    }

    #[test]
    fn test_strip_trailing_slash_mut_single() {
        let mut s = "abc001/".to_string();
        strip_trailing_slash_mut(&mut s);
        assert_eq!(s, "abc001");
    }

    #[test]
    fn test_strip_trailing_slash_mut_multiple() {
        let mut s = "abc001//".to_string();
        strip_trailing_slash_mut(&mut s);
        assert_eq!(s, "abc001");
    }

    #[test]
    fn test_strip_trailing_slash_mut_no_slash() {
        let mut s = "abc001".to_string();
        strip_trailing_slash_mut(&mut s);
        assert_eq!(s, "abc001");
    }

    #[test]
    fn test_strip_trailing_slash_mut_empty() {
        let mut s = String::new();
        strip_trailing_slash_mut(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_normalize_view() {
        let mut cmd = Command::View {
            args: vec!["abc001/a/".to_string()],
        };
        cmd.normalize();
        assert_eq!(
            cmd,
            Command::View {
                args: vec!["abc001".to_string(), "a".to_string()]
            }
        );
    }

    #[test]
    fn test_normalize_update() {
        let mut cmd = Command::Update {
            args: vec!["abc001/a/".to_string()],
            tests: false,
            code: false,
            deps: false,
        };
        cmd.normalize();
        assert_eq!(
            cmd,
            Command::Update {
                args: vec!["abc001".to_string(), "a".to_string()],
                tests: false,
                code: false,
                deps: false,
            }
        );
    }

    #[test]
    fn test_normalize_add() {
        let mut cmd = Command::Add {
            problems: vec!["a/".to_string(), "b/".to_string()],
        };
        cmd.normalize();
        assert_eq!(
            cmd,
            Command::Add {
                problems: vec!["a".to_string(), "b".to_string()]
            }
        );
    }

    #[test]
    fn test_normalize_test() {
        let mut cmd = Command::Test {
            problem: Some("a/".to_string()),
        };
        cmd.normalize();
        assert_eq!(
            cmd,
            Command::Test {
                problem: Some("a".to_string())
            }
        );
    }

    #[test]
    fn test_normalize_submit() {
        let mut cmd = Command::Submit {
            problem: Some("a/".to_string()),
            force: false,
            web: false,
            language_id: None,
        };
        cmd.normalize();
        assert_eq!(
            cmd,
            Command::Submit {
                problem: Some("a".to_string()),
                force: false,
                web: false,
                language_id: None,
            }
        );
    }

    #[test]
    fn test_normalize_submissions() {
        let mut cmd = Command::Submissions {
            contest_id: Some("abc001/".to_string()),
        };
        cmd.normalize();
        assert_eq!(
            cmd,
            Command::Submissions {
                contest_id: Some("abc001".to_string())
            }
        );
    }

    #[test]
    fn test_normalize_test_none() {
        let mut cmd = Command::Test { problem: None };
        cmd.normalize();
        assert_eq!(cmd, Command::Test { problem: None });
    }
}

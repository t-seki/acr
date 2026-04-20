use std::path::Path;

use crate::browser;
use crate::config;
use crate::launcher::parse_command_spec;

/// Open the editor on a workspace directory and launch the configured browser
/// at a problem page.
///
/// The argument layout matches the behavior that `acr new` has always produced:
/// `<editor> <user args> <workspace_dir> [<problem_main_rs>]`. Both the browser
/// URL and the focused source file are optional so that callers can use this
/// helper even when a workspace has no problems registered yet.
pub fn launch_workspace(
    workspace_dir: &Path,
    problem_url: Option<&str>,
    problem_main_rs: Option<&Path>,
) {
    if let Some(url) = problem_url {
        browser::open(url);
    }

    let editor_spec = config::global::load()
        .map(|c| c.editor)
        .unwrap_or_else(|_| "vim".to_string());
    let (program, user_args) = parse_command_spec(&editor_spec).unwrap_or_else(|| {
        if !editor_spec.trim().is_empty() {
            eprintln!(
                "acr: could not parse editor config '{}', falling back to vim",
                editor_spec
            );
        }
        ("vim".to_string(), Vec::new())
    });

    let mut cmd = std::process::Command::new(&program);
    cmd.args(&user_args);
    cmd.arg(workspace_dir);
    if let Some(path) = problem_main_rs {
        cmd.arg(path);
    }
    let _ = cmd.spawn();
}

# acr

AtCoder competitive programming CLI tool for Rust.

## Install

Prebuilt binaries (fastest, no Rust toolchain required):

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/t-seki/acr/releases/latest/download/acr-cli-installer.sh | sh

# Windows (PowerShell)
irm https://github.com/t-seki/acr/releases/latest/download/acr-cli-installer.ps1 | iex

# Homebrew (macOS / Linux)
brew install t-seki/acr/acr-cli
```

Via Cargo:

```bash
# If you already have cargo-binstall (fetches the prebuilt binary)
cargo binstall acr-cli

# From source (compiles locally)
cargo install acr-cli
```

## Setup

```bash
acr init      # Interactive setup (editor, browser, template)
acr login     # Login to AtCoder
```

## Usage

```bash
acr new abc001          # Create contest workspace and open editor (alias: n)
acr new abc001 --at 21:00  # Wait until 21:00, then create workspace
acr add e               # Add a problem to the workspace
acr open abc001         # Reopen existing workspace in editor + browser (alias: o)
acr open abc001 a       # Reopen focused on problem A

# From a problem directory
acr view                # Open current problem page in browser (alias: v)
acr update              # Re-fetch sample test cases (alias: u)
acr update -c           # Regenerate src/main.rs from template
acr update -d           # Update Cargo.toml dependencies to latest built-in list
acr update -tc          # Re-fetch test cases and regenerate src/main.rs
acr test                # Run sample tests for current problem (alias: t)
acr submit              # Test and submit current problem (alias: s)
acr submit -f           # Submit even if tests fail

# From a contest directory
acr view a              # Open problem A page in browser
acr open                # Reopen current workspace (editor + first problem page)
acr open a              # Reopen focused on problem A
acr update a            # Re-fetch test cases for problem A
acr update a b c        # Re-fetch test cases for problems A, B, C
acr update a -c         # Regenerate src/main.rs for problem A
acr update              # Re-fetch test cases for all problems in the contest
acr test a              # Run tests for problem A
acr submit a            # Submit problem A
acr submissions         # Open my submissions page in browser

# From outside the contest directory
acr update abc001       # Re-fetch all test cases in abc001/
acr update abc001 a b   # Re-fetch test cases for problems A, B in abc001/
acr update abc001 -cd   # Regenerate code and deps for all problems in abc001/
acr submissions abc001  # Open submissions page for abc001

# Session management
acr session             # Check login status
acr logout              # Logout from AtCoder

# Configuration
acr config              # Show current config
acr config editor nvim  # Change editor
acr config editor "code --new-window"  # Editor may include flags
acr config browser open # Change browser (default: xdg-open)
acr config browser "google-chrome --new-window"  # Browser may include flags
acr config browser '"/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window'  # WSL2 + Chrome
```

## Workspace Structure

`acr new abc001` generates:

```
abc001/
├── Cargo.toml          # [workspace]
├── a/
│   ├── Cargo.toml
│   ├── src/main.rs     # Generated from template
│   └── tests/
│       ├── 1.in
│       └── 1.out
├── b/
│   └── ...
```

## Configuration

Config files are stored in `~/.config/acr/`:

- `config.toml` - Editor and browser settings
- `template.rs` - Source code template
- `session.json` - Login session

### Template

`acr init` creates a default template at `~/.config/acr/template.rs`:

```rust
use proconio::input;

fn main() {
    input! {
    }
}
```

Edit this file to customize the boilerplate generated for each problem.

## Releasing (maintainers)

Releases are automated with [release-please](https://github.com/googleapis/release-please-action) and crates.io [Trusted Publishing](https://crates.io/docs/trusted-publishing):

1. Merge PRs to `main` using Conventional Commits (`feat:`, `fix:`, `chore:`, `feat!:` for breaking).
2. A **Release PR** is automatically opened by release-please with the version bump and CHANGELOG updates.
3. Merging the Release PR tags `vX.Y.Z` and publishes a GitHub Release.
4. The tag push triggers `.github/workflows/publish.yml`, which uses OIDC to obtain a short-lived crates.io token and runs `cargo publish`.

One-time setup on crates.io is required before the first automated publish: visit the `acr-cli` crate settings on crates.io and register the GitHub repo `t-seki/acr` with workflow filename `publish.yml` as a trusted publisher.

## License

MIT

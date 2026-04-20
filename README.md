# acr

AtCoder competitive programming CLI tool for Rust.

## Install

```bash
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

## License

MIT

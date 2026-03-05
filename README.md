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
acr new abc001          # Create contest workspace and open editor
acr add e               # Add a problem to the workspace

# From a problem directory
acr view                # Open current problem page in browser
acr test                # Run sample tests for current problem
acr submit              # Test and submit current problem

# From anywhere in a contest workspace
acr view a              # Open problem A page in browser
acr test a              # Run tests for problem A
acr submit a            # Submit problem A

# Session management
acr session             # Check login status
acr logout              # Logout from AtCoder

# Configuration
acr config              # Show current config
acr config editor nvim  # Change editor
acr config browser open # Change browser (default: xdg-open)
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

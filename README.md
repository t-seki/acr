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

## License

MIT

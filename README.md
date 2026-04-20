# acr

AtCoder competitive programming CLI tool for Rust. One command sets up the
Cargo workspace, fetches sample inputs, drops you in your editor with the
problem page already open, and ships your solution.

<!--
Demo media (maintainer step — fill in after recording):

1) Full-desktop screencast (preferred, 30-60s)
   - Record editor + browser + terminal in one take.
   - macOS:   QuickTime Player / Kap
   - Linux:   peek / kazam / OBS
   - Windows: Xbox Game Bar / ShareX
   - Save as MP4 or WebM; drag into a GitHub issue/PR comment to upload.
     GitHub returns a `https://user-images.githubusercontent.com/.../*.mp4`
     URL. Paste it into the <video> src below.

2) Optional still shot fallback (editor + browser side-by-side) under docs/
   so viewers with video autoplay disabled still see something.

3) Optional asciinema for terminal-only moments (see the Usage section).
-->

<!-- Replace src with the GitHub user-images URL once uploaded. -->
<video src="REPLACE_ME.mp4" autoplay muted loop playsinline width="720"></video>

<!-- <img src="docs/demo.png" alt="acr in action" width="720"> -->

## Why acr?

- **One-shot setup**: `acr new abc400` creates a Cargo workspace, fetches
  every sample input, and drops you in your editor focused on problem A.
- **Prebuilt binaries for macOS / Linux / Windows**: install in seconds via
  shell script, PowerShell, Homebrew tap, or `cargo binstall` — no Rust
  toolchain required.
- **Start-time wait**: `acr new abc400 --at 21:00` blocks until the contest
  opens, then creates the workspace the moment the tasks appear.
- **Native editor + browser integration**: `acr open`, `acr view`, and
  `acr new` launch your configured editor and browser together, shell flags
  and all (`"code --new-window"`, `"firefox --new-window"`, ...).
- **Judge-parity dependencies**: generated `Cargo.toml` pins the exact crates
  and versions that AtCoder's judge uses, so "compiles locally" means
  "compiles on the judge".
- **Portable template**: `~/.config/acr/template.rs` is your snippet library;
  share and import other people's templates with `acr template add <url>`
  (planned).

> Note: AtCoder recently introduced Cloudflare Turnstile, which makes CLI-only
> login and submission infeasible for any tool right now. `acr` uses a pasted
> `REVEL_SESSION` cookie for reads and hands the final submit off to your
> browser.

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

1. **Install** — see above. Verify with `acr --version`.
2. **Initialize the config** — `acr init` walks you through:
   - Editor command (e.g. `vim`, `nvim`, `code --new-window`)
   - Browser command (e.g. `xdg-open`, `google-chrome --new-window`)
   - Default source template (created at `~/.config/acr/template.rs`)

   Re-run `acr init` anytime; or edit `~/.config/acr/config.toml` directly.
3. **Log in to AtCoder** — `acr login` opens the AtCoder login page in
   your browser and waits for you to paste the `REVEL_SESSION` cookie
   value. Grab it from DevTools → Application → Cookies → atcoder.jp.
   Cookies typically last around a month; re-run when they expire.

> Why paste a cookie manually? AtCoder's Cloudflare Turnstile blocks
> automated form-login for CLI tools. Copying the cookie once via
> DevTools → Application → Cookies → `REVEL_SESSION` is a one-minute step.

## Usage

<!-- asciinema cast for terminal-only moments (test/update/submit).
     Record with `asciinema rec demo.cast --idle-time-limit=1`, upload via
     `asciinema upload demo.cast`, and replace the URL below. -->
<!-- [![asciicast](https://asciinema.org/a/REPLACE_ME.svg)](https://asciinema.org/a/REPLACE_ME) -->

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

## Libraries

Every problem `Cargo.toml` that `acr new` generates pins the **same crate set
and versions that AtCoder's judge provides**. Commonly reached-for crates
work out of the box:

```rust
use proconio::input;
use ac_library::ModInt998244353 as Mint;

fn main() {
    input! { n: usize, a: [Mint; n] }
    // ...
}
```

The full list lives in `src/workspace/generator.rs` and includes
`ac-library-rs`, `proconio`, `itertools`, `num`, `nalgebra`, `ndarray`,
`petgraph`, `rustc-hash`, and others.

When the judge updates its crate set, `acr update -d` refreshes the pinned
dependencies for an existing workspace.

> Adding a crate that is not in the judge's list is possible locally (edit
> `Cargo.toml` directly), but **the judge will fail to compile the submission**
> and `acr update -d` will overwrite your local edits.

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

# acr

[English](README.md) | 日本語

Rust 向け AtCoder 競技プログラミング CLI ツール。コマンド一発で Cargo
ワークスペースのセットアップとサンプル入力の取得を行い、問題ページを開いた
状態でエディタを起動し、解答の提出までこなします。

<!--
Demo media (maintainer step — fill in after recording):

1) Full-desktop screencast (preferred, 30-60s)
   - Record editor + browser + terminal in one take.
   - macOS:   QuickTime Player / Kap
   - Linux:   peek / kazam / OBS
   - Windows: Xbox Game Bar / ShareX
   - Save as MP4 or WebM; drag into a GitHub issue/PR comment to upload.
     GitHub returns a `https://user-images.githubusercontent.com/.../*.mp4`
     URL. Paste it into the <video> src below and uncomment the tag.

2) Optional still shot (editor + browser side-by-side) under docs/ so
   viewers with video autoplay disabled still see something.

3) Optional asciinema cast for terminal-only moments (see the Usage
   section).
-->

<!-- Uncomment and fill in once recorded:
<video src="REPLACE_ME.mp4" autoplay muted loop playsinline width="720"></video>
-->

## acr を使う理由

- **ワンショットセットアップ**: `acr new abc400` で Cargo ワークスペースを
  作成し、全問題のサンプル入力を取得して、A 問題にフォーカスした状態で
  エディタを起動します。
- **macOS / Linux / Windows 向けビルド済みバイナリ**: シェルスクリプト、
  PowerShell、Homebrew tap、`cargo binstall` で数秒でインストール —
  Rust ツールチェイン不要。
- **開始時刻待機**: `acr new abc400 --at 21:00` はコンテスト開始まで待機し、
  問題が公開された瞬間にワークスペースを作成します。
  `acr virtual abc300` は同じ仕組みでバーチャルコンテストに対応 —
  過去の任意のコンテストを選んで、自分だけの計時付き参加ができます。
- **エディタ + ブラウザのネイティブ連携**: `acr open`、`acr view`、
  `acr new` は設定したエディタとブラウザをシェルフラグ込みで同時に起動
  します (`"code --new-window"`、`"firefox --new-window"` など)。
- **ジャッジと同一の依存関係**: 生成される `Cargo.toml` は AtCoder の
  ジャッジが使うクレートとバージョンを正確に固定するので、「ローカルで
  コンパイルが通る」=「ジャッジでもコンパイルが通る」になります。
- **ポータブルで共有可能なテンプレート**: `~/.config/acr/template.rs` が
  あなたのスニペットライブラリです。`acr template add <url>` で誰かの
  Gist やリポジトリのファイルを取り込めます。自動バックアップ付きなので
  安心して試せます。

> 補足: AtCoder は最近 Cloudflare Turnstile を導入したため、現時点では
> どのツールでも CLI のみでのログイン・提出は実現不可能です。`acr` は
> 読み取りには貼り付けた `REVEL_SESSION` クッキーを使い、最終的な提出は
> ブラウザに引き継ぎます。

## インストール

ビルド済みバイナリ (最速、Rust ツールチェイン不要):

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/t-seki/acr/releases/latest/download/acr-cli-installer.sh | sh

# Windows (PowerShell)
irm https://github.com/t-seki/acr/releases/latest/download/acr-cli-installer.ps1 | iex

# Homebrew (macOS / Linux)
brew install t-seki/acr/acr-cli
```

Cargo 経由:

```bash
# cargo-binstall がある場合 (ビルド済みバイナリを取得)
cargo binstall acr-cli

# ソースから (ローカルでコンパイル)
cargo install acr-cli
```

## セットアップ

1. **インストール** — 上記参照。`acr --version` で確認してください。
2. **設定の初期化** — 作業ルート (例: `~/dev/atcoder-rs/`) で `acr init`
   を一度実行します。対話形式で以下を設定します:
   - エディタコマンド (例: `vim`、`nvim`、`code --new-window`)
   - ブラウザコマンド (デフォルトは macOS では `open`、Windows では
     `explorer`、それ以外では `xdg-open`。`--new-window` などのフラグを
     付けたい場合は上書きしてください)

   あわせてデフォルトのソーステンプレートを `~/.config/acr/template.rs`
   に生成し、カレントディレクトリに `.acr` マーカーファイルを作成します。
   `acr new` や `acr update` などのコマンドはディレクトリツリーのどこかに
   このマーカーが存在することを要求します (祖先ディレクトリを上方向に
   探索します)。そのためルートで一度 `acr init` を実行すれば十分で、
   `~/dev/atcoder-rs/awc/` のようなカテゴリ別サブフォルダも自動的に
   カバーされます。

   `acr init` は何度でも再実行できます — 現在の設定値がプロンプトの
   デフォルトとして提示されるので、Enter を押せばそのまま維持されます。

3. **AtCoder にログイン** — `acr login` は AtCoder のログインページを
   ブラウザで開き、`REVEL_SESSION` クッキーの値の貼り付けを待ちます。
   DevTools → Application → Cookies → atcoder.jp から `REVEL_SESSION`
   行の値をコピーしてください。クッキーの有効期限はおよそ 1 か月なので、
   切れたら再実行してください。

> なぜクッキーを手動で貼り付けるのか? AtCoder の Cloudflare Turnstile が
> CLI ツールによる自動フォームログインをブロックしているためです。
> DevTools → Application → Cookies → atcoder.jp から `REVEL_SESSION`
> クッキーを一度コピーするだけの 1 分で終わる作業です。

## 使い方

<!-- asciinema cast for terminal-only moments (test / update / submit).
     Record with `asciinema rec demo.cast --idle-time-limit=1`, upload
     via `asciinema upload demo.cast`, and uncomment the line below with
     the returned URL. -->
<!-- [![asciicast](https://asciinema.org/a/REPLACE_ME.svg)](https://asciinema.org/a/REPLACE_ME) -->


```bash
acr new abc001          # コンテストワークスペースを作成しエディタを起動 (エイリアス: n)
acr new abc001 --at 21:00  # 21:00 まで待機してからワークスペースを作成
acr virtual abc300      # バーチャルコンテストを開始 (約 2 分後にスタート)
acr virtual abc300 --at 21:00  # 21:00 開始のバーチャルコンテスト
acr virtual abc300 a b  # A と B の問題だけでバーチャルコンテストを実施
acr add e               # ワークスペースに問題を追加
acr open abc001         # 既存のワークスペースをエディタ + ブラウザで再度開く (エイリアス: o)
acr open abc001 a       # A 問題にフォーカスして再度開く

# 問題ディレクトリから
acr view                # 現在の問題ページをブラウザで開く (エイリアス: v)
acr update              # サンプルテストケースを再取得 (エイリアス: u)
acr update -c           # テンプレートから src/main.rs を再生成
acr update -d           # Cargo.toml の依存関係を最新の組み込みリストに更新
acr update -tc          # テストケースの再取得と src/main.rs の再生成
acr test                # 現在の問題のサンプルテストを実行 (エイリアス: t)
acr submit              # 現在の問題をテストして提出 (エイリアス: s)
acr submit -f           # テストが失敗しても提出

# コンテストディレクトリから
acr view a              # A 問題のページをブラウザで開く
acr open                # 現在のワークスペースを再度開く (エディタ + 最初の問題ページ)
acr open a              # A 問題にフォーカスして再度開く
acr update a            # A 問題のテストケースを再取得
acr update a b c        # A、B、C 問題のテストケースを再取得
acr update a -c         # A 問題の src/main.rs を再生成
acr update              # コンテスト内の全問題のテストケースを再取得
acr test a              # A 問題のテストを実行
acr submit a            # A 問題を提出
acr submissions         # 自分の提出一覧ページをブラウザで開く

# コンテストディレクトリの外から
acr update abc001       # abc001/ の全テストケースを再取得
acr update abc001 a b   # abc001/ の A、B 問題のテストケースを再取得
acr update abc001 -cd   # abc001/ の全問題のコードと依存関係を再生成
acr submissions abc001  # abc001 の提出一覧ページを開く

# セッション管理
acr session             # ログイン状態を確認
acr logout              # AtCoder からログアウト

# 設定
acr config              # 現在の設定を表示
acr config editor nvim  # エディタを変更
acr config editor "code --new-window"  # エディタにはフラグも指定可能
acr config browser open # ブラウザを変更 (デフォルト: xdg-open)
acr config browser "google-chrome --new-window"  # ブラウザにもフラグを指定可能
acr config browser '"/mnt/c/Program Files/Google/Chrome/Application/chrome.exe" --new-window'  # WSL2 + Chrome
```

## ライブラリ

`acr new` が生成する各問題の `Cargo.toml` は、**AtCoder のジャッジが提供
するクレートとバージョンをそのまま**固定します。よく使うクレートはそのまま
動きます:

```rust
use proconio::input;
use ac_library::ModInt998244353 as Mint;

fn main() {
    input! { n: usize, a: [Mint; n] }
    // ...
}
```

完全なリストは `src/workspace/generator.rs` に定義されており、
`ac-library-rs`、`proconio`、`itertools`、`num`、`nalgebra`、`ndarray`、
`petgraph`、`rustc-hash` などが含まれます。

ジャッジ側のクレートセットが更新されたときは、`acr update -d` で既存の
ワークスペースの固定済み依存関係を更新できます。

> ジャッジのリストにないクレートをローカルで追加すること自体は可能ですが
> (`Cargo.toml` を直接編集)、**その提出はジャッジ上でコンパイルに失敗**
> します。また `acr update -d` はローカルの編集を上書きします。

## ワークスペース構成

`acr new abc001` は以下を生成します:

```
abc001/
├── Cargo.toml          # [workspace]
├── a/
│   ├── Cargo.toml
│   ├── src/main.rs     # テンプレートから生成
│   └── tests/
│       ├── 1.in
│       └── 1.out
├── b/
│   └── ...
```

## 設定

設定ファイルは `~/.config/acr/` に保存されます:

- `config.toml` - エディタとブラウザの設定
- `template.rs` - ソースコードテンプレート
- `session.json` - ログインセッション

### テンプレート

`acr init` はデフォルトテンプレートを `~/.config/acr/template.rs` に
作成します:

```rust
use proconio::input;

fn main() {
    input! {
    }
}
```

このファイルを編集すると、各問題で生成されるボイラープレートを
カスタマイズできます。

### テンプレートの共有

他の人のテンプレート (または自分のもの) をファイルを手で編集せずに
インストールできます:

```bash
acr template add https://gist.github.com/someone/abcdef1234
acr template add https://github.com/someone/dotfiles/blob/main/atcoder.rs
acr template add ./my_template.rs
acr template show    # 現在のテンプレートを表示
acr template reset   # 組み込みのデフォルトに戻す
```

`add` と `reset` は書き込み前に既存のファイルを
`~/.config/acr/template.rs.bak` に退避するので、いつでも
`mv ~/.config/acr/template.rs.bak ~/.config/acr/template.rs` で
ロールバックできます。

GitHub の "blob" URL と Gist の通常 URL は自動的に raw コンテンツの URL に
書き換えられます。それ以外の `http(s)://` URL はそのまま取得されます。

## リリース (メンテナ向け)

リリースは [release-please](https://github.com/googleapis/release-please-action) と crates.io の [Trusted Publishing](https://crates.io/docs/trusted-publishing) で自動化されています:

1. Conventional Commits (`feat:`、`fix:`、`chore:`、破壊的変更は `feat!:`) を使って PR を `main` にマージします。
2. release-please がバージョンアップと CHANGELOG 更新を含む **Release PR** を自動で作成します。
3. Release PR をマージすると `vX.Y.Z` タグが打たれ、GitHub Release が公開されます。
4. タグの push が `.github/workflows/publish.yml` をトリガーし、OIDC で短命の crates.io トークンを取得して `cargo publish` を実行します。

初回の自動 publish の前に crates.io 側での一度きりのセットアップが必要です: crates.io の `acr-cli` クレート設定で、GitHub リポジトリ `t-seki/acr` とワークフローファイル名 `publish.yml` を trusted publisher として登録してください。

## ライセンス

MIT

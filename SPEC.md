# acr — 設計仕様書

AtCoder 向け Rust CLI ツール。atcoder-cli / cargo-compete の後継として設計。

---

## プロジェクト概要

| 項目 | 内容 |
|---|---|
| ツール名 | `acr` |
| 言語 | Rust |
| 目的 | AtCoder の競技プログラミングをRustで快適に行うCLIツール |
| 差別化 | AtCoder専用・ゼロ外部依存・2025/10ジャッジ対応済み |

---

## コマンド一覧

```
acr init                # 初回セットアップ（対話式）
acr login               # AtCoder ログイン
acr logout              # ログアウト
acr session             # ログイン状態確認
acr new <contest-id>    # コンテストワークスペース作成 + エディタ起動
acr add <problem>       # 問題を追加（コンテストディレクトリから実行）
acr test                # テスト実行（問題ディレクトリから実行）
acr submit              # テスト→提出→ブラウザ表示（問題ディレクトリから実行）
acr config [key] [val]  # 設定の表示・変更
```

---

## ディレクトリ構成（生成物）

```
abc001/                          ← acr new abc001 で生成
├── Cargo.toml                   ← [workspace] members = ["a", "b", ...]
├── a/
│   ├── Cargo.toml               ← package名: "abc001-a"
│   ├── src/main.rs              ← テンプレートから生成
│   └── tests/
│       ├── 1.in
│       ├── 1.out
│       ├── 2.in
│       └── 2.out
└── b/
    └── ...
```

### ワークスペース Cargo.toml

```toml
[workspace]
members = ["a", "b", "c", "d", "e", "f"]
resolver = "2"
```

### 問題 Cargo.toml

```toml
[package]
name = "abc001-a"
version = "0.1.0"
edition = "2021"

[package.metadata.acr]
problem_url = "https://atcoder.jp/contests/abc001/tasks/abc001_a"

[dependencies]
proconio = "0.4.5"
ac-library-rs = "0.1.1"
```

---

## 設定ファイル

### グローバル設定 `~/.config/acr/config.toml`

```toml
editor = "nvim"
browser = "open"   # macOS: open, Linux: xdg-open
```

### テンプレート `~/.config/acr/template.rs`

```rust
use proconio::input;

fn main() {
    input! {
    }
}
```

### セッション `~/.config/acr/session.json`

```json
{
  "revel_session": "xxxxxxxx..."
}
```

---

## AtCoder API（スクレイピング）

### ログイン

```
GET  https://atcoder.jp/login
     → input[name="csrf_token"] を取得

POST https://atcoder.jp/login
     body: username, password, csrf_token
     → Set-Cookie: REVEL_SESSION
```

### ログイン確認

```
GET  https://atcoder.jp/
     → li a[href^="/users/"] が存在すればログイン済み
```

### 問題一覧（JSON）

```
GET  https://atcoder.jp/contests/{contest_id}/standings/json
     ※ ログイン必須（REVEL_SESSION クッキーが必要）

レスポンスの TaskInfo を使用:
{
  "TaskInfo": [
    {
      "Assignment": "A",
      "TaskName": "どちらが低い？",
      "TaskScreenName": "abc001_a"
    }
  ]
}
```

### 問題一覧（フォールバック：タスクページ）

```
GET  https://atcoder.jp/contests/{contest_id}/tasks
     ※ standings/json が利用できない場合（typical90 等）のフォールバック

HTMLテーブルをスクレイピング:
- 1列目: 問題ラベル（"A", "B" / "001", "002" 等）
- 2列目: 問題名リンク（href に /tasks/{task_screen_name} を含む）
```

### サンプルケース取得

```
GET  https://atcoder.jp/contests/{contest_id}/tasks/{task_screen_name}
     → #task-statement 内の <pre> をスクレイピング
```

### 提出

```
GET  https://atcoder.jp/contests/{contest_id}/submit
     → csrf_token を再取得

POST https://atcoder.jp/contests/{contest_id}/submit
     body:
       csrf_token
       data.TaskScreenName = abc001_a
       data.LanguageId     = (提出ページのselectから動的取得)
       sourceCode          = (ソースコード)
```

#### language_id の取得方法

提出ページの `<select name="data.LanguageId">` から "Rust" を含む `<option>` を選ぶ。
バイナリ埋め込みにすると言語アップデートで壊れるため、**毎回動的取得**する。

### 提出結果確認

```
GET  https://atcoder.jp/contests/{contest_id}/submissions/me
     → 最新の提出ステータスをポーリング
```

---

## ソースコード構成

```
acr/
├── Cargo.toml
└── src/
    ├── main.rs               # エントリーポイント
    ├── cli.rs                # clap によるコマンド定義
    │
    ├── atcoder.rs            # AtCoderClient 本体
    ├── atcoder/
    │   ├── auth.rs           # ログイン・セッション管理
    │   ├── contest.rs        # 問題一覧取得（standings/json）
    │   ├── submit.rs         # 提出・結果確認
    │   └── scraper.rs        # HTML解析ユーティリティ
    │
    ├── workspace.rs
    ├── workspace/
    │   ├── generator.rs      # ディレクトリ・Cargo.toml生成
    │   └── testcase.rs       # テストケース保存・読み込み
    │
    ├── runner.rs
    ├── runner/
    │   └── tester.rs         # ビルド・テスト実行・結果表示
    │
    ├── config.rs
    ├── config/
    │   ├── global.rs         # ~/.config/acr/config.toml
    │   └── session.rs        # ~/.config/acr/session.json
    │
    └── error.rs              # エラー型定義
```

---

## 依存クレート

```toml
[dependencies]
# CLI
clap = { version = "4", features = ["derive"] }

# HTTP + スクレイピング
reqwest = { version = "0.12", features = ["cookies", "json"] }
scraper = "0.20"

# 非同期
tokio = { version = "1", features = ["full"] }

# シリアライズ
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# エラーハンドリング
anyhow = "1"
thiserror = "2"

# ターミナル表示
colored = "2"
indicatif = "0.17"

# ユーティリティ
dirs = "5"
```

---

## エラー型

```rust
#[derive(Error, Debug)]
pub enum AcrError {
    #[error("ログインしていません。`acr login` を実行してください")]
    NotLoggedIn,

    #[error("コンテスト '{0}' が見つかりません")]
    ContestNotFound(String),

    #[error("問題 '{0}' が見つかりません")]
    ProblemNotFound(String),

    #[error("テストが失敗しました（{passed}/{total} AC）")]
    TestFailed { passed: usize, total: usize },

    #[error("スクレイピングに失敗しました: {0}")]
    ScrapingFailed(String),

    #[error("設定ファイルが見つかりません。`acr init` を実行してください")]
    ConfigNotFound,

    #[error("既にコンテストディレクトリが存在します: {0}")]
    ContestAlreadyExists(String),
}
```

---

## コアデータ型

```rust
pub struct ContestInfo {
    pub contest_id: String,
    pub problems: Vec<Problem>,
}

pub struct Problem {
    pub alphabet: String,          // "A", "B", ...
    pub name: String,
    pub task_screen_name: String,  // "abc001_a"
    pub url: String,
}

pub struct TestCase {
    pub index: usize,
    pub input: String,
    pub expected: String,
}

pub enum TestResult {
    Ac,
    Wa { actual: String, expected: String },
    Re { stderr: String },
    Tle,
}
```

---

## コマンド別フロー

### `acr new <contest_id>`

1. `config::load()` — 設定読み込み
2. `config::load_session()` — セッション確認（なければエラー）
3. `atcoder::contest::fetch(contest_id)` — standings/json → ContestInfo
4. `workspace::generator::create(contest_id, problems)` — ディレクトリ・Cargo.toml 生成
5. 各問題を `tokio::spawn` で並列スクレイピング → テストケース保存
6. `open_editor(contest_dir)` — エディタ起動

### `acr test`

1. `workspace::detect_problem()` — カレントディレクトリから問題を特定
2. `workspace::testcase::load()` — tests/*.in/*.out 読み込み
3. `runner::tester::run_all()` — `cargo build` → 全テスト実行
4. 結果表示（colored で AC/WA/TLE/RE を色分け）

### `acr submit`

1. `runner::tester::run_all()` — テスト（失敗なら中断）
2. `atcoder::submit::post()` — CSRF 取得 → form POST
3. `atcoder::submit::poll_result()` — submissions/me をポーリング
4. `open_browser(result_url)` — 結果ページを開く

---

## 実装上の注意点

- `standings/json` はログイン必須（REVEL_SESSION クッキーが必要）
- `language_id` は提出ページの `<select>` から毎回動的取得する（ジャッジアップデート対策）
- サンプルケース取得は `tokio::spawn` で並列化（indicatif でプログレス表示）
- CSRF token はログイン・提出それぞれで毎回再取得する
- `acr add` はコンテストディレクトリから、`acr test/submit` は問題ディレクトリから実行
- `acr init` は既存ファイルをスキップ、`--force` で上書き

---

## 言語アップデート履歴（参考）

| 時期 | Rust バージョン | language_id |
|---|---|---|
| 2020 | 1.42.0 | 4050 |
| 2023/08 | 1.70.0 | 5054 |
| 2025/10 | 1.86.0 | 動的取得 |
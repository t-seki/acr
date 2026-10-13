# Development

## Supervisor

`/supervise`（dotfiles の共通 skill）が読む repo 固有の手順。

### worktree の準備

なし。git 管理外のファイルはビルド出力の `target/` だけで、秘密や設定のファイルは無い。worker の worktree は `.claude/worktrees/` に切られる（`.gitignore` 済み）。

### マージ前の検証

なし。CI（`.github/workflows/ci.yml` の fmt・clippy・3 OS の test・msrv・cargo-deny）で足りる。

### マージの承認

自動。ただし次のパスを触る PR は承認制。

- `.release-please-manifest.json`: release-please の Release PR が必ず触る。Release PR のマージでタグが打たれ、crates.io への publish（`.github/workflows/publish.yml`）と GitHub Release / cargo-dist のビルド（`release.yml`）が走る。publish は取り消せない
- `release-please-config.json`、`dist-workspace.toml`: リリースの仕組みの設定
- `.github/workflows/`: `publish.yml` はタグの push と手動実行（`workflow_dispatch`）でしか動かず、PR の段階では検査されない。また workflow を触る PR のマージには gh のトークンの `workflow` スコープが要ることがある。Renovate の github-actions の更新 PR（action の digest を更新して workflow を触る）も承認制になるが、意図どおり

```auto-merge
.release-please-manifest.json
release-please-config.json
dist-workspace.toml
.github/workflows/
```

### マージ後の確認（デプロイ）

普通の PR はなし（release-please が Release PR を自動で開く・更新する）。Release PR のマージ後だけ、タグの push で走る publish と Release の workflow が成功したかを `gh run list` で確かめる。リリースの流れは README の [Releasing (maintainers)](../README.md#releasing-maintainers) を参照。

### 大きさの基準

`38 行・2185 バイト（9dc2ce3）`

この development.md の大きさの基準（測り方と値の形は [REPO-SECTION.md の「大きさの基準」](https://github.com/t-seki/dotfiles/blob/main/claude/.claude/skills/supervise/REPO-SECTION.md#大きさの基準)）。

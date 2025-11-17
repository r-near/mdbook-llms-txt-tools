# mdbook-llms-txt

mdbookをllmstxt.org形式に変換するためのツールです。

## 概要

このツールは[mdbook](https://rust-lang.github.io/mdBook/)で作成されたドキュメントを[llmstxt.org](https://llmstxt.org/)形式に変換するためのRendererです。これにより、mdbookで作成された技術文書をLLMsに最適化された形式で共有することができます。

## 機能

### 実装済みの機能
- ✅ 基本的なllms.txt形式へのレンダリング
- ✅ 詳細情報を含むllms-full.txt形式へのレンダリング
- ✅ `only_section`設定によるセクションフィルタリング

### 計画中の機能
- 🚧 llms.txt形式専用のメタデータサポート

## インストール方法

```bash
cargo install mdbook-llms-txt-tools
```

## 使用方法

### 基本的な使い方

1. プロジェクトの`book.toml`に以下の設定を追加します：

```toml
# 基本的なllmstxt.org形式の出力
[output.llms-txt]
document_root_uri = "https://your-docs-site.com"

# より詳細な情報を含むllmstxt.org形式の出力
[output.llms-txt-full]
```

2. 通常通り`mdbook build`を実行すると、選択した出力形式でファイルが生成されます。

### 高度な使い方：セクションによるフィルタリング

`only_section`設定を使用することで、特定のセクションのみのllms.txtファイルを生成できます。これは、AIエージェント向けや特定の用途に特化したドキュメントを作成する場合に便利です。

```toml
[output.llms-txt]
document_root_uri = "https://your-docs-site.com"
only_section = "For AI Agents"

[output.llms-txt-full]
only_section = "For AI Agents"
```

`only_section`の値は、`SUMMARY.md`ファイル内のチャプタータイトルと完全に一致する必要があります。設定すると、指定されたセクションとそのサブチャプターのみが生成される出力に含まれます。

**SUMMARY.mdの例：**
```markdown
# Summary

[はじめに](./intro.md)
- [はじめる](./getting-started.md)
- [For AI Agents](./ai-agents.md)
  - [エージェント設定](./ai-agents/config.md)
  - [エージェント例](./ai-agents/examples.md)
- [高度なトピック](./advanced.md)
```

`only_section = "For AI Agents"`と設定すると、出力には「For AI Agents」チャプターとその2つのサブチャプター（設定と例）のみが含まれます。

## 出力形式について

- `llms-txt`: 基本的なllmstxt.org形式の出力を生成します。一般的な用途に適しています。
- `llms-full-txt`: より詳細な情報を含むllmstxt.org形式の出力を生成します。より高度な分析や処理が必要な場合に使用します。

## ライセンス

MIT

## コントリビューション

バグ報告や機能要望は[GitHub Issues](https://github.com/higumachan/mdbook-llms-txt/issues)にお願いします。
プルリクエストも歓迎です。 
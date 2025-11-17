use anyhow::Result;
use clap::{Parser, Subcommand};
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mdbook-llms-txt",
    about = "A mdbook backend for generating llms.txt files"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Supports {
        #[arg(required = true)]
        renderer: String,
    },
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    if let Some(Commands::Supports { renderer }) = cli.command {
        if renderer == "llms-txt" {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin)?;

    let output = render_llm_txt(&ctx)?;

    let output_path = PathBuf::from(&ctx.destination).join("llms.txt");
    fs::write(output_path, output)?;

    Ok(())
}

pub fn render_llm_txt(ctx: &RenderContext) -> anyhow::Result<String> {
    let mut output = String::new();
    let book = &ctx.book;

    // Get document_root_uri from config
    let document_root_uri = match ctx
        .config
        .get("output.llms-txt.document_root_uri")
        .and_then(|v| v.as_str())
    {
        Some(uri) => uri,
        None => {
            log::warn!("document_root_uri is not set in book.toml. Links will be generated without base URL.");
            ""
        }
    };

    // Use the title from book.toml
    let title = ctx
        .config
        .book
        .title
        .as_deref()
        .expect("book.title is required");

    output.push_str(&format!("# {}\n\n", title));

    // Add description if exists
    if let Some(description) = &ctx.config.book.description {
        output.push_str(&format!("> {}\n\n", description));
    }

    // NEW: optional section filter
    let only_section = ctx
        .config
        .get("output.llms-txt.only_section")
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned());

    // Process chapters
    for item in &book.sections {
        if let BookItem::Chapter(chapter) = item {
            // If we configured only_section, skip everything else
            if let Some(ref section_name) = only_section {
                if &chapter.name != section_name {
                    continue;
                }
            }

            // Add section name
            output.push_str(&format!("## {}\n\n", chapter.name));

            // Add chapter content as a link
            if let Some(path) = &chapter.path {
                output.push_str(&format!(
                    "- [{}]({}/{})\n",
                    chapter.name,
                    document_root_uri,
                    path.display()
                ));
            }

            // Process subchapters
            for sub_item in &chapter.sub_items {
                if let BookItem::Chapter(sub_chapter) = sub_item {
                    if let Some(path) = &sub_chapter.path {
                        output.push_str(&format!(
                            "- [{}]({}/{})\n",
                            sub_chapter.name,
                            document_root_uri,
                            path.display()
                        ));
                    }
                }
            }
            output.push('\n');

            // If we're filtering to a single section, we can stop after it
            if only_section.is_some() {
                break;
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    const SAMPLE_RENDERED_OUTPUT: &str = "\
# サンプルブック

> これはサンプルブックです。

## はじめに

- [はじめに](https://example.com/docs/introduction.md)

## 第1章: サンプル

- [第1章: サンプル](https://example.com/docs/chapter_1.md)
- [1.1 サブセクション](https://example.com/docs/chapter_1/section_1.md)

## 第2章: 機能紹介

- [第2章: 機能紹介](https://example.com/docs/chapter_2.md)

";

    #[test]
    fn simple_project_json() -> Result<()> {
        let json_str = include_str!("../../assets/test_render_contexts/simple-project.json");
        let ctx: RenderContext = serde_json::from_str(json_str)?;

        let output = render_llm_txt(&ctx)?;

        let expected = SAMPLE_RENDERED_OUTPUT;

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn test_only_section_filter() -> Result<()> {
        let json_str = include_str!("../../assets/test_render_contexts/simple-project.json");
        let mut ctx: RenderContext = serde_json::from_str(json_str)?;

        // Add only_section config
        use serde_json::json;
        ctx.config
            .set("output.llms-txt.only_section", json!("第1章: サンプル"))
            .unwrap();

        let output = render_llm_txt(&ctx)?;

        // Should only contain the first chapter
        assert!(output.contains("# サンプルブック"));
        assert!(output.contains("> これはサンプルブックです。"));
        assert!(output.contains("## 第1章: サンプル"));
        assert!(output.contains("- [第1章: サンプル](https://example.com/docs/chapter_1.md)"));
        assert!(output.contains("- [1.1 サブセクション](https://example.com/docs/chapter_1/section_1.md)"));

        // Should NOT contain other chapters
        assert!(!output.contains("## はじめに"));
        assert!(!output.contains("## 第2章: 機能紹介"));

        Ok(())
    }
}

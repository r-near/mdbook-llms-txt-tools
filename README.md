# mdbook-llms-txt

A tool to convert mdbook documents to llmstxt.org format.

## Overview

This tool is a renderer that converts documents created with [mdbook](https://rust-lang.github.io/mdBook/) into [llmstxt.org](https://llmstxt.org/) format. This allows you to share technical documentation created with mdbook in a format optimized for LLMs.

## Features

### Implemented Features
- ✅ Basic llms.txt rendering
- ✅ llms-full.txt rendering with detailed information
- ✅ Section filtering with `only_section` configuration

### Planned Features
- 🚧 Additional metadata support specifically for llms.txt format

## Installation

```bash
cargo install mdbook-llms-txt-tools
```

## Usage

### Basic Usage

1. Add the following configuration to your project's `book.toml`:

```toml
# Basic llmstxt.org format output
[output.llms-txt]
document_root_uri = "https://your-docs-site.com"

# Detailed llmstxt.org format output with additional information
[output.llms-txt-full]
```

2. Run `mdbook build` as usual, and files will be generated in your chosen output format.

### Advanced Usage: Filtering by Section

You can generate llms.txt files for a specific section only by using the `only_section` configuration. This is useful when you want to create documentation targeted specifically for AI agents or other specific use cases.

```toml
[output.llms-txt]
document_root_uri = "https://your-docs-site.com"
only_section = "For AI Agents"

[output.llms-txt-full]
only_section = "For AI Agents"
```

The `only_section` value should match the exact chapter title from your `SUMMARY.md` file. When configured, only the specified section and its subchapters will be included in the generated output.

**Example SUMMARY.md:**
```markdown
# Summary

[Introduction](./intro.md)
- [Getting Started](./getting-started.md)
- [For AI Agents](./ai-agents.md)
  - [Agent Configuration](./ai-agents/config.md)
  - [Agent Examples](./ai-agents/examples.md)
- [Advanced Topics](./advanced.md)
```

With `only_section = "For AI Agents"`, the output will only include the "For AI Agents" chapter and its two subchapters (Configuration and Examples).

## Output Formats

- `llms-txt`: Generates basic llmstxt.org format output. Suitable for general use.
- `llms-full-txt`: Generates llmstxt.org format output with more detailed information. Use this when more advanced analysis or processing is needed.

## License

MIT

## Contributing

Please report bugs and feature requests on [GitHub Issues](https://github.com/higumachan/mdbook-llms-txt/issues).
Pull requests are welcome. 
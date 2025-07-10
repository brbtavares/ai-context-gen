# AI Context Generator

[![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
[![Documentation](https://docs.rs/ai-context-gen/badge.svg)](https://docs.rs/ai-context-gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Build Status](https://github.com/brbtavares/ai-context-gen/workflows/CI/badge.svg)](https://github.com/brbtavares/ai-context-gen/actions)
[![Downloads](https://img.shields.io/crates/d/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)

A context generator for Rust repositories that creates a structured markdown file with relevant information for LLMs and AI agents.

Available as:

- 🔧 **CLI Tool**: Use directly from command line
- 📚 **Rust Library**: Integrate into your Rust application

## Features

- 🔍 **Complete Scanning**: Analyzes all `.rs` and `.md` files in the repository
- 🌳 **Abstract Syntax Tree**: Extracts and documents structures, functions, enums and implementations
- 📊 **Token Control**: Respects token limits and prioritizes important content
- 📁 **Project Structure**: Generates file tree visualization
- 📖 **Documentation**: Includes markdown files like README, documentation, etc.
- ⚡ **Performance**: Asynchronous and optimized processing

## Installation

### Prerequisites

- Rust 1.70 or higher
- Linux system (tested on Ubuntu/Debian)

### Build

```bash
# Clone the repository
git clone https://github.com/brbtavares/ai-context-gen
cd ai-context-gen

# Option 1: Using make (recommended)
make build

# Option 2: Using the script directly
chmod +x build.sh
./build.sh

# Option 3: Using cargo directly
make release
```

### Global Installation

```bash
# Using make (recommended)
make install

# Or using the installation script
./install.sh

# Or manually
sudo cp target/release/ai-context-gen /usr/local/bin/
```

### Available Make Commands

```bash
# Development
make dev          # Build and run in development mode
make demo         # Run demo with current directory
make help         # Show application help

# Build
make build        # Build using script (recommended)
make release      # Optimized build for release

# Code quality
make test         # Run tests
make check        # Run format, lint and tests
make fmt          # Check formatting
make lint         # Check linting

# Installation
make install      # Install on system
make uninstall    # Remove from system

# Utilities
make clean        # Clean build artifacts
make info         # Show project information
make help-make    # Show all make commands
```

## Usage

### CLI Usage

#### Basic Usage

```bash
# Analyze current directory
ai-context-gen

# Analyze a specific directory
ai-context-gen --path /path/to/project

# Specify token limit
ai-context-gen --max-tokens 100000

# Specify output file name
ai-context-gen --output project_context.md
```

#### Available Options

```bash
ai-context-gen --help
```

**Options:**

- `-p, --path <PATH>`: Path to repository (default: current directory)
- `-m, --max-tokens <MAX_TOKENS>`: Maximum number of tokens (default: 50000)
- `-o, --output <OUTPUT>`: Output file name (default: repo_context.md)
- `--include-hidden`: Include hidden files and directories
- `--include-deps`: Include external dependencies analysis

#### Examples

```bash
# Complete analysis with high token limit
ai-context-gen --max-tokens 200000 --output complete_context.md

# Analysis including hidden files
ai-context-gen --include-hidden --path ~/my-project

# Quick analysis with low limit
ai-context-gen --max-tokens 10000 --output summary.md

# Using make for development
make demo

# Running tests
make test
```

### Rust Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ai-context-gen = "0.1.0"
```

#### Basic Example

```rust
use ai_context_gen::generate_context;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Generate context for current directory
    generate_context(PathBuf::from("."), "context.md".to_string()).await?;
    println!("Context generated in context.md");
    Ok(())
}
```

#### Advanced Example

```rust
use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Custom configuration
    let config = Config {
        repo_path: PathBuf::from("./my-project"),
        max_tokens: 100000,
        output_file: "detailed_context.md".to_string(),
        include_hidden: true,
        include_deps: true,
    };

    // Two-step process
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;
    
    println!("Files found: {}", scan_result.files.len());
    
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;
    
    println!("Context generated successfully!");
    Ok(())
}
```

#### Available API

- **`generate_context(path, output)`**: Simple function for basic cases
- **`generate_context_with_config(config)`**: Function with custom configuration
- **`Config`**: Configuration structure
- **`RepositoryScanner`**: File scanning
- **`ContextGenerator`**: Context generation
- **`RustParser`**: Rust code parser

## Generated File Structure

The generated file contains the following sections (in priority order):

### 1. Project Metadata (High Priority)

- Project name and description
- Rust version
- Main dependencies
- Project statistics

### 2. Project Structure (High Priority)

- File tree
- Directory organization

### 3. Markdown Documentation (High Priority)

- README.md
- Other .md files found
- Project documentation

### 4. Rust AST Analysis (Medium Priority)

- Structures (structs)
- Enumerations (enums)
- Functions
- Implementations (impls)
- Modules
- Code documentation

### 5. Source Code (Low Priority)

- Complete content of .rs files
- Syntax highlighting for markdown

## Prioritization Algorithm

The system uses an intelligent prioritization algorithm:

1. **High Priority (9)**: Metadata, structure and documentation
2. **Medium Priority (5)**: AST analysis and code architecture
3. **Low Priority (1)**: Complete source code

When the token limit is reached, the system:

- Includes high priority sections first
- Truncates low priority sections if necessary
- Reports which sections were truncated

## Ignored Files

The system automatically ignores:

**Directories:**

- `target/`
- `node_modules/`
- `.git/`
- `.vscode/`
- `.idea/`

**Files:**

- `Cargo.lock`
- `.gitignore`
- `.DS_Store`

## Token Counting

Uses the GPT-4 tokenizer for precise token counting, ensuring compatibility with:

- OpenAI GPT-4
- Claude
- Other models based on similar tokens

## Use Cases

### For Developers

- Automatic project documentation
- Onboarding new team members
- Code architecture analysis

### For LLMs/AI

- Structured context for code assistants
- Analysis of existing projects
- Documentation generation
- Automated code review

### For Documentation

- Project wiki generation
- Architecture reports
- Technical documentation

## Limitations

- Supports only Rust projects
- Analyzes only `.rs` and `.md` files
- Requires Linux system for execution
- Token limit may truncate content

## Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a branch for your feature
3. Implement your changes
4. Add tests if necessary
5. Open a Pull Request

## License

This project is licensed under the MIT license. See the `LICENSE` file for details.

## Roadmap

- [ ] Web interface
- [ ] Git integration
- [ ] Commit history analysis
- [ ] Support for other output formats (JSON, YAML)
- [ ] Cache for better performance

## Changelog

### v0.1.0

- Initial implementation
- Support for Rust AST analysis
- Content prioritization system
- Token counting with tiktoken
- Structured markdown file generation

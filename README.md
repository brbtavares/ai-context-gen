# AI Context Generator

[![Crates.i```toml
# Cargo.toml
[dependencies]
ai-context-gen = "0.1.2"
```tps://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
[![Documentation](https://docs.rs/ai-context-gen/badge.svg)](https://docs.rs/ai-context-gen)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Build Status](https://github.com/brbtavares/ai-context-gen/workflows/CI/badge.svg)](https://github.com/brbtavares/ai-context-gen/actions)
[![Downloads](https://img.shields.io/crates/d/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)

A context generator for Rust repositories that creates a structured markdown file with relevant information for LLMs and AI agents.

## 🎯 Quick Start

**Choose your preferred way to use AI Context Generator:**

| Usage Mode | When to Use | Quick Start |
|------------|-------------|-------------|
| 🔧 **CLI Tool** | Interactive use, one-time analysis, scripts | `ai-context-gen --path ./my-project` |
| 📚 **Rust Library** | Integrate into Rust apps, custom workflows | `cargo add ai-context-gen` |

---

### 🚀 CLI Quick Start

```bash
# Install globally
git clone https://github.com/brbtavares/ai-context-gen
cd ai-context-gen && make install

# Use anywhere
ai-context-gen --path /path/to/project
```

### 📦 Library Quick Start

```toml
# Cargo.toml
[dependencies]
ai-context-gen = "0.1.1"
```

```rust
use ai_context_gen::generate_context;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    generate_context(PathBuf::from("."), "context.md".to_string()).await?;
    Ok(())
}
```

## Features

- 🔍 **Complete Scanning**: Analyzes all `.rs` and `.md` files in the repository
- 🌳 **Abstract Syntax Tree**: Extracts and documents structures, functions, enums and implementations
- 📊 **Token Control**: Respects token limits and prioritizes important content
- 📁 **Project Structure**: Generates file tree visualization
- 📖 **Documentation**: Includes markdown files like README, documentation, etc.
- ⚡ **Performance**: Asynchronous and optimized processing

---

## 🔧 CLI Tool Usage

The AI Context Generator CLI is perfect for interactive use, one-time analysis, and shell scripts.

### Installation

#### Prerequisites

- Rust 1.76 or higher
- Linux system (tested on Ubuntu/Debian)

#### Build and Install

```bash
# Clone the repository
git clone https://github.com/brbtavares/ai-context-gen
cd ai-context-gen

# Build and install globally (recommended)
make install

# Alternative: step by step
make build
sudo cp target/release/ai-context-gen /usr/local/bin/
```

#### Verify Installation

```bash
# Check if installed correctly
ai-context-gen --version
ai-context-gen --help

# Should work from any directory
cd /tmp && ai-context-gen --path ~/my-project
```

### Developer Commands (Make)

```bash
# Development & Testing
make dev          # Build and run in development mode
make demo         # Run demo with current directory
make test         # Run tests
make check        # Run format, lint and tests

# Build & Installation
make build        # Build using script (recommended)
make install      # Install on system
make uninstall    # Remove from system

# Utilities
make clean        # Clean build artifacts
make help-make    # Show all make commands
```

### CLI Commands

#### Basic Usage

```bash
# Analyze current directory (interactive mode)
ai-context-gen

# Analyze specific directory
ai-context-gen --path /path/to/project

# Custom output file
ai-context-gen --output my_context.md

# High token limit for large projects
ai-context-gen --max-tokens 100000
```

#### All CLI Options

```bash
ai-context-gen [OPTIONS]

Options:
    -p, --path <PATH>              Path to repository (default: current directory)
    -m, --max-tokens <MAX_TOKENS>  Maximum number of tokens (default: 50000)
    -o, --output <OUTPUT>          Output file name (default: repo_context.md)
        --include-hidden           Include hidden files and directories
        --include-deps             Include external dependencies analysis
    -h, --help                     Print help
    -V, --version                  Print version
```

#### CLI Examples

```bash
# Complete analysis with all options
ai-context-gen --path ~/my-rust-project --max-tokens 200000 --output complete_analysis.md --include-hidden

# Quick summary
ai-context-gen --max-tokens 10000 --output summary.md

# Analyze remote/different project
ai-context-gen --path /opt/some-project --output /tmp/analysis.md
```

---

## 📚 Rust Library Usage

The AI Context Generator library is perfect for integrating context generation into your Rust applications.

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ai-context-gen = "0.1.2"
```

### Library Examples

#### Simple Usage

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

#### Advanced Usage with Custom Configuration

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

    // Two-step process for more control
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;
    
    println!("Files found: {}", scan_result.files.len());
    
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;
    
    println!("Context generated successfully!");
    Ok(())
}
```

#### Using with Custom Configuration Function

```rust
use ai_context_gen::{Config, generate_context_with_config};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config {
        repo_path: PathBuf::from("/path/to/analyze"),
        max_tokens: 75000,
        output_file: "custom_context.md".to_string(),
        include_hidden: false,
        include_deps: true,
    };

    generate_context_with_config(config).await?;
    Ok(())
}
```

### Available Library API

- **`generate_context(path, output)`**: Simple function for basic cases
- **`generate_context_with_config(config)`**: Function with custom configuration
- **`Config`**: Configuration structure
- **`RepositoryScanner`**: File scanning and analysis
- **`ContextGenerator`**: Context generation with priorities
- **`RustParser`**: Rust code AST parser

---

## 📋 Generated File Structure

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

## 🧠 Prioritization Algorithm

The system uses an intelligent prioritization algorithm:

1. **High Priority (9)**: Metadata, structure and documentation
2. **Medium Priority (5)**: AST analysis and code architecture
3. **Low Priority (1)**: Complete source code

When the token limit is reached, the system:

- Includes high priority sections first
- Truncates low priority sections if necessary
- Reports which sections were truncated

## 🚫 Ignored Files

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

## 🔢 Token Counting

Uses the GPT-4 tokenizer for precise token counting, ensuring compatibility with:

- OpenAI GPT-4
- Claude
- Other models based on similar tokens

## 🎯 Use Cases

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

## ⚠️ Limitations

- Supports only Rust projects
- Analyzes only `.rs` and `.md` files
- Requires Linux system for execution
- Token limit may truncate content

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a branch for your feature
3. Implement your changes
4. Add tests if necessary
5. Open a Pull Request

### Development Setup

```bash
git clone https://github.com/brbtavares/ai-context-gen
cd ai-context-gen
cargo build
cargo test
```

### Release Process

For maintainers, releases are automated. See [RELEASE.md](RELEASE.md) for details.

```bash
# Update version and changelog, then:
git tag v0.1.2
git push origin v0.1.2
# GitHub Actions handles the rest!
```

## 📄 License

This project is licensed under the MIT license. See the `LICENSE` file for details.

## 🛤️ Roadmap

- [ ] Web interface
- [ ] Git integration
- [ ] Commit history analysis
- [ ] Support for other output formats (JSON, YAML)
- [ ] Cache for better performance

## 📝 Changelog

### v0.1.2

- 🔧 **MSRV Compatibility**: Updated MSRV to Rust 1.76 to support Cargo lock file version 4
- 🚀 **Workflow Improvements**: Modernized GitHub Actions workflows with updated actions
- 🛠️ **CI Fixes**: Fixed MSRV check to handle newer lock file formats correctly
- 📦 **Release Automation**: Enhanced release workflow with better asset handling
- 🔒 **Better Error Handling**: Improved release workflow with proper secret handling

### v0.1.1

- ✨ **Documentation Improvements**: Completely restructured README.md and lib.rs documentation
- 🔧 **Clear CLI/Library Separation**: Added clear distinction between CLI and library usage
- 📚 **Enhanced Library Examples**: Added multiple usage patterns and integration examples
- 🎯 **Quick Start Guide**: Added comparison table and clear guidance on when to use each mode
- 🛠️ **Better Error Handling**: Improved CLI output messages and error reporting
- 📖 **API Documentation**: Enhanced rustdoc comments with comprehensive examples
- 🧹 **Code Organization**: Improved module structure and exports

### v0.1.0

- Initial implementation
- Support for Rust AST analysis
- Content prioritization system
- Token counting with tiktoken
- Structured markdown file generation

## 🔧 Troubleshooting

### CLI Installation Issues

#### Command not found after installation

```bash
# Check if /usr/local/bin is in your PATH
echo $PATH | grep -o '/usr/local/bin'

# If not found, add to your shell profile
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
which ai-context-gen
ai-context-gen --version
```

#### Permission denied during installation

```bash
# Make sure you have sudo privileges
sudo make install

# Or install manually
make build
sudo cp target/release/ai-context-gen /usr/local/bin/
sudo chmod +x /usr/local/bin/ai-context-gen
```

#### Old version conflicts

```bash
# Remove old installations
rm -f ~/.local/bin/ai-context-gen
sudo rm -f /usr/local/bin/ai-context-gen

# Reinstall fresh
make clean
make install
```

### Library Usage Issues

#### Tokio runtime errors

Make sure you're using `#[tokio::main]` or initializing a runtime:

```rust
// Option 1: Use tokio::main
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // your code here
}

// Option 2: Manual runtime
fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        // your async code here
    })
}
```

#### File permission errors

```rust
use ai_context_gen::{Config, generate_context_with_config};
use std::path::PathBuf;

// Make sure output directory is writable
let config = Config {
    repo_path: PathBuf::from("./my-project"),
    output_file: "/tmp/context.md".to_string(), // Use temp dir if needed
    // ... other config
};
```

### General Issues

#### Large projects hitting token limits

```bash
# Use higher token limits for large projects
ai-context-gen --path ./large-project --max-tokens 200000

# Or focus on specific parts
ai-context-gen --path ./large-project/src --max-tokens 50000
```

#### Including/excluding files

```bash
# Include hidden files
ai-context-gen --include-hidden

# For library usage, modify Config
let config = Config {
    include_hidden: true,
    include_deps: true,
    // ...
};
```

---

## ⚠️ Limitations

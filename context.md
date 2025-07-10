# AI Context Generation Report

Generated on: 2025-07-10 12:38:21 UTC
Repository: .
Max tokens: 50000

## Table of Contents

1. Project Metadata
2. Project Structure
3. Documentation: context.md (truncated)

---

# Project Metadata

**Name:** ai-context-gen
**Description:** [![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
[![Documentation](https://docs.rs/ai-context-gen/badge.svg)](https://docs.rs/ai-context-gen)
**Dependencies:**
- clap
- walkdir
- serde
- serde_json
- syn
- quote
- proc-macro2
- tiktoken-rs
- anyhow
- regex
- chrono
- version
- features
**Version:** 0.1.1
**Total files:** 12
**Total size:** 319018 bytes

---

# Project Structure

```
├── README.md
├── RELEASE.md
├── context.md
│   ├── examples/advanced_usage.rs
│   ├── examples/basic_usage.rs
│   ├── src/config.rs
│   ├── src/generator.rs
│   ├── src/lib.rs
│   ├── src/main.rs
│   ├── src/parser.rs
│   ├── src/scanner.rs
│   └── src/token_counter.rs
```

---

# Documentation: context.md

# AI Context Generation Report

Generated on: 2025-07-10 12:27:04 UTC
Repository: .
Max tokens: 50000

## Table of Contents

1. Project Metadata
2. Project Structure
3. Documentation: context.md
4. Documentation: README.md
5. Rust Analysis: src/main.rs
6. Rust Analysis: src/generator.rs
7. Rust Analysis: src/config.rs
8. Rust Analysis: src/scanner.rs
9. Rust Analysis: src/lib.rs
10. Rust Analysis: src/parser.rs
11. Rust Analysis: src/token_counter.rs
12. Rust Analysis: examples/advanced_usage.rs
13. Rust Analysis: examples/basic_usage.rs
14. Source: src/main.rs
15. Source: src/generator.rs
16. Source: src/config.rs
17. Source: src/scanner.rs
18. Source: src/lib.rs
19. Source: src/parser.rs
20. Source: src/token_counter.rs
21. Source: context.md (truncated)

---

# Project Metadata

**Name:** ai-context-gen
**Description:** [![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
[![Documentation](https://docs.rs/ai-context-gen/badge.svg)](https://docs.rs/ai-context-gen)
**Dependencies:**
- clap
- walkdir
- serde
- serde_json
- syn
- quote
- proc-macro2
- tiktoken-rs
- anyhow
- regex
- chrono
- version
- features
**Version:** 0.1.1
**Total files:** 11
**Total size:** 180531 bytes

---

# Project Structure

```
├── README.md
├── context.md
│   ├── examples/advanced_usage.rs
│   ├── examples/basic_usage.rs
│   ├── src/config.rs
│   ├── src/generator.rs
│   ├── src/lib.rs
│   ├── src/main.rs
│   ├── src/parser.rs
│   ├── src/scanner.rs
│   └── src/token_counter.rs
```

---

# Documentation: context.md

# AI Context Generation Report

Generated on: 2025-07-10 12:26:25 UTC
Repository: .
Max tokens: 50000

## Table of Contents

1. Project Metadata
2. Project Structure
3. Documentation: README.md
4. Rust Analysis: src/main.rs
5. Rust Analysis: src/generator.rs
6. Rust Analysis: src/config.rs
7. Rust Analysis: src/scanner.rs
8. Rust Analysis: src/lib.rs
9. Rust Analysis: src/parser.rs
10. Rust Analysis: src/token_counter.rs
11. Rust Analysis: examples/advanced_usage.rs
12. Rust Analysis: examples/basic_usage.rs
13. Source: src/main.rs
14. Source: src/generator.rs
15. Source: src/config.rs
16. Source: src/scanner.rs
17. Source: src/lib.rs
18. Source: src/parser.rs
19. Source: src/token_counter.rs
20. Source: README.md
21. Source: examples/advanced_usage.rs
22. Source: examples/basic_usage.rs

---

# Project Metadata

**Name:** ai-context-gen
**Description:** [![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
[![Documentation](https://docs.rs/ai-context-gen/badge.svg)](https://docs.rs/ai-context-gen)
**Dependencies:**
- clap
- walkdir
- serde
- serde_json
- syn
- quote
- proc-macro2
- tiktoken-rs
- anyhow
- regex
- chrono
- version
- features
**Version:** 0.1.1
**Total files:** 10
**Total size:** 82062 bytes

---

# Project Structure

```
├── README.md
│   ├── examples/advanced_usage.rs
│   ├── examples/basic_usage.rs
│   ├── src/config.rs
│   ├── src/generator.rs
│   ├── src/lib.rs
│   ├── src/main.rs
│   ├── src/parser.rs
│   ├── src/scanner.rs
│   └── src/token_counter.rs
```

---

# Documentation: README.md

# AI Context Generator

[![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
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

- Rust 1.70 or higher
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
ai-context-gen = "0.1.1"
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

## 📄 License

This project is licensed under the MIT license. See the `LICENSE` file for details.

## 🛤️ Roadmap

- [ ] Web interface
- [ ] Git integration
- [ ] Commit history analysis
- [ ] Support for other output formats (JSON, YAML)
- [ ] Cache for better performance

## 📝 Changelog

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

---

# Rust Analysis: src/main.rs

## Functions
- **main**() -> Result < () > (private)

## Structs
- **Args**: 5 fields (private)

---

# Rust Analysis: src/generator.rs

## Structs
- **ContextGenerator**: 2 fields (pub)

## Implementations
- **impl ContextGenerator**: 8 methods

---

# Rust Analysis: src/config.rs

## Structs
- **Config**: 5 fields (pub)

## Implementations
- **impl Config**: 1 methods

---

# Rust Analysis: src/scanner.rs

## Structs
- **FileInfo**: 5 fields (pub)
- **ScanResult**: 3 fields (pub)
- **ProjectStructure**: 3 fields (pub)
- **ProjectMetadata**: 4 fields (pub)
- **RepositoryScanner**: 1 fields (pub)

## Enums
- **FileType**: 2 variants (pub)

## Implementations
- **impl RepositoryScanner**: 8 methods

---

# Rust Analysis: src/lib.rs

## Modules
- **config**: pub
- **generator**: pub
- **parser**: pub
- **scanner**: pub
- **token_counter**: pub

## Functions
- **generate_context**(path: PathBuf, output: String) -> Result < () > (pub)
- **generate_context_with_config**(config: Config) -> Result < () > (pub)

---

# Rust Analysis: src/parser.rs

## Structs
- **RustAnalysis**: 7 fields (pub)
- **ModuleInfo**: 3 fields (pub)
- **FunctionInfo**: 6 fields (pub)
- **StructInfo**: 4 fields (pub)
- **FieldInfo**: 3 fields (pub)
- **EnumInfo**: 4 fields (pub)
- **ImplInfo**: 3 fields (pub)
- **RustParser**: 0 fields (pub)

## Implementations
- **impl RustParser**: 11 methods

---

# Rust Analysis: src/token_counter.rs

## Structs
- **TokenCounter**: 1 fields (pub)
- **ContentPrioritizer**: 1 fields (pub)
- **ContentSection**: 4 fields (pub)

## Implementations
- **impl TokenCounter**: 3 methods
- **impl ContentPrioritizer**: 2 methods
- **impl ContentSection**: 4 methods

---

# Rust Analysis: examples/advanced_usage.rs

## Functions
- **main**() -> anyhow :: Result < () > (private)

---

# Rust Analysis: examples/basic_usage.rs

## Functions
- **main**() -> anyhow :: Result < () > (private)

---

# Source: src/main.rs

```rust
//! AI Context Generator CLI application.
//!
//! Command-line interface for generating structured context from Rust repositories.
//! This tool scans repository files, analyzes Rust code structure, and generates
//! markdown context suitable for LLMs and AI agents.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};

/// Command-line arguments for the AI Context Generator.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the repository to analyze
    ///
    /// Specifies the root directory of the project to scan. The tool will
    /// recursively process all supported files within this directory.
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Maximum number of tokens to include in the output
    ///
    /// Controls the size of the generated context to fit within LLM token limits.
    /// Content is prioritized and truncated as needed to stay within this limit.
    #[arg(short, long, default_value = "50000")]
    max_tokens: usize,

    /// Output file name for the generated context
    ///
    /// The markdown file where the generated context will be written.
    /// If the file exists, it will be overwritten.
    #[arg(short, long, default_value = "repo_context.md")]
    output: String,

    /// Include hidden files and directories in the analysis
    ///
    /// When enabled, files and directories starting with '.' will be included
    /// in the scan (except for those in the ignore list).
    #[arg(long)]
    include_hidden: bool,

    /// Include analysis of external dependencies
    ///
    /// When enabled, the tool will attempt to analyze and include information
    /// about external dependencies from Cargo.toml.
    #[arg(long)]
    include_deps: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config {
        repo_path: args.path,
        max_tokens: args.max_tokens,
        output_file: args.output,
        include_hidden: args.include_hidden,
        include_deps: args.include_deps,
    };

    println!("🔍 Scanning repository...");
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    println!("📝 Generating context...");
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    println!("✅ Context generated successfully!");
    Ok(())
}

```

---

# Source: src/generator.rs

```rust
//! Context generation module for the AI Context Generator.
//!
//! This module provides functionality to generate structured markdown context
//! from scanned repository data, with intelligent content prioritization and
//! token limit management.

use anyhow::Result;
use chrono::Utc;
use std::fs;

use crate::config::Config;
use crate::parser::RustParser;
use crate::scanner::{FileType, ScanResult};
use crate::token_counter::{ContentPrioritizer, ContentSection};

/// Context generator that creates structured markdown from repository scan results.
///
/// The generator takes scan results and creates a prioritized, token-limited markdown
/// document suitable for consumption by LLMs and AI agents. Content is organized by
/// priority, with metadata and documentation receiving higher priority than source code.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::default();
/// let scanner = RepositoryScanner::new(config.clone());
/// let scan_result = scanner.scan().await?;
///
/// let generator = ContextGenerator::new(config);
/// generator.generate_context(scan_result).await?;
/// # Ok(())
/// # }
/// ```
pub struct ContextGenerator {
    config: Config,
    prioritizer: ContentPrioritizer,
}

impl ContextGenerator {
    /// Creates a new context generator with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying output options and token limits
    ///
    /// # Panics
    ///
    /// Panics if the content prioritizer cannot be initialized (e.g., if the
    /// tiktoken model cannot be loaded).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, ContextGenerator};
    ///
    /// let config = Config::default();
    /// let generator = ContextGenerator::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config,
            prioritizer: ContentPrioritizer::new()
                .expect("Failed to initialize content prioritizer"),
        }
    }

    /// Generates a complete context document from scan results.
    ///
    /// This method creates a structured markdown document with prioritized content
    /// sections including project metadata, file structure, documentation, AST
    /// analysis, and source code. Content is prioritized and truncated based on
    /// the configured token limit.
    ///
    /// # Arguments
    ///
    /// * `scan_result` - Results from repository scanning containing files and metadata
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the context was successfully generated and written to the
    /// configured output file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - AST parsing fails for Rust files
    /// - The output file cannot be written
    /// - Token counting or content prioritization fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config.clone());
    /// let scan_result = scanner.scan().await?;
    ///
    /// let generator = ContextGenerator::new(config);
    /// generator.generate_context(scan_result).await?;
    ///
    /// println!("Context generated successfully!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_context(&self, scan_result: ScanResult) -> Result<()> {
        let mut sections = Vec::new();

        // Project metadata section (high priority)
        sections.push(self.create_metadata_section(&scan_result));

        // Project structure section (high priority)
        sections.push(self.create_structure_section(&scan_result));

        // Markdown documentation sections (high priority)
        sections.extend(self.create_markdown_sections(&scan_result));

        // AST analysis sections for Rust files (medium priority)
        sections.extend(self.create_rust_analysis_sections(&scan_result).await?);

        // Source code sections (low priority)
        sections.extend(self.create_source_code_sections(&scan_result));

        // Prioritize and truncate content based on token limit
        let final_sections = self
            .prioritizer
            .prioritize_content(sections, self.config.max_tokens);

        // Generate final context
        let context = self.format_context(final_sections);

        // Write to file
        fs::write(&self.config.output_file, context)?;

        println!(
            "Context generated successfully in: {}",
            self.config.output_file
        );
        Ok(())
    }

    fn create_metadata_section(&self, scan_result: &ScanResult) -> ContentSection {
        let mut content = String::new();
        content.push_str("# Project Metadata\n\n");
        content.push_str(&format!("**Name:** {}\n", scan_result.metadata.name));

        if let Some(description) = &scan_result.metadata.description {
            content.push_str(&format!("**Description:** {}\n", description));
        }

        if !scan_result.metadata.dependencies.is_empty() {
            content.push_str("**Dependencies:**\n");
            for dep in &scan_result.metadata.dependencies {
                content.push_str(&format!("- {}\n", dep));
            }
        }

        if let Some(rust_version) = &scan_result.metadata.rust_version {
            content.push_str(&format!("**Version:** {}\n", rust_version));
        }

        content.push_str(&format!(
            "**Total files:** {}\n",
            scan_result.project_structure.total_files
        ));
        content.push_str(&format!(
            "**Total size:** {} bytes\n\n",
            scan_result.project_structure.total_size
        ));

        ContentSection {
            title: "Project Metadata".to_string(),
            content,
            priority: 10,
            truncated: false,
        }
    }

    fn create_structure_section(&self, scan_result: &ScanResult) -> ContentSection {
        let mut content = String::new();
        content.push_str("# Project Structure\n\n");
        content.push_str(&scan_result.project_structure.tree);
        content.push('\n');

        ContentSection {
            title: "Project Structure".to_string(),
            content,
            priority: 9,
            truncated: false,
        }
    }

    fn create_markdown_sections(&self, scan_result: &ScanResult) -> Vec<ContentSection> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            if matches!(file.file_type, FileType::Markdown) {
                let mut content = String::new();
                content.push_str(&format!(
                    "# Documentation: {}\n\n",
                    file.relative_path.display()
                ));
                content.push_str(&file.content);
                content.push('\n');

                sections.push(ContentSection {
                    title: format!("Documentation: {}", file.relative_path.display()),
                    content,
                    priority: 8,
                    truncated: false,
                });
            }
        }

        sections
    }

    async fn create_rust_analysis_sections(
        &self,
        scan_result: &ScanResult,
    ) -> Result<Vec<ContentSection>> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            if matches!(file.file_type, FileType::Rust) {
                match RustParser::parse_rust_file(&file.path.to_string_lossy(), &file.content) {
                    Ok(analysis) => {
                        let mut content = String::new();
                        content.push_str(&format!(
                            "# Rust Analysis: {}\n\n",
                            file.relative_path.display()
                        ));

                        if !analysis.modules.is_empty() {
                            content.push_str("## Modules\n");
                            for module in &analysis.modules {
                                content.push_str(&format!(
                                    "- **{}**: {}\n",
                                    module.name, module.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.functions.is_empty() {
                            content.push_str("## Functions\n");
                            for function in &analysis.functions {
                                let params = function.parameters.join(", ");
                                let return_type = function.return_type.as_deref().unwrap_or("()");
                                content.push_str(&format!(
                                    "- **{}**({}) -> {} ({})\n",
                                    function.name, params, return_type, function.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.structs.is_empty() {
                            content.push_str("## Structs\n");
                            for struct_info in &analysis.structs {
                                content.push_str(&format!(
                                    "- **{}**: {} fields ({})\n",
                                    struct_info.name,
                                    struct_info.fields.len(),
                                    struct_info.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.enums.is_empty() {
                            content.push_str("## Enums\n");
                            for enum_info in &analysis.enums {
                                content.push_str(&format!(
                                    "- **{}**: {} variants ({})\n",
                                    enum_info.name,
                                    enum_info.variants.len(),
                                    enum_info.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.implementations.is_empty() {
                            content.push_str("## Implementations\n");
                            for impl_info in &analysis.implementations {
                                content.push_str(&format!(
                                    "- **impl {}**: {} methods\n",
                                    impl_info.target,
                                    impl_info.methods.len()
                                ));
                            }
                            content.push('\n');
                        }

                        sections.push(ContentSection {
                            title: format!("Rust Analysis: {}", file.relative_path.display()),
                            content,
                            priority: 6,
                            truncated: false,
                        });
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse {}: {}",
                            file.relative_path.display(),
                            e
                        );
                    }
                }
            }
        }

        Ok(sections)
    }

    fn create_source_code_sections(&self, scan_result: &ScanResult) -> Vec<ContentSection> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            let mut content = String::new();
            content.push_str(&format!("# Source: {}\n\n", file.relative_path.display()));
            content.push_str("```");

            match file.file_type {
                FileType::Rust => content.push_str("rust"),
                FileType::Markdown => content.push_str("markdown"),
            }

            content.push('\n');
            content.push_str(&file.content);
            content.push_str("\n```\n\n");

            sections.push(ContentSection {
                title: format!("Source: {}", file.relative_path.display()),
                content,
                priority: 3,
                truncated: false,
            });
        }

        sections
    }

    fn format_context(&self, sections: Vec<ContentSection>) -> String {
        let mut context = String::new();

        // Header
        context.push_str("# AI Context Generation Report\n\n");
        context.push_str(&format!(
            "Generated on: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        context.push_str(&format!(
            "Repository: {}\n",
            self.config.repo_path.display()
        ));
        context.push_str(&format!("Max tokens: {}\n\n", self.config.max_tokens));

        // Table of contents
        context.push_str("## Table of Contents\n\n");
        for (i, section) in sections.iter().enumerate() {
            context.push_str(&format!("{}. {}", i + 1, section.title));
            if section.truncated {
                context.push_str(" (truncated)");
            }
            context.push('\n');
        }
        context.push('\n');

        // Sections
        for section in sections {
            context.push_str("---\n\n");
            context.push_str(&section.content);
        }

        context
    }
}

```

---

# Source: src/config.rs

```rust
//! Configuration module for the AI Context Generator.
//!
//! This module provides configuration structures and constants for customizing
//! the behavior of the context generation process.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration structure for the AI Context Generator.
///
/// This structure holds all the configuration options that control how the
/// context generation process behaves, including input/output paths, token limits,
/// and scanning options.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::Config;
/// use std::path::PathBuf;
///
/// // Create a default configuration
/// let config = Config::default();
///
/// // Create a custom configuration
/// let custom_config = Config {
///     repo_path: PathBuf::from("./my-project"),
///     max_tokens: 100000,
///     output_file: "custom_context.md".to_string(),
///     include_hidden: true,
///     include_deps: false,
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Path to the repository to analyze.
    ///
    /// This should point to the root directory of the project you want to analyze.
    /// The scanner will recursively process all supported files within this directory.
    pub repo_path: PathBuf,

    /// Maximum number of tokens to include in the generated context.
    ///
    /// This limit helps ensure the generated context fits within LLM token limits.
    /// When the limit is reached, lower priority content will be truncated.
    /// Uses GPT-4 tokenizer for accurate counting.
    pub max_tokens: usize,

    /// Output file path for the generated context.
    ///
    /// The generated markdown context will be written to this file.
    /// If the file already exists, it will be overwritten.
    pub output_file: String,

    /// Whether to include hidden files and directories in the analysis.
    ///
    /// When `true`, files and directories starting with `.` will be included
    /// in the scan (except for those in [`IGNORED_DIRS`]).
    pub include_hidden: bool,

    /// Whether to include external dependency analysis.
    ///
    /// When `true`, the generator will attempt to analyze and include
    /// information about external dependencies from `Cargo.toml`.
    pub include_deps: bool,
}

impl Default for Config {
    /// Creates a default configuration with sensible defaults.
    ///
    /// # Default Values
    ///
    /// - `repo_path`: Current directory (`.`)
    /// - `max_tokens`: 50,000 tokens
    /// - `output_file`: `"repo_context.md"`
    /// - `include_hidden`: `false`
    /// - `include_deps`: `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.max_tokens, 50000);
    /// assert_eq!(config.output_file, "repo_context.md");
    /// ```
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("."),
            max_tokens: 50000,
            output_file: "repo_context.md".to_string(),
            include_hidden: false,
            include_deps: false,
        }
    }
}

/// File extensions that are supported for analysis.
///
/// Currently, the generator supports:
/// - `.rs` - Rust source files (full AST analysis)
/// - `.md` - Markdown documentation files
pub const SUPPORTED_EXTENSIONS: &[&str] = &[".rs", ".md"];

/// Directory names that are automatically ignored during scanning.
///
/// These directories are commonly used for build artifacts, dependencies,
/// or IDE-specific files that don't contain relevant source code.
pub const IGNORED_DIRS: &[&str] = &["target", "node_modules", ".git", ".vscode", ".idea"];

/// File names that are automatically ignored during scanning.
///
/// These files are typically metadata, configuration, or system files
/// that don't contribute meaningful content to the context.
pub const IGNORED_FILES: &[&str] = &["Cargo.lock", ".gitignore", ".DS_Store"];

```

---

# Source: src/scanner.rs

```rust
//! Repository scanning module for the AI Context Generator.
//!
//! This module provides functionality to scan and analyze repository structure,
//! extracting metadata, file information, and project organization.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{Config, IGNORED_DIRS, IGNORED_FILES, SUPPORTED_EXTENSIONS};

/// Information about a single file in the repository.
///
/// Contains both metadata and content for files that are included in the analysis.
/// This structure is used to pass file information between scanning and generation phases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Absolute path to the file on the filesystem.
    pub path: PathBuf,

    /// Path relative to the repository root.
    ///
    /// This is used for display purposes in the generated context.
    pub relative_path: PathBuf,

    /// Complete content of the file as a string.
    ///
    /// For text files, this contains the entire file content.
    /// Binary files are not processed and won't appear in scan results.
    pub content: String,

    /// Type classification of the file based on its extension.
    pub file_type: FileType,

    /// Size of the file in bytes.
    pub size: u64,
}

/// Classification of file types supported by the generator.
///
/// Different file types receive different processing and priority levels
/// during context generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    /// Rust source files (`.rs` extension).
    ///
    /// These files receive full AST analysis to extract structural information
    /// about modules, functions, structs, enums, and implementations.
    Rust,

    /// Markdown documentation files (`.md` extension).
    ///
    /// These files are included as high-priority documentation content.
    Markdown,
}

/// Complete result of repository scanning operation.
///
/// Contains all information gathered during the scanning phase, including
/// individual files, project structure, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// List of all files that were processed during scanning.
    ///
    /// Only files with supported extensions that passed filtering are included.
    pub files: Vec<FileInfo>,

    /// Structural information about the project organization.
    pub project_structure: ProjectStructure,

    /// Metadata extracted from project configuration files.
    pub metadata: ProjectMetadata,
}

/// Information about the overall structure and organization of the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    /// String representation of the project's file tree.
    ///
    /// This is formatted as a text-based tree structure suitable for
    /// inclusion in markdown documentation.
    pub tree: String,

    /// Total number of files that were processed.
    pub total_files: usize,

    /// Combined size of all processed files in bytes.
    pub total_size: u64,
}

/// Project metadata extracted from configuration files and repository structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Name of the project.
    ///
    /// Extracted from `Cargo.toml` if available, otherwise derived from
    /// the repository directory name.
    pub name: String,

    /// Project description, if available.
    ///
    /// Extracted from `Cargo.toml` description field or README.md content.
    pub description: Option<String>,

    /// List of main dependencies.
    ///
    /// Extracted from the `[dependencies]` section of `Cargo.toml`.
    pub dependencies: Vec<String>,

    /// Rust version or project version.
    ///
    /// Extracted from `Cargo.toml` version field.
    pub rust_version: Option<String>,
}

/// Repository scanner that processes project files and structure.
///
/// The scanner walks through the repository directory, identifies relevant files,
/// extracts their content, and gathers project metadata.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, RepositoryScanner};
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config {
///     repo_path: PathBuf::from("."),
///     max_tokens: 50000,
///     output_file: "context.md".to_string(),
///     include_hidden: false,
///     include_deps: true,
/// };
///
/// let scanner = RepositoryScanner::new(config);
/// let scan_result = scanner.scan().await?;
///
/// println!("Found {} files", scan_result.files.len());
/// # Ok(())
/// # }
/// ```
pub struct RepositoryScanner {
    config: Config,
}

impl RepositoryScanner {
    /// Creates a new repository scanner with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying scanning behavior and output options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Performs a complete scan of the repository.
    ///
    /// This method walks through the repository directory structure, processes
    /// all supported files, extracts project metadata, and builds a comprehensive
    /// scan result.
    ///
    /// # Returns
    ///
    /// A `ScanResult` containing all discovered files, project structure, and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The repository path doesn't exist or isn't accessible
    /// - File system errors occur during scanning
    /// - Files can't be read or parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// let result = scanner.scan().await?;
    ///
    /// println!("Scanned {} files", result.files.len());
    /// println!("Total size: {} bytes", result.project_structure.total_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn scan(&self) -> Result<ScanResult> {
        let mut files = Vec::new();
        let mut total_size = 0u64;

        for entry in WalkDir::new(&self.config.repo_path)
            .into_iter()
            .filter_entry(|e| self.should_include_path(e.path()))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_info) = self.process_file(path).await? {
                    total_size += file_info.size;
                    files.push(file_info);
                }
            }
        }

        let project_structure = self.build_project_structure(&files, total_size)?;
        let metadata = self.extract_project_metadata().await?;

        Ok(ScanResult {
            files,
            project_structure,
            metadata,
        })
    }

    /// Determines whether a path should be included in the scan.
    ///
    /// This method applies filtering rules based on the configuration and
    /// predefined ignore lists to determine if a file or directory should
    /// be processed.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to evaluate for inclusion
    ///
    /// # Returns
    ///
    /// `true` if the path should be included, `false` otherwise
    fn should_include_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Ignore hidden directories if not configured to include them
        if !self.config.include_hidden && path_str.contains("/.") {
            return false;
        }

        // Ignore specific directories
        for ignored_dir in IGNORED_DIRS {
            if path_str.contains(ignored_dir) {
                return false;
            }
        }

        // If it's a file, check if it's supported
        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            // Ignore specific files
            if IGNORED_FILES.contains(&filename.as_ref()) {
                return false;
            }

            // Check extension
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                return SUPPORTED_EXTENSIONS.contains(&ext_str.as_str());
            }

            return false;
        }

        true
    }

    /// Processes a single file and extracts its information.
    ///
    /// Reads the file content, determines its type based on extension,
    /// and creates a `FileInfo` structure with all relevant metadata.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to process
    ///
    /// # Returns
    ///
    /// `Some(FileInfo)` if the file was successfully processed and should be included,
    /// `None` if the file should be skipped
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or metadata cannot be accessed
    async fn process_file(&self, path: &Path) -> Result<Option<FileInfo>> {
        let content = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;

        let file_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("md") => FileType::Markdown,
            _ => return Ok(None),
        };

        let relative_path = path
            .strip_prefix(&self.config.repo_path)
            .unwrap_or(path)
            .to_path_buf();

        Ok(Some(FileInfo {
            path: path.to_path_buf(),
            relative_path,
            content,
            file_type,
            size: metadata.len(),
        }))
    }

    fn build_project_structure(
        &self,
        files: &[FileInfo],
        total_size: u64,
    ) -> Result<ProjectStructure> {
        let mut tree = String::new();
        let mut paths: Vec<_> = files.iter().map(|f| &f.relative_path).collect();
        paths.sort();

        tree.push_str("```\n");
        for (i, path) in paths.iter().enumerate() {
            let depth = path.components().count() - 1;
            let indent = "│   ".repeat(depth);
            let connector = if i == paths.len() - 1 {
                "└── "
            } else {
                "├── "
            };

            tree.push_str(&format!("{}{}{}\n", indent, connector, path.display()));
        }
        tree.push_str("```\n");

        Ok(ProjectStructure {
            tree,
            total_files: files.len(),
            total_size,
        })
    }

    async fn extract_project_metadata(&self) -> Result<ProjectMetadata> {
        let cargo_toml_path = self.config.repo_path.join("Cargo.toml");
        let readme_path = self.config.repo_path.join("README.md");

        let mut metadata = ProjectMetadata {
            name: self
                .config
                .repo_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            description: None,
            dependencies: Vec::new(),
            rust_version: None,
        };

        // Extract information from Cargo.toml
        if cargo_toml_path.exists() {
            let cargo_content = fs::read_to_string(&cargo_toml_path)?;
            self.parse_cargo_toml(&cargo_content, &mut metadata)?;
        }

        // Extract description from README.md
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            metadata.description = self.extract_description_from_readme(&readme_content);
        }

        Ok(metadata)
    }

    fn parse_cargo_toml(&self, content: &str, metadata: &mut ProjectMetadata) -> Result<()> {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_package = false;
        let mut in_dependencies = false;

        for line in lines {
            let line = line.trim();

            if line.starts_with("[package]") {
                in_package = true;
                in_dependencies = false;
                continue;
            }

            if line.starts_with("[dependencies") {
                in_package = false;
                in_dependencies = true;
                continue;
            }

            if line.starts_with("[") {
                in_package = false;
                in_dependencies = false;
                continue;
            }

            if in_package {
                if line.starts_with("name") {
                    if let Some(name) = line.split('=').nth(1) {
                        metadata.name = name.trim().trim_matches('"').to_string();
                    }
                } else if line.starts_with("version") {
                    if let Some(version) = line.split('=').nth(1) {
                        metadata.rust_version = Some(version.trim().trim_matches('"').to_string());
                    }
                }
            }

            if in_dependencies && !line.is_empty() {
                if let Some(dep_name) = line.split('=').next() {
                    metadata.dependencies.push(dep_name.trim().to_string());
                }
            }
        }

        Ok(())
    }

    fn extract_description_from_readme(&self, content: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut description = String::new();

        for line in lines.iter().take(10) {
            if line.starts_with('#') {
                continue;
            }

            if !line.trim().is_empty() {
                description.push_str(line);
                description.push('\n');

                if description.len() > 200 {
                    break;
                }
            }
        }

        if description.trim().is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        }
    }
}

```

---

# Source: src/lib.rs

```rust
//! # AI Context Generator - Rust Library
//!
//! A Rust library for generating structured context from code repositories,
//! specifically designed for LLMs and AI agents. This library provides both
//! simple convenience functions and advanced APIs for fine-grained control.
//!
//! ## When to Use This Library
//!
//! - **Integrate context generation into your Rust applications**
//! - **Build custom analysis workflows**
//! - **Create automated documentation systems**
//! - **Develop AI-powered developer tools**
//!
//! For standalone command-line usage, consider using the CLI tool instead.
//!
//! ## Features
//!
//! - 🔍 **Complete Scanning**: Analyzes all `.rs` and `.md` files in repositories
//! - 🌳 **AST Analysis**: Extracts structures, functions, enums and implementations
//! - 📊 **Token Control**: Respects limits and prioritizes important content
//! - 📁 **Project Structure**: Generates file tree visualizations
//! - 📖 **Documentation**: Includes markdown files and code documentation
//! - ⚡ **Async Processing**: Non-blocking, high-performance analysis
//!
//! ## Quick Start
//!
//! ### Simple Usage
//!
//! ```rust
//! use ai_context_gen::generate_context;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Generate context for current directory
//!     generate_context(PathBuf::from("."), "context.md".to_string()).await?;
//!     println!("Context generated successfully!");
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced Usage with Configuration
//!
//! ```rust
//! use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Custom configuration
//!     let config = Config {
//!         repo_path: PathBuf::from("./my-project"),
//!         max_tokens: 100000,
//!         output_file: "detailed_context.md".to_string(),
//!         include_hidden: true,
//!         include_deps: true,
//!     };
//!
//!     // Step-by-step process for more control
//!     let scanner = RepositoryScanner::new(config.clone());
//!     let scan_result = scanner.scan().await?;
//!     
//!     println!("Found {} files", scan_result.files.len());
//!     
//!     let generator = ContextGenerator::new(config);
//!     generator.generate_context(scan_result).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Using the Configuration Function
//!
//! ```rust
//! use ai_context_gen::{Config, generate_context_with_config};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config {
//!         repo_path: PathBuf::from("/path/to/analyze"),
//!         max_tokens: 75000,
//!         output_file: "analysis.md".to_string(),
//!         include_hidden: false,
//!         include_deps: true,
//!     };
//!
//!     generate_context_with_config(config).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## API Overview
//!
//! - [`generate_context`]: Simple function for basic use cases
//! - [`generate_context_with_config`]: Function with custom configuration
//! - [`Config`]: Configuration structure for all options
//! - [`RepositoryScanner`]: File scanning and discovery
//! - [`ContextGenerator`]: Context generation with priorities
//! - [`RustParser`]: Rust code AST analysis
//!
//! ## Integration Patterns
//!
//! ### Web Applications
//!
//! ```rust,no_run
//! use ai_context_gen::{Config, generate_context_with_config};
//! use std::path::PathBuf;
//!
//! async fn analyze_repo_endpoint(repo_path: String) -> Result<String, Box<dyn std::error::Error>> {
//!     let config = Config {
//!         repo_path: PathBuf::from(repo_path),
//!         max_tokens: 50000,
//!         output_file: format!("/tmp/analysis_{}.md", chrono::Utc::now().timestamp()),
//!         include_hidden: false,
//!         include_deps: false,
//!     };
//!     
//!     generate_context_with_config(config.clone()).await?;
//!     Ok(config.output_file)
//! }
//! ```
//!
//! ### Custom Workflows
//!
//! ```rust,no_run
//! use ai_context_gen::{Config, RepositoryScanner, ContextGenerator};
//! use std::path::PathBuf;
//!
//! async fn custom_analysis_workflow(repo_path: PathBuf) -> anyhow::Result<()> {
//!     let config = Config {
//!         repo_path: repo_path.clone(),
//!         max_tokens: 100000,
//!         output_file: "temp_analysis.md".to_string(),
//!         include_hidden: true,
//!         include_deps: true,
//!     };
//!
//!     // Scan first
//!     let scanner = RepositoryScanner::new(config.clone());
//!     let scan_result = scanner.scan().await?;
//!     
//!     // Custom filtering or processing here
//!     println!("Found {} Rust files", scan_result.files.iter()
//!         .filter(|f| matches!(f.file_type, ai_context_gen::FileType::Rust))
//!         .count());
//!     
//!     // Generate context
//!     let generator = ContextGenerator::new(config);
//!     generator.generate_context(scan_result).await?;
//!     
//!     Ok(())
//! }
//! ```

use std::path::PathBuf;

pub mod config;
pub mod generator;
pub mod parser;
pub mod scanner;
pub mod token_counter;

// Re-export main structs for easier usage
pub use config::Config;
pub use generator::ContextGenerator;
pub use parser::{EnumInfo, FunctionInfo, ImplInfo, RustAnalysis, RustParser, StructInfo};
pub use scanner::{FileInfo, FileType, RepositoryScanner, ScanResult};
pub use token_counter::{ContentPrioritizer, ContentSection, TokenCounter};

/// Default Result type used by the library
pub type Result<T> = anyhow::Result<T>;

/// Generates repository context with default configuration
///
/// This is a convenience function that configures and executes
/// the entire context generation process.
///
/// # Arguments
///
/// * `path` - Path to the repository
/// * `output` - Output file name
///
/// # Example
///
/// ```rust
/// use ai_context_gen::generate_context;
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// generate_context(PathBuf::from("."), "context.md".to_string()).await?;
/// # Ok(())
/// # }
/// ```
pub async fn generate_context(path: PathBuf, output: String) -> Result<()> {
    let config = Config {
        repo_path: path,
        max_tokens: 50000,
        output_file: output,
        include_hidden: false,
        include_deps: false,
    };

    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    Ok(())
}

/// Generates repository context with custom configuration
///
/// # Arguments
///
/// * `config` - Custom configuration
///
/// # Example
///
/// ```rust
/// use ai_context_gen::{Config, generate_context_with_config};
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config {
///     repo_path: PathBuf::from("./my-project"),
///     max_tokens: 100000,
///     output_file: "detailed_context.md".to_string(),
///     include_hidden: true,
///     include_deps: true,
/// };
///
/// generate_context_with_config(config).await?;
/// # Ok(())
/// # }
/// ```
pub async fn generate_context_with_config(config: Config) -> Result<()> {
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    Ok(())
}

```

---

# Source: src/parser.rs

```rust
//! Rust AST parsing module for the AI Context Generator.
//!
//! This module provides functionality to parse Rust source code and extract
//! structural information such as modules, functions, structs, enums, and
//! implementations using the `syn` crate.

use anyhow::Result;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{parse_file, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, Signature};

/// Complete analysis result for a single Rust source file.
///
/// Contains all structural information extracted from parsing the file's AST,
/// including modules, functions, structs, enums, and implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustAnalysis {
    /// Path to the analyzed file.
    pub file_path: String,

    /// List of modules defined in the file.
    pub modules: Vec<ModuleInfo>,

    /// List of functions defined in the file.
    pub functions: Vec<FunctionInfo>,

    /// List of structs defined in the file.
    pub structs: Vec<StructInfo>,

    /// List of enums defined in the file.
    pub enums: Vec<EnumInfo>,

    /// List of impl blocks defined in the file.
    pub implementations: Vec<ImplInfo>,

    /// Summary of the AST structure.
    pub ast_summary: String,
}

/// Information about a module definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Name of the module.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// Number of items contained in the module.
    pub items_count: usize,
}

/// Information about a function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Name of the function.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// Whether the function is async.
    pub is_async: bool,

    /// List of parameter types as strings.
    pub parameters: Vec<String>,

    /// Return type as a string, if any.
    pub return_type: Option<String>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about a struct definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructInfo {
    /// Name of the struct.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// List of fields in the struct.
    pub fields: Vec<FieldInfo>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about a struct field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// Name of the field.
    pub name: String,

    /// Type of the field as a string.
    pub field_type: String,

    /// Visibility modifier for the field.
    pub visibility: String,
}

/// Information about an enum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumInfo {
    /// Name of the enum.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// List of variant names.
    pub variants: Vec<String>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about an implementation block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplInfo {
    /// The type being implemented (e.g., "MyStruct", "Vec<T>").
    pub target: String,

    /// Name of the trait being implemented, if this is a trait impl.
    pub trait_name: Option<String>,

    /// List of methods defined in the implementation.
    pub methods: Vec<FunctionInfo>,
}

/// Rust source code parser using syn for AST analysis.
///
/// Provides static methods for parsing Rust source files and extracting
/// structural information about the code.
pub struct RustParser;

impl RustParser {
    /// Parses a Rust source file and extracts structural information.
    ///
    /// This method uses the `syn` crate to parse Rust source code into an AST
    /// and then extracts information about modules, functions, structs, enums,
    /// and implementations.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file being parsed (used for error reporting)
    /// * `content` - Source code content as a string
    ///
    /// # Returns
    ///
    /// A `RustAnalysis` containing all extracted structural information.
    ///
    /// # Errors
    ///
    /// Returns an error if the source code cannot be parsed as valid Rust syntax.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::parser::RustParser;
    ///
    /// let source = r#"
    /// pub struct MyStruct {
    ///     pub field: String,
    /// }
    ///
    /// impl MyStruct {
    ///     pub fn new() -> Self {
    ///         Self { field: String::new() }
    ///     }
    /// }
    /// "#;
    ///
    /// let analysis = RustParser::parse_rust_file("example.rs", source).unwrap();
    /// assert_eq!(analysis.structs.len(), 1);
    /// assert_eq!(analysis.implementations.len(), 1);
    /// ```
    pub fn parse_rust_file(file_path: &str, content: &str) -> Result<RustAnalysis> {
        let syntax_tree = parse_file(content)?;

        let mut analysis = RustAnalysis {
            file_path: file_path.to_string(),
            modules: Vec::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            implementations: Vec::new(),
            ast_summary: String::new(),
        };

        // Analyze each item in the file
        for item in &syntax_tree.items {
            match item {
                Item::Mod(item_mod) => {
                    analysis.modules.push(Self::parse_module(item_mod));
                }
                Item::Fn(item_fn) => {
                    analysis.functions.push(Self::parse_function(item_fn));
                }
                Item::Struct(item_struct) => {
                    analysis.structs.push(Self::parse_struct(item_struct));
                }
                Item::Enum(item_enum) => {
                    analysis.enums.push(Self::parse_enum(item_enum));
                }
                Item::Impl(item_impl) => {
                    analysis.implementations.push(Self::parse_impl(item_impl));
                }
                _ => {}
            }
        }

        analysis.ast_summary = Self::generate_ast_summary(&analysis);

        Ok(analysis)
    }

    fn parse_module(item: &ItemMod) -> ModuleInfo {
        let items_count = item
            .content
            .as_ref()
            .map(|(_, items)| items.len())
            .unwrap_or(0);

        ModuleInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            items_count,
        }
    }

    fn parse_function(item: &ItemFn) -> FunctionInfo {
        let sig = &item.sig;

        FunctionInfo {
            name: sig.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            is_async: sig.asyncness.is_some(),
            parameters: Self::parse_parameters(sig),
            return_type: Self::parse_return_type(sig),
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_struct(item: &ItemStruct) -> StructInfo {
        let fields = match &item.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| FieldInfo {
                    name: f.ident.as_ref().unwrap().to_string(),
                    field_type: f.ty.to_token_stream().to_string(),
                    visibility: Self::parse_visibility(&f.vis),
                })
                .collect(),
            syn::Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| FieldInfo {
                    name: format!("field_{}", i),
                    field_type: f.ty.to_token_stream().to_string(),
                    visibility: Self::parse_visibility(&f.vis),
                })
                .collect(),
            syn::Fields::Unit => Vec::new(),
        };

        StructInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            fields,
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_enum(item: &ItemEnum) -> EnumInfo {
        let variants = item.variants.iter().map(|v| v.ident.to_string()).collect();

        EnumInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            variants,
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_impl(item: &ItemImpl) -> ImplInfo {
        let target = item.self_ty.to_token_stream().to_string();
        let trait_name = item
            .trait_
            .as_ref()
            .map(|(_, path, _)| path.to_token_stream().to_string());

        let methods = item
            .items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Fn(method) = item {
                    Some(FunctionInfo {
                        name: method.sig.ident.to_string(),
                        visibility: Self::parse_visibility(&method.vis),
                        is_async: method.sig.asyncness.is_some(),
                        parameters: Self::parse_parameters(&method.sig),
                        return_type: Self::parse_return_type(&method.sig),
                        documentation: Self::extract_doc_comments(&method.attrs),
                    })
                } else {
                    None
                }
            })
            .collect();

        ImplInfo {
            target,
            trait_name,
            methods,
        }
    }

    fn parse_visibility(vis: &syn::Visibility) -> String {
        match vis {
            syn::Visibility::Public(_) => "pub".to_string(),
            syn::Visibility::Restricted(restricted) => {
                format!("pub({})", restricted.path.to_token_stream())
            }
            syn::Visibility::Inherited => "private".to_string(),
        }
    }

    fn parse_parameters(sig: &Signature) -> Vec<String> {
        sig.inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_some() {
                        "&mut self".to_string()
                    } else {
                        "&self".to_string()
                    }
                }
                syn::FnArg::Typed(typed) => {
                    format!(
                        "{}: {}",
                        typed.pat.to_token_stream(),
                        typed.ty.to_token_stream()
                    )
                }
            })
            .collect()
    }

    fn parse_return_type(sig: &Signature) -> Option<String> {
        match &sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
        }
    }

    fn extract_doc_comments(attrs: &[syn::Attribute]) -> Option<String> {
        let mut doc_comments = Vec::new();

        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let Ok(syn::Lit::Str(lit_str)) = attr.parse_args() {
                    doc_comments.push(lit_str.value());
                }
            }
        }

        if doc_comments.is_empty() {
            None
        } else {
            Some(doc_comments.join("\n"))
        }
    }

    fn generate_ast_summary(analysis: &RustAnalysis) -> String {
        let mut summary = String::new();

        summary.push_str(&format!("# AST Summary for {}\n\n", analysis.file_path));

        if !analysis.modules.is_empty() {
            summary.push_str("## Modules\n");
            for module in &analysis.modules {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} items\n",
                    module.name, module.visibility, module.items_count
                ));
            }
            summary.push('\n');
        }

        if !analysis.structs.is_empty() {
            summary.push_str("## Structs\n");
            for struct_info in &analysis.structs {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} fields\n",
                    struct_info.name,
                    struct_info.visibility,
                    struct_info.fields.len()
                ));
            }
            summary.push('\n');
        }

        if !analysis.enums.is_empty() {
            summary.push_str("## Enums\n");
            for enum_info in &analysis.enums {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} variants\n",
                    enum_info.name,
                    enum_info.visibility,
                    enum_info.variants.len()
                ));
            }
            summary.push('\n');
        }

        if !analysis.functions.is_empty() {
            summary.push_str("## Functions\n");
            for func in &analysis.functions {
                let async_marker = if func.is_async { "async " } else { "" };
                summary.push_str(&format!(
                    "- `{}{}{}` ({})\n",
                    async_marker,
                    func.name,
                    if func.parameters.is_empty() {
                        "()"
                    } else {
                        "(...)"
                    },
                    func.visibility
                ));
            }
            summary.push('\n');
        }

        if !analysis.implementations.is_empty() {
            summary.push_str("## Implementations\n");
            for impl_info in &analysis.implementations {
                let trait_part = impl_info
                    .trait_name
                    .as_ref()
                    .map(|t| format!("{} for ", t))
                    .unwrap_or_default();
                summary.push_str(&format!("- `impl {}{}`\n", trait_part, impl_info.target));
            }
        }

        summary
    }
}

```

---

# Source: src/token_counter.rs

```rust
//! Token counting and content prioritization module.
//!
//! This module provides functionality for accurate token counting using the GPT-4
//! tokenizer and intelligent content prioritization to fit within token limits.

use anyhow::Result;
use tiktoken_rs::{get_bpe_from_model, CoreBPE};

/// Token counter using the GPT-4 tokenizer for accurate token counting.
///
/// Provides methods for counting tokens in text and truncating content to fit
/// within specified token limits while maintaining text coherence.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::TokenCounter;
///
/// let counter = TokenCounter::new().unwrap();
/// let text = "Hello, world!";
/// let token_count = counter.count_tokens(text);
/// println!("Text has {} tokens", token_count);
/// ```
pub struct TokenCounter {
    bpe: CoreBPE,
}

impl TokenCounter {
    /// Creates a new token counter using the GPT-4 tokenizer.
    ///
    /// # Returns
    ///
    /// A new `TokenCounter` instance configured with the GPT-4 BPE tokenizer.
    ///
    /// # Errors
    ///
    /// Returns an error if the GPT-4 tokenizer model cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        let bpe = get_bpe_from_model("gpt-4")?;
        Ok(Self { bpe })
    }

    /// Counts the number of tokens in the given text.
    ///
    /// Uses the GPT-4 tokenizer to provide accurate token counts that match
    /// what would be used by OpenAI's models and similar systems.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to count tokens for
    ///
    /// # Returns
    ///
    /// The number of tokens in the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// let count = counter.count_tokens("Hello, world!");
    /// assert!(count > 0);
    /// ```
    pub fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }

    /// Truncates text to fit within a specified token limit.
    ///
    /// Attempts to preserve text coherence by truncating at token boundaries
    /// rather than character boundaries. Falls back to character truncation
    /// if token decoding fails.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to truncate
    /// * `max_tokens` - Maximum number of tokens to include
    ///
    /// # Returns
    ///
    /// The truncated text that fits within the token limit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// let long_text = "This is a very long text that exceeds the token limit...";
    /// let truncated = counter.truncate_to_token_limit(long_text, 10);
    /// assert!(counter.count_tokens(&truncated) <= 10);
    /// ```
    pub fn truncate_to_token_limit(&self, text: &str, max_tokens: usize) -> String {
        let tokens = self.bpe.encode_with_special_tokens(text);

        if tokens.len() <= max_tokens {
            return text.to_string();
        }

        let truncated_tokens = &tokens[..max_tokens];
        match self.bpe.decode(truncated_tokens.to_vec()) {
            Ok(truncated_text) => truncated_text,
            Err(_) => {
                // Fallback: truncate by characters
                let char_limit = (text.len() * max_tokens) / tokens.len();
                text.chars().take(char_limit).collect()
            }
        }
    }
}

/// Content prioritizer that manages sections based on priority and token limits.
///
/// The prioritizer sorts content sections by priority and ensures the total
/// content fits within specified token limits by truncating lower priority
/// sections when necessary.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::{ContentPrioritizer, ContentSection};
///
/// let prioritizer = ContentPrioritizer::new().unwrap();
/// let sections = vec![
///     ContentSection::new("High Priority".to_string(), "Content...".to_string(), 10),
///     ContentSection::new("Low Priority".to_string(), "More content...".to_string(), 1),
/// ];
/// let prioritized = prioritizer.prioritize_content(sections, 1000);
/// ```
pub struct ContentPrioritizer {
    token_counter: TokenCounter,
}

impl ContentPrioritizer {
    /// Creates a new content prioritizer.
    ///
    /// # Returns
    ///
    /// A new `ContentPrioritizer` instance with an initialized token counter.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying token counter cannot be initialized.
    pub fn new() -> Result<Self> {
        Ok(Self {
            token_counter: TokenCounter::new()?,
        })
    }

    /// Prioritizes and truncates content sections to fit within token limits.
    ///
    /// Sorts sections by priority (highest first) and includes as many complete
    /// sections as possible. When a section would exceed the token limit, it
    /// attempts to truncate it if there are sufficient remaining tokens.
    ///
    /// # Arguments
    ///
    /// * `sections` - List of content sections to prioritize
    /// * `max_tokens` - Maximum total tokens allowed
    ///
    /// # Returns
    ///
    /// A vector of sections that fit within the token limit, sorted by priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::{ContentPrioritizer, ContentSection};
    ///
    /// let prioritizer = ContentPrioritizer::new().unwrap();
    /// let sections = vec![
    ///     ContentSection::new("Important".to_string(), "Critical info".to_string(), 10),
    ///     ContentSection::new("Less Important".to_string(), "Extra details".to_string(), 5),
    /// ];
    /// let result = prioritizer.prioritize_content(sections, 100);
    /// // Higher priority sections appear first
    /// ```
    pub fn prioritize_content(
        &self,
        sections: Vec<ContentSection>,
        max_tokens: usize,
    ) -> Vec<ContentSection> {
        let mut prioritized = sections;

        // Sort by priority (highest priority first)
        prioritized.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut total_tokens = 0;
        let mut result = Vec::new();

        for mut section in prioritized {
            let section_tokens = self.token_counter.count_tokens(&section.content);

            if total_tokens + section_tokens <= max_tokens {
                total_tokens += section_tokens;
                result.push(section);
            } else {
                // Try to truncate content to fit within the limit
                let remaining_tokens = max_tokens - total_tokens;
                if remaining_tokens > 100 {
                    // Only include if at least 100 tokens remain
                    section.content = self
                        .token_counter
                        .truncate_to_token_limit(&section.content, remaining_tokens);
                    section.truncated = true;
                    result.push(section);
                    break;
                }
            }
        }

        result
    }
}

/// A content section with associated metadata for prioritization.
///
/// Represents a section of content (like project metadata, source code, or
/// documentation) with a title, content, priority level, and truncation status.
///
/// # Priority Levels
///
/// - `9-10`: High priority (metadata, documentation)
/// - `5-6`: Medium priority (AST analysis, structure)
/// - `1-2`: Low priority (source code)
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::ContentSection;
///
/// // Create a high-priority section
/// let section = ContentSection::high_priority(
///     "Project Metadata".to_string(),
///     "Important project information...".to_string()
/// );
///
/// // Create a custom priority section
/// let custom = ContentSection::new(
///     "Custom Section".to_string(),
///     "Content here...".to_string(),
///     7
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ContentSection {
    /// Title of the content section.
    pub title: String,

    /// The actual content of the section.
    pub content: String,

    /// Priority level (higher numbers = higher priority).
    pub priority: u8,

    /// Whether this section was truncated to fit token limits.
    pub truncated: bool,
}

impl ContentSection {
    /// Creates a new content section with the specified priority.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    /// * `priority` - Priority level (0-255, higher is more important)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::ContentSection;
    ///
    /// let section = ContentSection::new(
    ///     "My Section".to_string(),
    ///     "Section content...".to_string(),
    ///     8
    /// );
    /// ```
    pub fn new(title: String, content: String, priority: u8) -> Self {
        Self {
            title,
            content,
            priority,
            truncated: false,
        }
    }

    /// Creates a high-priority content section (priority 9).
    ///
    /// Use for critical content like project metadata and documentation
    /// that should always be included.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn high_priority(title: String, content: String) -> Self {
        Self::new(title, content, 9)
    }

    /// Creates a medium-priority content section (priority 5).
    ///
    /// Use for structural information like AST analysis and project organization.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn medium_priority(title: String, content: String) -> Self {
        Self::new(title, content, 5)
    }

    /// Creates a low-priority content section (priority 1).
    ///
    /// Use for detailed content like complete source code that can be
    /// truncated if necessary.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn low_priority(title: String, content: String) -> Self {
        Self::new(title, content, 1)
    }
}

```

---

# Source: README.md

```markdown
# AI Context Generator

[![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
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

- Rust 1.70 or higher
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
ai-context-gen = "0.1.1"
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

## 📄 License

This project is licensed under the MIT license. See the `LICENSE` file for details.

## 🛤️ Roadmap

- [ ] Web interface
- [ ] Git integration
- [ ] Commit history analysis
- [ ] Support for other output formats (JSON, YAML)
- [ ] Cache for better performance

## 📝 Changelog

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

```

---

# Source: examples/advanced_usage.rs

```rust
// Advanced example showing direct usage of structures
// To run: cargo run --example advanced_usage

use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 AI Context Generator - Advanced Example");
    println!("===========================================");

    // Custom configuration
    let config = Config {
        repo_path: PathBuf::from("."),
        max_tokens: 100000,
        output_file: "advanced_context.md".to_string(),
        include_hidden: true,
        include_deps: true,
    };

    println!("📁 Analyzing project: {:?}", config.repo_path);
    println!("🔍 Including hidden files: {}", config.include_hidden);
    println!("📦 Including dependencies: {}", config.include_deps);

    // Step 1: Scanning
    println!("\n🔍 Step 1: Scanning files...");
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    println!("📄 Files found: {}", scan_result.files.len());
    println!("📁 Project: {}", scan_result.metadata.name);

    if let Some(description) = &scan_result.metadata.description {
        println!("📝 Description: {}", description);
    }

    // List found files
    println!("\n📋 Files to be analyzed:");
    for file in &scan_result.files {
        println!(
            "  - {} ({:?})",
            file.relative_path.display(),
            file.file_type
        );
    }

    // Step 2: Context generation
    println!("\n🏗️  Step 2: Generating context...");
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    println!("✅ Advanced context generated successfully!");
    println!("📖 Check the 'advanced_context.md' file");

    Ok(())
}

```

---

# Source: examples/basic_usage.rs

```rust
// Basic usage example of the ai-context-gen library
// To run: cargo run --example basic_usage

use ai_context_gen::{generate_context_with_config, Config};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🚀 AI Context Generator - Basic Example");
    println!("========================================");

    // Basic configuration
    let config = Config {
        repo_path: PathBuf::from("."),
        max_tokens: 30000,
        output_file: "example_context.md".to_string(),
        include_hidden: false,
        include_deps: false,
    };

    println!("📁 Analyzing project: {:?}", config.repo_path);
    println!("📊 Token limit: {}", config.max_tokens);
    println!("📄 Output file: {}", config.output_file);

    // Generate context
    match generate_context_with_config(config).await {
        Ok(()) => {
            println!("✅ Context generated successfully!");
            println!("📖 Check the 'example_context.md' file");
        }
        Err(e) => {
            eprintln!("❌ Error generating context: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

```


---

# Documentation: README.md

# AI Context Generator

[![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
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

- Rust 1.70 or higher
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
ai-context-gen = "0.1.1"
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

## 📄 License

This project is licensed under the MIT license. See the `LICENSE` file for details.

## 🛤️ Roadmap

- [ ] Web interface
- [ ] Git integration
- [ ] Commit history analysis
- [ ] Support for other output formats (JSON, YAML)
- [ ] Cache for better performance

## 📝 Changelog

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

---

# Rust Analysis: src/main.rs

## Functions
- **main**() -> Result < () > (private)

## Structs
- **Args**: 5 fields (private)

---

# Rust Analysis: src/generator.rs

## Structs
- **ContextGenerator**: 2 fields (pub)

## Implementations
- **impl ContextGenerator**: 8 methods

---

# Rust Analysis: src/config.rs

## Structs
- **Config**: 5 fields (pub)

## Implementations
- **impl Config**: 1 methods

---

# Rust Analysis: src/scanner.rs

## Structs
- **FileInfo**: 5 fields (pub)
- **ScanResult**: 3 fields (pub)
- **ProjectStructure**: 3 fields (pub)
- **ProjectMetadata**: 4 fields (pub)
- **RepositoryScanner**: 1 fields (pub)

## Enums
- **FileType**: 2 variants (pub)

## Implementations
- **impl RepositoryScanner**: 8 methods

---

# Rust Analysis: src/lib.rs

## Modules
- **config**: pub
- **generator**: pub
- **parser**: pub
- **scanner**: pub
- **token_counter**: pub

## Functions
- **generate_context**(path: PathBuf, output: String) -> Result < () > (pub)
- **generate_context_with_config**(config: Config) -> Result < () > (pub)

---

# Rust Analysis: src/parser.rs

## Structs
- **RustAnalysis**: 7 fields (pub)
- **ModuleInfo**: 3 fields (pub)
- **FunctionInfo**: 6 fields (pub)
- **StructInfo**: 4 fields (pub)
- **FieldInfo**: 3 fields (pub)
- **EnumInfo**: 4 fields (pub)
- **ImplInfo**: 3 fields (pub)
- **RustParser**: 0 fields (pub)

## Implementations
- **impl RustParser**: 11 methods

---

# Rust Analysis: src/token_counter.rs

## Structs
- **TokenCounter**: 1 fields (pub)
- **ContentPrioritizer**: 1 fields (pub)
- **ContentSection**: 4 fields (pub)

## Implementations
- **impl TokenCounter**: 3 methods
- **impl ContentPrioritizer**: 2 methods
- **impl ContentSection**: 4 methods

---

# Rust Analysis: examples/advanced_usage.rs

## Functions
- **main**() -> anyhow :: Result < () > (private)

---

# Rust Analysis: examples/basic_usage.rs

## Functions
- **main**() -> anyhow :: Result < () > (private)

---

# Source: src/main.rs

```rust
//! AI Context Generator CLI application.
//!
//! Command-line interface for generating structured context from Rust repositories.
//! This tool scans repository files, analyzes Rust code structure, and generates
//! markdown context suitable for LLMs and AI agents.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};

/// Command-line arguments for the AI Context Generator.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the repository to analyze
    ///
    /// Specifies the root directory of the project to scan. The tool will
    /// recursively process all supported files within this directory.
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Maximum number of tokens to include in the output
    ///
    /// Controls the size of the generated context to fit within LLM token limits.
    /// Content is prioritized and truncated as needed to stay within this limit.
    #[arg(short, long, default_value = "50000")]
    max_tokens: usize,

    /// Output file name for the generated context
    ///
    /// The markdown file where the generated context will be written.
    /// If the file exists, it will be overwritten.
    #[arg(short, long, default_value = "repo_context.md")]
    output: String,

    /// Include hidden files and directories in the analysis
    ///
    /// When enabled, files and directories starting with '.' will be included
    /// in the scan (except for those in the ignore list).
    #[arg(long)]
    include_hidden: bool,

    /// Include analysis of external dependencies
    ///
    /// When enabled, the tool will attempt to analyze and include information
    /// about external dependencies from Cargo.toml.
    #[arg(long)]
    include_deps: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config {
        repo_path: args.path,
        max_tokens: args.max_tokens,
        output_file: args.output,
        include_hidden: args.include_hidden,
        include_deps: args.include_deps,
    };

    println!("🔍 Scanning repository...");
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    println!("📝 Generating context...");
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    println!("✅ Context generated successfully!");
    Ok(())
}

```

---

# Source: src/generator.rs

```rust
//! Context generation module for the AI Context Generator.
//!
//! This module provides functionality to generate structured markdown context
//! from scanned repository data, with intelligent content prioritization and
//! token limit management.

use anyhow::Result;
use chrono::Utc;
use std::fs;

use crate::config::Config;
use crate::parser::RustParser;
use crate::scanner::{FileType, ScanResult};
use crate::token_counter::{ContentPrioritizer, ContentSection};

/// Context generator that creates structured markdown from repository scan results.
///
/// The generator takes scan results and creates a prioritized, token-limited markdown
/// document suitable for consumption by LLMs and AI agents. Content is organized by
/// priority, with metadata and documentation receiving higher priority than source code.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::default();
/// let scanner = RepositoryScanner::new(config.clone());
/// let scan_result = scanner.scan().await?;
///
/// let generator = ContextGenerator::new(config);
/// generator.generate_context(scan_result).await?;
/// # Ok(())
/// # }
/// ```
pub struct ContextGenerator {
    config: Config,
    prioritizer: ContentPrioritizer,
}

impl ContextGenerator {
    /// Creates a new context generator with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying output options and token limits
    ///
    /// # Panics
    ///
    /// Panics if the content prioritizer cannot be initialized (e.g., if the
    /// tiktoken model cannot be loaded).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, ContextGenerator};
    ///
    /// let config = Config::default();
    /// let generator = ContextGenerator::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config,
            prioritizer: ContentPrioritizer::new()
                .expect("Failed to initialize content prioritizer"),
        }
    }

    /// Generates a complete context document from scan results.
    ///
    /// This method creates a structured markdown document with prioritized content
    /// sections including project metadata, file structure, documentation, AST
    /// analysis, and source code. Content is prioritized and truncated based on
    /// the configured token limit.
    ///
    /// # Arguments
    ///
    /// * `scan_result` - Results from repository scanning containing files and metadata
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the context was successfully generated and written to the
    /// configured output file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - AST parsing fails for Rust files
    /// - The output file cannot be written
    /// - Token counting or content prioritization fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config.clone());
    /// let scan_result = scanner.scan().await?;
    ///
    /// let generator = ContextGenerator::new(config);
    /// generator.generate_context(scan_result).await?;
    ///
    /// println!("Context generated successfully!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_context(&self, scan_result: ScanResult) -> Result<()> {
        let mut sections = Vec::new();

        // Project metadata section (high priority)
        sections.push(self.create_metadata_section(&scan_result));

        // Project structure section (high priority)
        sections.push(self.create_structure_section(&scan_result));

        // Markdown documentation sections (high priority)
        sections.extend(self.create_markdown_sections(&scan_result));

        // AST analysis sections for Rust files (medium priority)
        sections.extend(self.create_rust_analysis_sections(&scan_result).await?);

        // Source code sections (low priority)
        sections.extend(self.create_source_code_sections(&scan_result));

        // Prioritize and truncate content based on token limit
        let final_sections = self
            .prioritizer
            .prioritize_content(sections, self.config.max_tokens);

        // Generate final context
        let context = self.format_context(final_sections);

        // Write to file
        fs::write(&self.config.output_file, context)?;

        println!(
            "Context generated successfully in: {}",
            self.config.output_file
        );
        Ok(())
    }

    fn create_metadata_section(&self, scan_result: &ScanResult) -> ContentSection {
        let mut content = String::new();
        content.push_str("# Project Metadata\n\n");
        content.push_str(&format!("**Name:** {}\n", scan_result.metadata.name));

        if let Some(description) = &scan_result.metadata.description {
            content.push_str(&format!("**Description:** {}\n", description));
        }

        if !scan_result.metadata.dependencies.is_empty() {
            content.push_str("**Dependencies:**\n");
            for dep in &scan_result.metadata.dependencies {
                content.push_str(&format!("- {}\n", dep));
            }
        }

        if let Some(rust_version) = &scan_result.metadata.rust_version {
            content.push_str(&format!("**Version:** {}\n", rust_version));
        }

        content.push_str(&format!(
            "**Total files:** {}\n",
            scan_result.project_structure.total_files
        ));
        content.push_str(&format!(
            "**Total size:** {} bytes\n\n",
            scan_result.project_structure.total_size
        ));

        ContentSection {
            title: "Project Metadata".to_string(),
            content,
            priority: 10,
            truncated: false,
        }
    }

    fn create_structure_section(&self, scan_result: &ScanResult) -> ContentSection {
        let mut content = String::new();
        content.push_str("# Project Structure\n\n");
        content.push_str(&scan_result.project_structure.tree);
        content.push('\n');

        ContentSection {
            title: "Project Structure".to_string(),
            content,
            priority: 9,
            truncated: false,
        }
    }

    fn create_markdown_sections(&self, scan_result: &ScanResult) -> Vec<ContentSection> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            if matches!(file.file_type, FileType::Markdown) {
                let mut content = String::new();
                content.push_str(&format!(
                    "# Documentation: {}\n\n",
                    file.relative_path.display()
                ));
                content.push_str(&file.content);
                content.push('\n');

                sections.push(ContentSection {
                    title: format!("Documentation: {}", file.relative_path.display()),
                    content,
                    priority: 8,
                    truncated: false,
                });
            }
        }

        sections
    }

    async fn create_rust_analysis_sections(
        &self,
        scan_result: &ScanResult,
    ) -> Result<Vec<ContentSection>> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            if matches!(file.file_type, FileType::Rust) {
                match RustParser::parse_rust_file(&file.path.to_string_lossy(), &file.content) {
                    Ok(analysis) => {
                        let mut content = String::new();
                        content.push_str(&format!(
                            "# Rust Analysis: {}\n\n",
                            file.relative_path.display()
                        ));

                        if !analysis.modules.is_empty() {
                            content.push_str("## Modules\n");
                            for module in &analysis.modules {
                                content.push_str(&format!(
                                    "- **{}**: {}\n",
                                    module.name, module.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.functions.is_empty() {
                            content.push_str("## Functions\n");
                            for function in &analysis.functions {
                                let params = function.parameters.join(", ");
                                let return_type = function.return_type.as_deref().unwrap_or("()");
                                content.push_str(&format!(
                                    "- **{}**({}) -> {} ({})\n",
                                    function.name, params, return_type, function.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.structs.is_empty() {
                            content.push_str("## Structs\n");
                            for struct_info in &analysis.structs {
                                content.push_str(&format!(
                                    "- **{}**: {} fields ({})\n",
                                    struct_info.name,
                                    struct_info.fields.len(),
                                    struct_info.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.enums.is_empty() {
                            content.push_str("## Enums\n");
                            for enum_info in &analysis.enums {
                                content.push_str(&format!(
                                    "- **{}**: {} variants ({})\n",
                                    enum_info.name,
                                    enum_info.variants.len(),
                                    enum_info.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.implementations.is_empty() {
                            content.push_str("## Implementations\n");
                            for impl_info in &analysis.implementations {
                                content.push_str(&format!(
                                    "- **impl {}**: {} methods\n",
                                    impl_info.target,
                                    impl_info.methods.len()
                                ));
                            }
                            content.push('\n');
                        }

                        sections.push(ContentSection {
                            title: format!("Rust Analysis: {}", file.relative_path.display()),
                            content,
                            priority: 6,
                            truncated: false,
                        });
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse {}: {}",
                            file.relative_path.display(),
                            e
                        );
                    }
                }
            }
        }

        Ok(sections)
    }

    fn create_source_code_sections(&self, scan_result: &ScanResult) -> Vec<ContentSection> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            let mut content = String::new();
            content.push_str(&format!("# Source: {}\n\n", file.relative_path.display()));
            content.push_str("```");

            match file.file_type {
                FileType::Rust => content.push_str("rust"),
                FileType::Markdown => content.push_str("markdown"),
            }

            content.push('\n');
            content.push_str(&file.content);
            content.push_str("\n```\n\n");

            sections.push(ContentSection {
                title: format!("Source: {}", file.relative_path.display()),
                content,
                priority: 3,
                truncated: false,
            });
        }

        sections
    }

    fn format_context(&self, sections: Vec<ContentSection>) -> String {
        let mut context = String::new();

        // Header
        context.push_str("# AI Context Generation Report\n\n");
        context.push_str(&format!(
            "Generated on: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        context.push_str(&format!(
            "Repository: {}\n",
            self.config.repo_path.display()
        ));
        context.push_str(&format!("Max tokens: {}\n\n", self.config.max_tokens));

        // Table of contents
        context.push_str("## Table of Contents\n\n");
        for (i, section) in sections.iter().enumerate() {
            context.push_str(&format!("{}. {}", i + 1, section.title));
            if section.truncated {
                context.push_str(" (truncated)");
            }
            context.push('\n');
        }
        context.push('\n');

        // Sections
        for section in sections {
            context.push_str("---\n\n");
            context.push_str(&section.content);
        }

        context
    }
}

```

---

# Source: src/config.rs

```rust
//! Configuration module for the AI Context Generator.
//!
//! This module provides configuration structures and constants for customizing
//! the behavior of the context generation process.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration structure for the AI Context Generator.
///
/// This structure holds all the configuration options that control how the
/// context generation process behaves, including input/output paths, token limits,
/// and scanning options.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::Config;
/// use std::path::PathBuf;
///
/// // Create a default configuration
/// let config = Config::default();
///
/// // Create a custom configuration
/// let custom_config = Config {
///     repo_path: PathBuf::from("./my-project"),
///     max_tokens: 100000,
///     output_file: "custom_context.md".to_string(),
///     include_hidden: true,
///     include_deps: false,
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Path to the repository to analyze.
    ///
    /// This should point to the root directory of the project you want to analyze.
    /// The scanner will recursively process all supported files within this directory.
    pub repo_path: PathBuf,

    /// Maximum number of tokens to include in the generated context.
    ///
    /// This limit helps ensure the generated context fits within LLM token limits.
    /// When the limit is reached, lower priority content will be truncated.
    /// Uses GPT-4 tokenizer for accurate counting.
    pub max_tokens: usize,

    /// Output file path for the generated context.
    ///
    /// The generated markdown context will be written to this file.
    /// If the file already exists, it will be overwritten.
    pub output_file: String,

    /// Whether to include hidden files and directories in the analysis.
    ///
    /// When `true`, files and directories starting with `.` will be included
    /// in the scan (except for those in [`IGNORED_DIRS`]).
    pub include_hidden: bool,

    /// Whether to include external dependency analysis.
    ///
    /// When `true`, the generator will attempt to analyze and include
    /// information about external dependencies from `Cargo.toml`.
    pub include_deps: bool,
}

impl Default for Config {
    /// Creates a default configuration with sensible defaults.
    ///
    /// # Default Values
    ///
    /// - `repo_path`: Current directory (`.`)
    /// - `max_tokens`: 50,000 tokens
    /// - `output_file`: `"repo_context.md"`
    /// - `include_hidden`: `false`
    /// - `include_deps`: `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.max_tokens, 50000);
    /// assert_eq!(config.output_file, "repo_context.md");
    /// ```
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("."),
            max_tokens: 50000,
            output_file: "repo_context.md".to_string(),
            include_hidden: false,
            include_deps: false,
        }
    }
}

/// File extensions that are supported for analysis.
///
/// Currently, the generator supports:
/// - `.rs` - Rust source files (full AST analysis)
/// - `.md` - Markdown documentation files
pub const SUPPORTED_EXTENSIONS: &[&str] = &[".rs", ".md"];

/// Directory names that are automatically ignored during scanning.
///
/// These directories are commonly used for build artifacts, dependencies,
/// or IDE-specific files that don't contain relevant source code.
pub const IGNORED_DIRS: &[&str] = &["target", "node_modules", ".git", ".vscode", ".idea"];

/// File names that are automatically ignored during scanning.
///
/// These files are typically metadata, configuration, or system files
/// that don't contribute meaningful content to the context.
pub const IGNORED_FILES: &[&str] = &["Cargo.lock", ".gitignore", ".DS_Store"];

```

---

# Source: src/scanner.rs

```rust
//! Repository scanning module for the AI Context Generator.
//!
//! This module provides functionality to scan and analyze repository structure,
//! extracting metadata, file information, and project organization.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{Config, IGNORED_DIRS, IGNORED_FILES, SUPPORTED_EXTENSIONS};

/// Information about a single file in the repository.
///
/// Contains both metadata and content for files that are included in the analysis.
/// This structure is used to pass file information between scanning and generation phases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Absolute path to the file on the filesystem.
    pub path: PathBuf,

    /// Path relative to the repository root.
    ///
    /// This is used for display purposes in the generated context.
    pub relative_path: PathBuf,

    /// Complete content of the file as a string.
    ///
    /// For text files, this contains the entire file content.
    /// Binary files are not processed and won't appear in scan results.
    pub content: String,

    /// Type classification of the file based on its extension.
    pub file_type: FileType,

    /// Size of the file in bytes.
    pub size: u64,
}

/// Classification of file types supported by the generator.
///
/// Different file types receive different processing and priority levels
/// during context generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    /// Rust source files (`.rs` extension).
    ///
    /// These files receive full AST analysis to extract structural information
    /// about modules, functions, structs, enums, and implementations.
    Rust,

    /// Markdown documentation files (`.md` extension).
    ///
    /// These files are included as high-priority documentation content.
    Markdown,
}

/// Complete result of repository scanning operation.
///
/// Contains all information gathered during the scanning phase, including
/// individual files, project structure, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// List of all files that were processed during scanning.
    ///
    /// Only files with supported extensions that passed filtering are included.
    pub files: Vec<FileInfo>,

    /// Structural information about the project organization.
    pub project_structure: ProjectStructure,

    /// Metadata extracted from project configuration files.
    pub metadata: ProjectMetadata,
}

/// Information about the overall structure and organization of the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    /// String representation of the project's file tree.
    ///
    /// This is formatted as a text-based tree structure suitable for
    /// inclusion in markdown documentation.
    pub tree: String,

    /// Total number of files that were processed.
    pub total_files: usize,

    /// Combined size of all processed files in bytes.
    pub total_size: u64,
}

/// Project metadata extracted from configuration files and repository structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Name of the project.
    ///
    /// Extracted from `Cargo.toml` if available, otherwise derived from
    /// the repository directory name.
    pub name: String,

    /// Project description, if available.
    ///
    /// Extracted from `Cargo.toml` description field or README.md content.
    pub description: Option<String>,

    /// List of main dependencies.
    ///
    /// Extracted from the `[dependencies]` section of `Cargo.toml`.
    pub dependencies: Vec<String>,

    /// Rust version or project version.
    ///
    /// Extracted from `Cargo.toml` version field.
    pub rust_version: Option<String>,
}

/// Repository scanner that processes project files and structure.
///
/// The scanner walks through the repository directory, identifies relevant files,
/// extracts their content, and gathers project metadata.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, RepositoryScanner};
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config {
///     repo_path: PathBuf::from("."),
///     max_tokens: 50000,
///     output_file: "context.md".to_string(),
///     include_hidden: false,
///     include_deps: true,
/// };
///
/// let scanner = RepositoryScanner::new(config);
/// let scan_result = scanner.scan().await?;
///
/// println!("Found {} files", scan_result.files.len());
/// # Ok(())
/// # }
/// ```
pub struct RepositoryScanner {
    config: Config,
}

impl RepositoryScanner {
    /// Creates a new repository scanner with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying scanning behavior and output options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Performs a complete scan of the repository.
    ///
    /// This method walks through the repository directory structure, processes
    /// all supported files, extracts project metadata, and builds a comprehensive
    /// scan result.
    ///
    /// # Returns
    ///
    /// A `ScanResult` containing all discovered files, project structure, and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The repository path doesn't exist or isn't accessible
    /// - File system errors occur during scanning
    /// - Files can't be read or parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// let result = scanner.scan().await?;
    ///
    /// println!("Scanned {} files", result.files.len());
    /// println!("Total size: {} bytes", result.project_structure.total_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn scan(&self) -> Result<ScanResult> {
        let mut files = Vec::new();
        let mut total_size = 0u64;

        for entry in WalkDir::new(&self.config.repo_path)
            .into_iter()
            .filter_entry(|e| self.should_include_path(e.path()))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_info) = self.process_file(path).await? {
                    total_size += file_info.size;
                    files.push(file_info);
                }
            }
        }

        let project_structure = self.build_project_structure(&files, total_size)?;
        let metadata = self.extract_project_metadata().await?;

        Ok(ScanResult {
            files,
            project_structure,
            metadata,
        })
    }

    /// Determines whether a path should be included in the scan.
    ///
    /// This method applies filtering rules based on the configuration and
    /// predefined ignore lists to determine if a file or directory should
    /// be processed.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to evaluate for inclusion
    ///
    /// # Returns
    ///
    /// `true` if the path should be included, `false` otherwise
    fn should_include_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Ignore hidden directories if not configured to include them
        if !self.config.include_hidden && path_str.contains("/.") {
            return false;
        }

        // Ignore specific directories
        for ignored_dir in IGNORED_DIRS {
            if path_str.contains(ignored_dir) {
                return false;
            }
        }

        // If it's a file, check if it's supported
        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            // Ignore specific files
            if IGNORED_FILES.contains(&filename.as_ref()) {
                return false;
            }

            // Check extension
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                return SUPPORTED_EXTENSIONS.contains(&ext_str.as_str());
            }

            return false;
        }

        true
    }

    /// Processes a single file and extracts its information.
    ///
    /// Reads the file content, determines its type based on extension,
    /// and creates a `FileInfo` structure with all relevant metadata.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to process
    ///
    /// # Returns
    ///
    /// `Some(FileInfo)` if the file was successfully processed and should be included,
    /// `None` if the file should be skipped
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or metadata cannot be accessed
    async fn process_file(&self, path: &Path) -> Result<Option<FileInfo>> {
        let content = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;

        let file_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("md") => FileType::Markdown,
            _ => return Ok(None),
        };

        let relative_path = path
            .strip_prefix(&self.config.repo_path)
            .unwrap_or(path)
            .to_path_buf();

        Ok(Some(FileInfo {
            path: path.to_path_buf(),
            relative_path,
            content,
            file_type,
            size: metadata.len(),
        }))
    }

    fn build_project_structure(
        &self,
        files: &[FileInfo],
        total_size: u64,
    ) -> Result<ProjectStructure> {
        let mut tree = String::new();
        let mut paths: Vec<_> = files.iter().map(|f| &f.relative_path).collect();
        paths.sort();

        tree.push_str("```\n");
        for (i, path) in paths.iter().enumerate() {
            let depth = path.components().count() - 1;
            let indent = "│   ".repeat(depth);
            let connector = if i == paths.len() - 1 {
                "└── "
            } else {
                "├── "
            };

            tree.push_str(&format!("{}{}{}\n", indent, connector, path.display()));
        }
        tree.push_str("```\n");

        Ok(ProjectStructure {
            tree,
            total_files: files.len(),
            total_size,
        })
    }

    async fn extract_project_metadata(&self) -> Result<ProjectMetadata> {
        let cargo_toml_path = self.config.repo_path.join("Cargo.toml");
        let readme_path = self.config.repo_path.join("README.md");

        let mut metadata = ProjectMetadata {
            name: self
                .config
                .repo_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            description: None,
            dependencies: Vec::new(),
            rust_version: None,
        };

        // Extract information from Cargo.toml
        if cargo_toml_path.exists() {
            let cargo_content = fs::read_to_string(&cargo_toml_path)?;
            self.parse_cargo_toml(&cargo_content, &mut metadata)?;
        }

        // Extract description from README.md
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            metadata.description = self.extract_description_from_readme(&readme_content);
        }

        Ok(metadata)
    }

    fn parse_cargo_toml(&self, content: &str, metadata: &mut ProjectMetadata) -> Result<()> {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_package = false;
        let mut in_dependencies = false;

        for line in lines {
            let line = line.trim();

            if line.starts_with("[package]") {
                in_package = true;
                in_dependencies = false;
                continue;
            }

            if line.starts_with("[dependencies") {
                in_package = false;
                in_dependencies = true;
                continue;
            }

            if line.starts_with("[") {
                in_package = false;
                in_dependencies = false;
                continue;
            }

            if in_package {
                if line.starts_with("name") {
                    if let Some(name) = line.split('=').nth(1) {
                        metadata.name = name.trim().trim_matches('"').to_string();
                    }
                } else if line.starts_with("version") {
                    if let Some(version) = line.split('=').nth(1) {
                        metadata.rust_version = Some(version.trim().trim_matches('"').to_string());
                    }
                }
            }

            if in_dependencies && !line.is_empty() {
                if let Some(dep_name) = line.split('=').next() {
                    metadata.dependencies.push(dep_name.trim().to_string());
                }
            }
        }

        Ok(())
    }

    fn extract_description_from_readme(&self, content: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut description = String::new();

        for line in lines.iter().take(10) {
            if line.starts_with('#') {
                continue;
            }

            if !line.trim().is_empty() {
                description.push_str(line);
                description.push('\n');

                if description.len() > 200 {
                    break;
                }
            }
        }

        if description.trim().is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        }
    }
}

```

---

# Source: src/lib.rs

```rust
//! # AI Context Generator - Rust Library
//!
//! A Rust library for generating structured context from code repositories,
//! specifically designed for LLMs and AI agents. This library provides both
//! simple convenience functions and advanced APIs for fine-grained control.
//!
//! ## When to Use This Library
//!
//! - **Integrate context generation into your Rust applications**
//! - **Build custom analysis workflows**
//! - **Create automated documentation systems**
//! - **Develop AI-powered developer tools**
//!
//! For standalone command-line usage, consider using the CLI tool instead.
//!
//! ## Features
//!
//! - 🔍 **Complete Scanning**: Analyzes all `.rs` and `.md` files in repositories
//! - 🌳 **AST Analysis**: Extracts structures, functions, enums and implementations
//! - 📊 **Token Control**: Respects limits and prioritizes important content
//! - 📁 **Project Structure**: Generates file tree visualizations
//! - 📖 **Documentation**: Includes markdown files and code documentation
//! - ⚡ **Async Processing**: Non-blocking, high-performance analysis
//!
//! ## Quick Start
//!
//! ### Simple Usage
//!
//! ```rust
//! use ai_context_gen::generate_context;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Generate context for current directory
//!     generate_context(PathBuf::from("."), "context.md".to_string()).await?;
//!     println!("Context generated successfully!");
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced Usage with Configuration
//!
//! ```rust,no_run
//! use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Custom configuration
//!     let config = Config {
//!         repo_path: PathBuf::from("./my-project"),
//!         max_tokens: 100000,
//!         output_file: "detailed_context.md".to_string(),
//!         include_hidden: true,
//!         include_deps: true,
//!     };
//!
//!     // Step-by-step process for more control
//!     let scanner = RepositoryScanner::new(config.clone());
//!     let scan_result = scanner.scan().await?;
//!     
//!     println!("Found {} files", scan_result.files.len());
//!     
//!     let generator = ContextGenerator::new(config);
//!     generator.generate_context(scan_result).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Using the Configuration Function
//!
//! ```rust,no_run
//! use ai_context_gen::{Config, generate_context_with_config};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = Config {
//!         repo_path: PathBuf::from("/path/to/analyze"),
//!         max_tokens: 75000,
//!         output_file: "analysis.md".to_string(),
//!         include_hidden: false,
//!         include_deps: true,
//!     };
//!
//!     generate_context_with_config(config).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## API Overview
//!
//! - [`generate_context`]: Simple function for basic use cases
//! - [`generate_context_with_config`]: Function with custom configuration
//! - [`Config`]: Configuration structure for all options
//! - [`RepositoryScanner`]: File scanning and discovery
//! - [`ContextGenerator`]: Context generation with priorities
//! - [`RustParser`]: Rust code AST analysis
//!
//! ## Integration Patterns
//!
//! ### Web Applications
//!
//! ```rust,no_run
//! use ai_context_gen::{Config, generate_context_with_config};
//! use std::path::PathBuf;
//!
//! async fn analyze_repo_endpoint(repo_path: String) -> Result<String, Box<dyn std::error::Error>> {
//!     let config = Config {
//!         repo_path: PathBuf::from(repo_path),
//!         max_tokens: 50000,
//!         output_file: format!("/tmp/analysis_{}.md", chrono::Utc::now().timestamp()),
//!         include_hidden: false,
//!         include_deps: false,
//!     };
//!     
//!     generate_context_with_config(config.clone()).await?;
//!     Ok(config.output_file)
//! }
//! ```
//!
//! ### Custom Workflows
//!
//! ```rust,no_run
//! use ai_context_gen::{Config, RepositoryScanner, ContextGenerator};
//! use std::path::PathBuf;
//!
//! async fn custom_analysis_workflow(repo_path: PathBuf) -> anyhow::Result<()> {
//!     let config = Config {
//!         repo_path: repo_path.clone(),
//!         max_tokens: 100000,
//!         output_file: "temp_analysis.md".to_string(),
//!         include_hidden: true,
//!         include_deps: true,
//!     };
//!
//!     // Scan first
//!     let scanner = RepositoryScanner::new(config.clone());
//!     let scan_result = scanner.scan().await?;
//!     
//!     // Custom filtering or processing here
//!     println!("Found {} Rust files", scan_result.files.iter()
//!         .filter(|f| matches!(f.file_type, ai_context_gen::FileType::Rust))
//!         .count());
//!     
//!     // Generate context
//!     let generator = ContextGenerator::new(config);
//!     generator.generate_context(scan_result).await?;
//!     
//!     Ok(())
//! }
//! ```

use std::path::PathBuf;

pub mod config;
pub mod generator;
pub mod parser;
pub mod scanner;
pub mod token_counter;

// Re-export main structs for easier usage
pub use config::Config;
pub use generator::ContextGenerator;
pub use parser::{EnumInfo, FunctionInfo, ImplInfo, RustAnalysis, RustParser, StructInfo};
pub use scanner::{FileInfo, FileType, RepositoryScanner, ScanResult};
pub use token_counter::{ContentPrioritizer, ContentSection, TokenCounter};

/// Default Result type used by the library
pub type Result<T> = anyhow::Result<T>;

/// Generates repository context with default configuration
///
/// This is a convenience function that configures and executes
/// the entire context generation process.
///
/// # Arguments
///
/// * `path` - Path to the repository
/// * `output` - Output file name
///
/// # Example
///
/// ```rust
/// use ai_context_gen::generate_context;
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// generate_context(PathBuf::from("."), "context.md".to_string()).await?;
/// # Ok(())
/// # }
/// ```
pub async fn generate_context(path: PathBuf, output: String) -> Result<()> {
    let config = Config {
        repo_path: path,
        max_tokens: 50000,
        output_file: output,
        include_hidden: false,
        include_deps: false,
    };

    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    Ok(())
}

/// Generates repository context with custom configuration
///
/// # Arguments
///
/// * `config` - Custom configuration
///
/// # Example
///
/// ```rust
/// use ai_context_gen::{Config, generate_context_with_config};
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config {
///     repo_path: PathBuf::from("./my-project"),
///     max_tokens: 100000,
///     output_file: "detailed_context.md".to_string(),
///     include_hidden: true,
///     include_deps: true,
/// };
///
/// generate_context_with_config(config).await?;
/// # Ok(())
/// # }
/// ```
pub async fn generate_context_with_config(config: Config) -> Result<()> {
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    Ok(())
}

```

---

# Source: src/parser.rs

```rust
//! Rust AST parsing module for the AI Context Generator.
//!
//! This module provides functionality to parse Rust source code and extract
//! structural information such as modules, functions, structs, enums, and
//! implementations using the `syn` crate.

use anyhow::Result;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{parse_file, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, Signature};

/// Complete analysis result for a single Rust source file.
///
/// Contains all structural information extracted from parsing the file's AST,
/// including modules, functions, structs, enums, and implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustAnalysis {
    /// Path to the analyzed file.
    pub file_path: String,

    /// List of modules defined in the file.
    pub modules: Vec<ModuleInfo>,

    /// List of functions defined in the file.
    pub functions: Vec<FunctionInfo>,

    /// List of structs defined in the file.
    pub structs: Vec<StructInfo>,

    /// List of enums defined in the file.
    pub enums: Vec<EnumInfo>,

    /// List of impl blocks defined in the file.
    pub implementations: Vec<ImplInfo>,

    /// Summary of the AST structure.
    pub ast_summary: String,
}

/// Information about a module definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Name of the module.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// Number of items contained in the module.
    pub items_count: usize,
}

/// Information about a function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Name of the function.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// Whether the function is async.
    pub is_async: bool,

    /// List of parameter types as strings.
    pub parameters: Vec<String>,

    /// Return type as a string, if any.
    pub return_type: Option<String>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about a struct definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructInfo {
    /// Name of the struct.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// List of fields in the struct.
    pub fields: Vec<FieldInfo>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about a struct field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// Name of the field.
    pub name: String,

    /// Type of the field as a string.
    pub field_type: String,

    /// Visibility modifier for the field.
    pub visibility: String,
}

/// Information about an enum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumInfo {
    /// Name of the enum.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// List of variant names.
    pub variants: Vec<String>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about an implementation block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplInfo {
    /// The type being implemented (e.g., "MyStruct", "Vec<T>").
    pub target: String,

    /// Name of the trait being implemented, if this is a trait impl.
    pub trait_name: Option<String>,

    /// List of methods defined in the implementation.
    pub methods: Vec<FunctionInfo>,
}

/// Rust source code parser using syn for AST analysis.
///
/// Provides static methods for parsing Rust source files and extracting
/// structural information about the code.
pub struct RustParser;

impl RustParser {
    /// Parses a Rust source file and extracts structural information.
    ///
    /// This method uses the `syn` crate to parse Rust source code into an AST
    /// and then extracts information about modules, functions, structs, enums,
    /// and implementations.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file being parsed (used for error reporting)
    /// * `content` - Source code content as a string
    ///
    /// # Returns
    ///
    /// A `RustAnalysis` containing all extracted structural information.
    ///
    /// # Errors
    ///
    /// Returns an error if the source code cannot be parsed as valid Rust syntax.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::parser::RustParser;
    ///
    /// let source = r#"
    /// pub struct MyStruct {
    ///     pub field: String,
    /// }
    ///
    /// impl MyStruct {
    ///     pub fn new() -> Self {
    ///         Self { field: String::new() }
    ///     }
    /// }
    /// "#;
    ///
    /// let analysis = RustParser::parse_rust_file("example.rs", source).unwrap();
    /// assert_eq!(analysis.structs.len(), 1);
    /// assert_eq!(analysis.implementations.len(), 1);
    /// ```
    pub fn parse_rust_file(file_path: &str, content: &str) -> Result<RustAnalysis> {
        let syntax_tree = parse_file(content)?;

        let mut analysis = RustAnalysis {
            file_path: file_path.to_string(),
            modules: Vec::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            implementations: Vec::new(),
            ast_summary: String::new(),
        };

        // Analyze each item in the file
        for item in &syntax_tree.items {
            match item {
                Item::Mod(item_mod) => {
                    analysis.modules.push(Self::parse_module(item_mod));
                }
                Item::Fn(item_fn) => {
                    analysis.functions.push(Self::parse_function(item_fn));
                }
                Item::Struct(item_struct) => {
                    analysis.structs.push(Self::parse_struct(item_struct));
                }
                Item::Enum(item_enum) => {
                    analysis.enums.push(Self::parse_enum(item_enum));
                }
                Item::Impl(item_impl) => {
                    analysis.implementations.push(Self::parse_impl(item_impl));
                }
                _ => {}
            }
        }

        analysis.ast_summary = Self::generate_ast_summary(&analysis);

        Ok(analysis)
    }

    fn parse_module(item: &ItemMod) -> ModuleInfo {
        let items_count = item
            .content
            .as_ref()
            .map(|(_, items)| items.len())
            .unwrap_or(0);

        ModuleInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            items_count,
        }
    }

    fn parse_function(item: &ItemFn) -> FunctionInfo {
        let sig = &item.sig;

        FunctionInfo {
            name: sig.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            is_async: sig.asyncness.is_some(),
            parameters: Self::parse_parameters(sig),
            return_type: Self::parse_return_type(sig),
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_struct(item: &ItemStruct) -> StructInfo {
        let fields = match &item.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| FieldInfo {
                    name: f.ident.as_ref().unwrap().to_string(),
                    field_type: f.ty.to_token_stream().to_string(),
                    visibility: Self::parse_visibility(&f.vis),
                })
                .collect(),
            syn::Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| FieldInfo {
                    name: format!("field_{}", i),
                    field_type: f.ty.to_token_stream().to_string(),
                    visibility: Self::parse_visibility(&f.vis),
                })
                .collect(),
            syn::Fields::Unit => Vec::new(),
        };

        StructInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            fields,
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_enum(item: &ItemEnum) -> EnumInfo {
        let variants = item.variants.iter().map(|v| v.ident.to_string()).collect();

        EnumInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            variants,
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_impl(item: &ItemImpl) -> ImplInfo {
        let target = item.self_ty.to_token_stream().to_string();
        let trait_name = item
            .trait_
            .as_ref()
            .map(|(_, path, _)| path.to_token_stream().to_string());

        let methods = item
            .items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Fn(method) = item {
                    Some(FunctionInfo {
                        name: method.sig.ident.to_string(),
                        visibility: Self::parse_visibility(&method.vis),
                        is_async: method.sig.asyncness.is_some(),
                        parameters: Self::parse_parameters(&method.sig),
                        return_type: Self::parse_return_type(&method.sig),
                        documentation: Self::extract_doc_comments(&method.attrs),
                    })
                } else {
                    None
                }
            })
            .collect();

        ImplInfo {
            target,
            trait_name,
            methods,
        }
    }

    fn parse_visibility(vis: &syn::Visibility) -> String {
        match vis {
            syn::Visibility::Public(_) => "pub".to_string(),
            syn::Visibility::Restricted(restricted) => {
                format!("pub({})", restricted.path.to_token_stream())
            }
            syn::Visibility::Inherited => "private".to_string(),
        }
    }

    fn parse_parameters(sig: &Signature) -> Vec<String> {
        sig.inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_some() {
                        "&mut self".to_string()
                    } else {
                        "&self".to_string()
                    }
                }
                syn::FnArg::Typed(typed) => {
                    format!(
                        "{}: {}",
                        typed.pat.to_token_stream(),
                        typed.ty.to_token_stream()
                    )
                }
            })
            .collect()
    }

    fn parse_return_type(sig: &Signature) -> Option<String> {
        match &sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
        }
    }

    fn extract_doc_comments(attrs: &[syn::Attribute]) -> Option<String> {
        let mut doc_comments = Vec::new();

        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let Ok(syn::Lit::Str(lit_str)) = attr.parse_args() {
                    doc_comments.push(lit_str.value());
                }
            }
        }

        if doc_comments.is_empty() {
            None
        } else {
            Some(doc_comments.join("\n"))
        }
    }

    fn generate_ast_summary(analysis: &RustAnalysis) -> String {
        let mut summary = String::new();

        summary.push_str(&format!("# AST Summary for {}\n\n", analysis.file_path));

        if !analysis.modules.is_empty() {
            summary.push_str("## Modules\n");
            for module in &analysis.modules {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} items\n",
                    module.name, module.visibility, module.items_count
                ));
            }
            summary.push('\n');
        }

        if !analysis.structs.is_empty() {
            summary.push_str("## Structs\n");
            for struct_info in &analysis.structs {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} fields\n",
                    struct_info.name,
                    struct_info.visibility,
                    struct_info.fields.len()
                ));
            }
            summary.push('\n');
        }

        if !analysis.enums.is_empty() {
            summary.push_str("## Enums\n");
            for enum_info in &analysis.enums {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} variants\n",
                    enum_info.name,
                    enum_info.visibility,
                    enum_info.variants.len()
                ));
            }
            summary.push('\n');
        }

        if !analysis.functions.is_empty() {
            summary.push_str("## Functions\n");
            for func in &analysis.functions {
                let async_marker = if func.is_async { "async " } else { "" };
                summary.push_str(&format!(
                    "- `{}{}{}` ({})\n",
                    async_marker,
                    func.name,
                    if func.parameters.is_empty() {
                        "()"
                    } else {
                        "(...)"
                    },
                    func.visibility
                ));
            }
            summary.push('\n');
        }

        if !analysis.implementations.is_empty() {
            summary.push_str("## Implementations\n");
            for impl_info in &analysis.implementations {
                let trait_part = impl_info
                    .trait_name
                    .as_ref()
                    .map(|t| format!("{} for ", t))
                    .unwrap_or_default();
                summary.push_str(&format!("- `impl {}{}`\n", trait_part, impl_info.target));
            }
        }

        summary
    }
}

```

---

# Source: src/token_counter.rs

```rust
//! Token counting and content prioritization module.
//!
//! This module provides functionality for accurate token counting using the GPT-4
//! tokenizer and intelligent content prioritization to fit within token limits.

use anyhow::Result;
use tiktoken_rs::{get_bpe_from_model, CoreBPE};

/// Token counter using the GPT-4 tokenizer for accurate token counting.
///
/// Provides methods for counting tokens in text and truncating content to fit
/// within specified token limits while maintaining text coherence.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::TokenCounter;
///
/// let counter = TokenCounter::new().unwrap();
/// let text = "Hello, world!";
/// let token_count = counter.count_tokens(text);
/// println!("Text has {} tokens", token_count);
/// ```
pub struct TokenCounter {
    bpe: CoreBPE,
}

impl TokenCounter {
    /// Creates a new token counter using the GPT-4 tokenizer.
    ///
    /// # Returns
    ///
    /// A new `TokenCounter` instance configured with the GPT-4 BPE tokenizer.
    ///
    /// # Errors
    ///
    /// Returns an error if the GPT-4 tokenizer model cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        let bpe = get_bpe_from_model("gpt-4")?;
        Ok(Self { bpe })
    }

    /// Counts the number of tokens in the given text.
    ///
    /// Uses the GPT-4 tokenizer to provide accurate token counts that match
    /// what would be used by OpenAI's models and similar systems.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to count tokens for
    ///
    /// # Returns
    ///
    /// The number of tokens in the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// let count = counter.count_tokens("Hello, world!");
    /// assert!(count > 0);
    /// ```
    pub fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }

    /// Truncates text to fit within a specified token limit.
    ///
    /// Attempts to preserve text coherence by truncating at token boundaries
    /// rather than character boundaries. Falls back to character truncation
    /// if token decoding fails.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to truncate
    /// * `max_tokens` - Maximum number of tokens to include
    ///
    /// # Returns
    ///
    /// The truncated text that fits within the token limit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// let long_text = "This is a very long text that exceeds the token limit...";
    /// let truncated = counter.truncate_to_token_limit(long_text, 10);
    /// assert!(counter.count_tokens(&truncated) <= 10);
    /// ```
    pub fn truncate_to_token_limit(&self, text: &str, max_tokens: usize) -> String {
        let tokens = self.bpe.encode_with_special_tokens(text);

        if tokens.len() <= max_tokens {
            return text.to_string();
        }

        let truncated_tokens = &tokens[..max_tokens];
        match self.bpe.decode(truncated_tokens.to_vec()) {
            Ok(truncated_text) => truncated_text,
            Err(_) => {
                // Fallback: truncate by characters
                let char_limit = (text.len() * max_tokens) / tokens.len();
                text.chars().take(char_limit).collect()
            }
        }
    }
}

/// Content prioritizer that manages sections based on priority and token limits.
///
/// The prioritizer sorts content sections by priority and ensures the total
/// content fits within specified token limits by truncating lower priority
/// sections when necessary.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::{ContentPrioritizer, ContentSection};
///
/// let prioritizer = ContentPrioritizer::new().unwrap();
/// let sections = vec![
///     ContentSection::new("High Priority".to_string(), "Content...".to_string(), 10),
///     ContentSection::new("Low Priority".to_string(), "More content...".to_string(), 1),
/// ];
/// let prioritized = prioritizer.prioritize_content(sections, 1000);
/// ```
pub struct ContentPrioritizer {
    token_counter: TokenCounter,
}

impl ContentPrioritizer {
    /// Creates a new content prioritizer.
    ///
    /// # Returns
    ///
    /// A new `ContentPrioritizer` instance with an initialized token counter.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying token counter cannot be initialized.
    pub fn new() -> Result<Self> {
        Ok(Self {
            token_counter: TokenCounter::new()?,
        })
    }

    /// Prioritizes and truncates content sections to fit within token limits.
    ///
    /// Sorts sections by priority (highest first) and includes as many complete
    /// sections as possible. When a section would exceed the token limit, it
    /// attempts to truncate it if there are sufficient remaining tokens.
    ///
    /// # Arguments
    ///
    /// * `sections` - List of content sections to prioritize
    /// * `max_tokens` - Maximum total tokens allowed
    ///
    /// # Returns
    ///
    /// A vector of sections that fit within the token limit, sorted by priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::{ContentPrioritizer, ContentSection};
    ///
    /// let prioritizer = ContentPrioritizer::new().unwrap();
    /// let sections = vec![
    ///     ContentSection::new("Important".to_string(), "Critical info".to_string(), 10),
    ///     ContentSection::new("Less Important".to_string(), "Extra details".to_string(), 5),
    /// ];
    /// let result = prioritizer.prioritize_content(sections, 100);
    /// // Higher priority sections appear first
    /// ```
    pub fn prioritize_content(
        &self,
        sections: Vec<ContentSection>,
        max_tokens: usize,
    ) -> Vec<ContentSection> {
        let mut prioritized = sections;

        // Sort by priority (highest priority first)
        prioritized.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut total_tokens = 0;
        let mut result = Vec::new();

        for mut section in prioritized {
            let section_tokens = self.token_counter.count_tokens(&section.content);

            if total_tokens + section_tokens <= max_tokens {
                total_tokens += section_tokens;
                result.push(section);
            } else {
                // Try to truncate content to fit within the limit
                let remaining_tokens = max_tokens - total_tokens;
                if remaining_tokens > 100 {
                    // Only include if at least 100 tokens remain
                    section.content = self
                        .token_counter
                        .truncate_to_token_limit(&section.content, remaining_tokens);
                    section.truncated = true;
                    result.push(section);
                    break;
                }
            }
        }

        result
    }
}

/// A content section with associated metadata for prioritization.
///
/// Represents a section of content (like project metadata, source code, or
/// documentation) with a title, content, priority level, and truncation status.
///
/// # Priority Levels
///
/// - `9-10`: High priority (metadata, documentation)
/// - `5-6`: Medium priority (AST analysis, structure)
/// - `1-2`: Low priority (source code)
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::ContentSection;
///
/// // Create a high-priority section
/// let section = ContentSection::high_priority(
///     "Project Metadata".to_string(),
///     "Important project information...".to_string()
/// );
///
/// // Create a custom priority section
/// let custom = ContentSection::new(
///     "Custom Section".to_string(),
///     "Content here...".to_string(),
///     7
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ContentSection {
    /// Title of the content section.
    pub title: String,

    /// The actual content of the section.
    pub content: String,

    /// Priority level (higher numbers = higher priority).
    pub priority: u8,

    /// Whether this section was truncated to fit token limits.
    pub truncated: bool,
}

impl ContentSection {
    /// Creates a new content section with the specified priority.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    /// * `priority` - Priority level (0-255, higher is more important)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::ContentSection;
    ///
    /// let section = ContentSection::new(
    ///     "My Section".to_string(),
    ///     "Section content...".to_string(),
    ///     8
    /// );
    /// ```
    pub fn new(title: String, content: String, priority: u8) -> Self {
        Self {
            title,
            content,
            priority,
            truncated: false,
        }
    }

    /// Creates a high-priority content section (priority 9).
    ///
    /// Use for critical content like project metadata and documentation
    /// that should always be included.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn high_priority(title: String, content: String) -> Self {
        Self::new(title, content, 9)
    }

    /// Creates a medium-priority content section (priority 5).
    ///
    /// Use for structural information like AST analysis and project organization.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn medium_priority(title: String, content: String) -> Self {
        Self::new(title, content, 5)
    }

    /// Creates a low-priority content section (priority 1).
    ///
    /// Use for detailed content like complete source code that can be
    /// truncated if necessary.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn low_priority(title: String, content: String) -> Self {
        Self::new(title, content, 1)
    }
}

```

---

# Source: context.md

```markdown
# AI Context Generation Report

Generated on: 2025-07-10 12:26:25 UTC
Repository: .
Max tokens: 50000

## Table of Contents

1. Project Metadata
2. Project Structure
3. Documentation: README.md
4. Rust Analysis: src/main.rs
5. Rust Analysis: src/generator.rs
6. Rust Analysis: src/config.rs
7. Rust Analysis: src/scanner.rs
8. Rust Analysis: src/lib.rs
9. Rust Analysis: src/parser.rs
10. Rust Analysis: src/token_counter.rs
11. Rust Analysis: examples/advanced_usage.rs
12. Rust Analysis: examples/basic_usage.rs
13. Source: src/main.rs
14. Source: src/generator.rs
15. Source: src/config.rs
16. Source: src/scanner.rs
17. Source: src/lib.rs
18. Source: src/parser.rs
19. Source: src/token_counter.rs
20. Source: README.md
21. Source: examples/advanced_usage.rs
22. Source: examples/basic_usage.rs

---

# Project Metadata

**Name:** ai-context-gen
**Description:** [![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
[![Documentation](https://docs.rs/ai-context-gen/badge.svg)](https://docs.rs/ai-context-gen)
**Dependencies:**
- clap
- walkdir
- serde
- serde_json
- syn
- quote
- proc-macro2
- tiktoken-rs
- anyhow
- regex
- chrono
- version
- features
**Version:** 0.1.1
**Total files:** 10
**Total size:** 82062 bytes

---

# Project Structure

```
├── README.md
│   ├── examples/advanced_usage.rs
│   ├── examples/basic_usage.rs
│   ├── src/config.rs
│   ├── src/generator.rs
│   ├── src/lib.rs
│   ├── src/main.rs
│   ├── src/parser.rs
│   ├── src/scanner.rs
│   └── src/token_counter.rs
```

---

# Documentation: README.md

# AI Context Generator

[![Crates.io](https://img.shields.io/crates/v/ai-context-gen.svg)](https://crates.io/crates/ai-context-gen)
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

- Rust 1.70 or higher
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
ai-context-gen = "0.1.1"
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

## 📄 License

This project is licensed under the MIT license. See the `LICENSE` file for details.

## 🛤️ Roadmap

- [ ] Web interface
- [ ] Git integration
- [ ] Commit history analysis
- [ ] Support for other output formats (JSON, YAML)
- [ ] Cache for better performance

## 📝 Changelog

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

---

# Rust Analysis: src/main.rs

## Functions
- **main**() -> Result < () > (private)

## Structs
- **Args**: 5 fields (private)

---

# Rust Analysis: src/generator.rs

## Structs
- **ContextGenerator**: 2 fields (pub)

## Implementations
- **impl ContextGenerator**: 8 methods

---

# Rust Analysis: src/config.rs

## Structs
- **Config**: 5 fields (pub)

## Implementations
- **impl Config**: 1 methods

---

# Rust Analysis: src/scanner.rs

## Structs
- **FileInfo**: 5 fields (pub)
- **ScanResult**: 3 fields (pub)
- **ProjectStructure**: 3 fields (pub)
- **ProjectMetadata**: 4 fields (pub)
- **RepositoryScanner**: 1 fields (pub)

## Enums
- **FileType**: 2 variants (pub)

## Implementations
- **impl RepositoryScanner**: 8 methods

---

# Rust Analysis: src/lib.rs

## Modules
- **config**: pub
- **generator**: pub
- **parser**: pub
- **scanner**: pub
- **token_counter**: pub

## Functions
- **generate_context**(path: PathBuf, output: String) -> Result < () > (pub)
- **generate_context_with_config**(config: Config) -> Result < () > (pub)

---

# Rust Analysis: src/parser.rs

## Structs
- **RustAnalysis**: 7 fields (pub)
- **ModuleInfo**: 3 fields (pub)
- **FunctionInfo**: 6 fields (pub)
- **StructInfo**: 4 fields (pub)
- **FieldInfo**: 3 fields (pub)
- **EnumInfo**: 4 fields (pub)
- **ImplInfo**: 3 fields (pub)
- **RustParser**: 0 fields (pub)

## Implementations
- **impl RustParser**: 11 methods

---

# Rust Analysis: src/token_counter.rs

## Structs
- **TokenCounter**: 1 fields (pub)
- **ContentPrioritizer**: 1 fields (pub)
- **ContentSection**: 4 fields (pub)

## Implementations
- **impl TokenCounter**: 3 methods
- **impl ContentPrioritizer**: 2 methods
- **impl ContentSection**: 4 methods

---

# Rust Analysis: examples/advanced_usage.rs

## Functions
- **main**() -> anyhow :: Result < () > (private)

---

# Rust Analysis: examples/basic_usage.rs

## Functions
- **main**() -> anyhow :: Result < () > (private)

---

# Source: src/main.rs

```rust
//! AI Context Generator CLI application.
//!
//! Command-line interface for generating structured context from Rust repositories.
//! This tool scans repository files, analyzes Rust code structure, and generates
//! markdown context suitable for LLMs and AI agents.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};

/// Command-line arguments for the AI Context Generator.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the repository to analyze
    ///
    /// Specifies the root directory of the project to scan. The tool will
    /// recursively process all supported files within this directory.
    #[arg(short, long, default_value = ".")]
    path: PathBuf,

    /// Maximum number of tokens to include in the output
    ///
    /// Controls the size of the generated context to fit within LLM token limits.
    /// Content is prioritized and truncated as needed to stay within this limit.
    #[arg(short, long, default_value = "50000")]
    max_tokens: usize,

    /// Output file name for the generated context
    ///
    /// The markdown file where the generated context will be written.
    /// If the file exists, it will be overwritten.
    #[arg(short, long, default_value = "repo_context.md")]
    output: String,

    /// Include hidden files and directories in the analysis
    ///
    /// When enabled, files and directories starting with '.' will be included
    /// in the scan (except for those in the ignore list).
    #[arg(long)]
    include_hidden: bool,

    /// Include analysis of external dependencies
    ///
    /// When enabled, the tool will attempt to analyze and include information
    /// about external dependencies from Cargo.toml.
    #[arg(long)]
    include_deps: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config {
        repo_path: args.path,
        max_tokens: args.max_tokens,
        output_file: args.output,
        include_hidden: args.include_hidden,
        include_deps: args.include_deps,
    };

    println!("🔍 Scanning repository...");
    let scanner = RepositoryScanner::new(config.clone());
    let scan_result = scanner.scan().await?;

    println!("📝 Generating context...");
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    println!("✅ Context generated successfully!");
    Ok(())
}

```

---

# Source: src/generator.rs

```rust
//! Context generation module for the AI Context Generator.
//!
//! This module provides functionality to generate structured markdown context
//! from scanned repository data, with intelligent content prioritization and
//! token limit management.

use anyhow::Result;
use chrono::Utc;
use std::fs;

use crate::config::Config;
use crate::parser::RustParser;
use crate::scanner::{FileType, ScanResult};
use crate::token_counter::{ContentPrioritizer, ContentSection};

/// Context generator that creates structured markdown from repository scan results.
///
/// The generator takes scan results and creates a prioritized, token-limited markdown
/// document suitable for consumption by LLMs and AI agents. Content is organized by
/// priority, with metadata and documentation receiving higher priority than source code.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config::default();
/// let scanner = RepositoryScanner::new(config.clone());
/// let scan_result = scanner.scan().await?;
///
/// let generator = ContextGenerator::new(config);
/// generator.generate_context(scan_result).await?;
/// # Ok(())
/// # }
/// ```
pub struct ContextGenerator {
    config: Config,
    prioritizer: ContentPrioritizer,
}

impl ContextGenerator {
    /// Creates a new context generator with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying output options and token limits
    ///
    /// # Panics
    ///
    /// Panics if the content prioritizer cannot be initialized (e.g., if the
    /// tiktoken model cannot be loaded).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, ContextGenerator};
    ///
    /// let config = Config::default();
    /// let generator = ContextGenerator::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            config,
            prioritizer: ContentPrioritizer::new()
                .expect("Failed to initialize content prioritizer"),
        }
    }

    /// Generates a complete context document from scan results.
    ///
    /// This method creates a structured markdown document with prioritized content
    /// sections including project metadata, file structure, documentation, AST
    /// analysis, and source code. Content is prioritized and truncated based on
    /// the configured token limit.
    ///
    /// # Arguments
    ///
    /// * `scan_result` - Results from repository scanning containing files and metadata
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the context was successfully generated and written to the
    /// configured output file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - AST parsing fails for Rust files
    /// - The output file cannot be written
    /// - Token counting or content prioritization fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config.clone());
    /// let scan_result = scanner.scan().await?;
    ///
    /// let generator = ContextGenerator::new(config);
    /// generator.generate_context(scan_result).await?;
    ///
    /// println!("Context generated successfully!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_context(&self, scan_result: ScanResult) -> Result<()> {
        let mut sections = Vec::new();

        // Project metadata section (high priority)
        sections.push(self.create_metadata_section(&scan_result));

        // Project structure section (high priority)
        sections.push(self.create_structure_section(&scan_result));

        // Markdown documentation sections (high priority)
        sections.extend(self.create_markdown_sections(&scan_result));

        // AST analysis sections for Rust files (medium priority)
        sections.extend(self.create_rust_analysis_sections(&scan_result).await?);

        // Source code sections (low priority)
        sections.extend(self.create_source_code_sections(&scan_result));

        // Prioritize and truncate content based on token limit
        let final_sections = self
            .prioritizer
            .prioritize_content(sections, self.config.max_tokens);

        // Generate final context
        let context = self.format_context(final_sections);

        // Write to file
        fs::write(&self.config.output_file, context)?;

        println!(
            "Context generated successfully in: {}",
            self.config.output_file
        );
        Ok(())
    }

    fn create_metadata_section(&self, scan_result: &ScanResult) -> ContentSection {
        let mut content = String::new();
        content.push_str("# Project Metadata\n\n");
        content.push_str(&format!("**Name:** {}\n", scan_result.metadata.name));

        if let Some(description) = &scan_result.metadata.description {
            content.push_str(&format!("**Description:** {}\n", description));
        }

        if !scan_result.metadata.dependencies.is_empty() {
            content.push_str("**Dependencies:**\n");
            for dep in &scan_result.metadata.dependencies {
                content.push_str(&format!("- {}\n", dep));
            }
        }

        if let Some(rust_version) = &scan_result.metadata.rust_version {
            content.push_str(&format!("**Version:** {}\n", rust_version));
        }

        content.push_str(&format!(
            "**Total files:** {}\n",
            scan_result.project_structure.total_files
        ));
        content.push_str(&format!(
            "**Total size:** {} bytes\n\n",
            scan_result.project_structure.total_size
        ));

        ContentSection {
            title: "Project Metadata".to_string(),
            content,
            priority: 10,
            truncated: false,
        }
    }

    fn create_structure_section(&self, scan_result: &ScanResult) -> ContentSection {
        let mut content = String::new();
        content.push_str("# Project Structure\n\n");
        content.push_str(&scan_result.project_structure.tree);
        content.push('\n');

        ContentSection {
            title: "Project Structure".to_string(),
            content,
            priority: 9,
            truncated: false,
        }
    }

    fn create_markdown_sections(&self, scan_result: &ScanResult) -> Vec<ContentSection> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            if matches!(file.file_type, FileType::Markdown) {
                let mut content = String::new();
                content.push_str(&format!(
                    "# Documentation: {}\n\n",
                    file.relative_path.display()
                ));
                content.push_str(&file.content);
                content.push('\n');

                sections.push(ContentSection {
                    title: format!("Documentation: {}", file.relative_path.display()),
                    content,
                    priority: 8,
                    truncated: false,
                });
            }
        }

        sections
    }

    async fn create_rust_analysis_sections(
        &self,
        scan_result: &ScanResult,
    ) -> Result<Vec<ContentSection>> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            if matches!(file.file_type, FileType::Rust) {
                match RustParser::parse_rust_file(&file.path.to_string_lossy(), &file.content) {
                    Ok(analysis) => {
                        let mut content = String::new();
                        content.push_str(&format!(
                            "# Rust Analysis: {}\n\n",
                            file.relative_path.display()
                        ));

                        if !analysis.modules.is_empty() {
                            content.push_str("## Modules\n");
                            for module in &analysis.modules {
                                content.push_str(&format!(
                                    "- **{}**: {}\n",
                                    module.name, module.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.functions.is_empty() {
                            content.push_str("## Functions\n");
                            for function in &analysis.functions {
                                let params = function.parameters.join(", ");
                                let return_type = function.return_type.as_deref().unwrap_or("()");
                                content.push_str(&format!(
                                    "- **{}**({}) -> {} ({})\n",
                                    function.name, params, return_type, function.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.structs.is_empty() {
                            content.push_str("## Structs\n");
                            for struct_info in &analysis.structs {
                                content.push_str(&format!(
                                    "- **{}**: {} fields ({})\n",
                                    struct_info.name,
                                    struct_info.fields.len(),
                                    struct_info.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.enums.is_empty() {
                            content.push_str("## Enums\n");
                            for enum_info in &analysis.enums {
                                content.push_str(&format!(
                                    "- **{}**: {} variants ({})\n",
                                    enum_info.name,
                                    enum_info.variants.len(),
                                    enum_info.visibility
                                ));
                            }
                            content.push('\n');
                        }

                        if !analysis.implementations.is_empty() {
                            content.push_str("## Implementations\n");
                            for impl_info in &analysis.implementations {
                                content.push_str(&format!(
                                    "- **impl {}**: {} methods\n",
                                    impl_info.target,
                                    impl_info.methods.len()
                                ));
                            }
                            content.push('\n');
                        }

                        sections.push(ContentSection {
                            title: format!("Rust Analysis: {}", file.relative_path.display()),
                            content,
                            priority: 6,
                            truncated: false,
                        });
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse {}: {}",
                            file.relative_path.display(),
                            e
                        );
                    }
                }
            }
        }

        Ok(sections)
    }

    fn create_source_code_sections(&self, scan_result: &ScanResult) -> Vec<ContentSection> {
        let mut sections = Vec::new();

        for file in &scan_result.files {
            let mut content = String::new();
            content.push_str(&format!("# Source: {}\n\n", file.relative_path.display()));
            content.push_str("```");

            match file.file_type {
                FileType::Rust => content.push_str("rust"),
                FileType::Markdown => content.push_str("markdown"),
            }

            content.push('\n');
            content.push_str(&file.content);
            content.push_str("\n```\n\n");

            sections.push(ContentSection {
                title: format!("Source: {}", file.relative_path.display()),
                content,
                priority: 3,
                truncated: false,
            });
        }

        sections
    }

    fn format_context(&self, sections: Vec<ContentSection>) -> String {
        let mut context = String::new();

        // Header
        context.push_str("# AI Context Generation Report\n\n");
        context.push_str(&format!(
            "Generated on: {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        context.push_str(&format!(
            "Repository: {}\n",
            self.config.repo_path.display()
        ));
        context.push_str(&format!("Max tokens: {}\n\n", self.config.max_tokens));

        // Table of contents
        context.push_str("## Table of Contents\n\n");
        for (i, section) in sections.iter().enumerate() {
            context.push_str(&format!("{}. {}", i + 1, section.title));
            if section.truncated {
                context.push_str(" (truncated)");
            }
            context.push('\n');
        }
        context.push('\n');

        // Sections
        for section in sections {
            context.push_str("---\n\n");
            context.push_str(&section.content);
        }

        context
    }
}

```

---

# Source: src/config.rs

```rust
//! Configuration module for the AI Context Generator.
//!
//! This module provides configuration structures and constants for customizing
//! the behavior of the context generation process.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration structure for the AI Context Generator.
///
/// This structure holds all the configuration options that control how the
/// context generation process behaves, including input/output paths, token limits,
/// and scanning options.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::Config;
/// use std::path::PathBuf;
///
/// // Create a default configuration
/// let config = Config::default();
///
/// // Create a custom configuration
/// let custom_config = Config {
///     repo_path: PathBuf::from("./my-project"),
///     max_tokens: 100000,
///     output_file: "custom_context.md".to_string(),
///     include_hidden: true,
///     include_deps: false,
/// };
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Path to the repository to analyze.
    ///
    /// This should point to the root directory of the project you want to analyze.
    /// The scanner will recursively process all supported files within this directory.
    pub repo_path: PathBuf,

    /// Maximum number of tokens to include in the generated context.
    ///
    /// This limit helps ensure the generated context fits within LLM token limits.
    /// When the limit is reached, lower priority content will be truncated.
    /// Uses GPT-4 tokenizer for accurate counting.
    pub max_tokens: usize,

    /// Output file path for the generated context.
    ///
    /// The generated markdown context will be written to this file.
    /// If the file already exists, it will be overwritten.
    pub output_file: String,

    /// Whether to include hidden files and directories in the analysis.
    ///
    /// When `true`, files and directories starting with `.` will be included
    /// in the scan (except for those in [`IGNORED_DIRS`]).
    pub include_hidden: bool,

    /// Whether to include external dependency analysis.
    ///
    /// When `true`, the generator will attempt to analyze and include
    /// information about external dependencies from `Cargo.toml`.
    pub include_deps: bool,
}

impl Default for Config {
    /// Creates a default configuration with sensible defaults.
    ///
    /// # Default Values
    ///
    /// - `repo_path`: Current directory (`.`)
    /// - `max_tokens`: 50,000 tokens
    /// - `output_file`: `"repo_context.md"`
    /// - `include_hidden`: `false`
    /// - `include_deps`: `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.max_tokens, 50000);
    /// assert_eq!(config.output_file, "repo_context.md");
    /// ```
    fn default() -> Self {
        Self {
            repo_path: PathBuf::from("."),
            max_tokens: 50000,
            output_file: "repo_context.md".to_string(),
            include_hidden: false,
            include_deps: false,
        }
    }
}

/// File extensions that are supported for analysis.
///
/// Currently, the generator supports:
/// - `.rs` - Rust source files (full AST analysis)
/// - `.md` - Markdown documentation files
pub const SUPPORTED_EXTENSIONS: &[&str] = &[".rs", ".md"];

/// Directory names that are automatically ignored during scanning.
///
/// These directories are commonly used for build artifacts, dependencies,
/// or IDE-specific files that don't contain relevant source code.
pub const IGNORED_DIRS: &[&str] = &["target", "node_modules", ".git", ".vscode", ".idea"];

/// File names that are automatically ignored during scanning.
///
/// These files are typically metadata, configuration, or system files
/// that don't contribute meaningful content to the context.
pub const IGNORED_FILES: &[&str] = &["Cargo.lock", ".gitignore", ".DS_Store"];

```

---

# Source: src/scanner.rs

```rust
//! Repository scanning module for the AI Context Generator.
//!
//! This module provides functionality to scan and analyze repository structure,
//! extracting metadata, file information, and project organization.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{Config, IGNORED_DIRS, IGNORED_FILES, SUPPORTED_EXTENSIONS};

/// Information about a single file in the repository.
///
/// Contains both metadata and content for files that are included in the analysis.
/// This structure is used to pass file information between scanning and generation phases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Absolute path to the file on the filesystem.
    pub path: PathBuf,

    /// Path relative to the repository root.
    ///
    /// This is used for display purposes in the generated context.
    pub relative_path: PathBuf,

    /// Complete content of the file as a string.
    ///
    /// For text files, this contains the entire file content.
    /// Binary files are not processed and won't appear in scan results.
    pub content: String,

    /// Type classification of the file based on its extension.
    pub file_type: FileType,

    /// Size of the file in bytes.
    pub size: u64,
}

/// Classification of file types supported by the generator.
///
/// Different file types receive different processing and priority levels
/// during context generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    /// Rust source files (`.rs` extension).
    ///
    /// These files receive full AST analysis to extract structural information
    /// about modules, functions, structs, enums, and implementations.
    Rust,

    /// Markdown documentation files (`.md` extension).
    ///
    /// These files are included as high-priority documentation content.
    Markdown,
}

/// Complete result of repository scanning operation.
///
/// Contains all information gathered during the scanning phase, including
/// individual files, project structure, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// List of all files that were processed during scanning.
    ///
    /// Only files with supported extensions that passed filtering are included.
    pub files: Vec<FileInfo>,

    /// Structural information about the project organization.
    pub project_structure: ProjectStructure,

    /// Metadata extracted from project configuration files.
    pub metadata: ProjectMetadata,
}

/// Information about the overall structure and organization of the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    /// String representation of the project's file tree.
    ///
    /// This is formatted as a text-based tree structure suitable for
    /// inclusion in markdown documentation.
    pub tree: String,

    /// Total number of files that were processed.
    pub total_files: usize,

    /// Combined size of all processed files in bytes.
    pub total_size: u64,
}

/// Project metadata extracted from configuration files and repository structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Name of the project.
    ///
    /// Extracted from `Cargo.toml` if available, otherwise derived from
    /// the repository directory name.
    pub name: String,

    /// Project description, if available.
    ///
    /// Extracted from `Cargo.toml` description field or README.md content.
    pub description: Option<String>,

    /// List of main dependencies.
    ///
    /// Extracted from the `[dependencies]` section of `Cargo.toml`.
    pub dependencies: Vec<String>,

    /// Rust version or project version.
    ///
    /// Extracted from `Cargo.toml` version field.
    pub rust_version: Option<String>,
}

/// Repository scanner that processes project files and structure.
///
/// The scanner walks through the repository directory, identifies relevant files,
/// extracts their content, and gathers project metadata.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, RepositoryScanner};
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config {
///     repo_path: PathBuf::from("."),
///     max_tokens: 50000,
///     output_file: "context.md".to_string(),
///     include_hidden: false,
///     include_deps: true,
/// };
///
/// let scanner = RepositoryScanner::new(config);
/// let scan_result = scanner.scan().await?;
///
/// println!("Found {} files", scan_result.files.len());
/// # Ok(())
/// # }
/// ```
pub struct RepositoryScanner {
    config: Config,
}

impl RepositoryScanner {
    /// Creates a new repository scanner with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying scanning behavior and output options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Performs a complete scan of the repository.
    ///
    /// This method walks through the repository directory structure, processes
    /// all supported files, extracts project metadata, and builds a comprehensive
    /// scan result.
    ///
    /// # Returns
    ///
    /// A `ScanResult` containing all discovered files, project structure, and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The repository path doesn't exist or isn't accessible
    /// - File system errors occur during scanning
    /// - Files can't be read or parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// let result = scanner.scan().await?;
    ///
    /// println!("Scanned {} files", result.files.len());
    /// println!("Total size: {} bytes", result.project_structure.total_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn scan(&self) -> Result<ScanResult> {
        let mut files = Vec::new();
        let mut total_size = 0u64;

        for entry in WalkDir::new(&self.config.repo_path)
            .into_iter()
            .filter_entry(|e| self.should_include_path(e.path()))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_info) = self.process_file(path).await? {
                    total_size += file_info.size;
                    files.push(file_info);
                }
            }
        }

        let project_structure = self.build_project_structure(&files, total_size)?;
        let metadata = self.extract_project_metadata().await?;

        Ok(ScanResult {
            files,
            project_structure,
            metadata,
        })
    }

    /// Determines whether a path should be included in the scan.
    ///
    /// This method applies filtering rules based on the configuration and
    /// predefined ignore lists to determine if a file or directory should
    /// be processed.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to evaluate for inclusion
    ///
    /// # Returns
    ///
    /// `true` if the path should be included, `false` otherwise
    fn should_include_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Ignore hidden directories if not configured to include them
        if !self.config.include_hidden && path_str.contains("/.") {
            return false;
        }

        // Ignore specific directories
        for ignored_dir in IGNORED_DIRS {
            if path_str.contains(ignored_dir) {
                return false;
            }
        }

        // If it's a file, check if it's supported
        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            // Ignore specific files
            if IGNORED_FILES.contains(&filename.as_ref()) {
                return false;
            }

            // Check extension
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                return SUPPORTED_EXTENSIONS.contains(&ext_str.as_str());
            }

            return false;
        }

        true
    }

    /// Processes a single file and extracts its information.
    ///
    /// Reads the file content, determines its type based on extension,
    /// and creates a `FileInfo` structure with all relevant metadata.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to process
    ///
    /// # Returns
    ///
    /// `Some(FileInfo)` if the file was successfully processed and should be included,
    /// `None` if the file should be skipped
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or metadata cannot be accessed
    async fn process_file(&self, path: &Path) -> Result<Option<FileInfo>> {
        let content = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;

        let file_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("md") => FileType::Markdown,
            _ => return Ok(None),
        };

        let relative_path = path
            .strip_prefix(&self.config.repo_path)
            .unwrap_or(path)
            .to_path_buf();

        Ok(Some(FileInfo {
            path: path.to_path_buf(),
            relative_path,
            content,
            file_type,
            size: metadata.len(),
        }))
    }

    fn build_project_structure(
        &self,
        files: &[FileInfo],
        total_size: u64,
    ) -> Result<ProjectStructure> {
        let mut tree = String::new();
        let mut paths: Vec<_> = files.iter().map(|f| &f.relative_path).collect();
        paths.sort();

        tree.push_str("```\n");
        for (i, path) in paths.iter().enumerate() {
            let depth = path.components().count() - 1;
            let indent = "│   ".repeat(depth);
            let connector = if i == paths.len() - 1 {
                "└── "
            } else {
                "├── "
            };

            tree.push_str(&format!("{}{}{}\n", indent, connector, path.display()));
        }
        tree.push_str("```\n");

        Ok(ProjectStructure {
            tree,
            total_files: files.len(),
            total_size,
        })
    }

    async fn extract_project_metadata(&self) -> Result<ProjectMetadata> {
        let cargo_toml_path = self.config.repo_path.join("Cargo.toml");
        let readme_path = self.config.repo_path.join("README.md");

        let mut metadata = ProjectMetadata {
            name: self
                .config
                .repo_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            description: None,
            dependencies: Vec::new(),
            rust_version: None,
        };

        // Extract information from Cargo.toml
        if cargo_toml_path.exists() {
            let cargo_content = fs::read_to_string(&cargo_toml_path)?;
            self.parse_cargo_toml(&cargo_content, &mut metadata)?;
        }

        // Extract description from README.md
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            metadata.description = self.extract_description_from_readme(&readme_content);
        }

        Ok(metadata)
    }

    fn parse_cargo_toml(&self, content: &str, metadata: &mut ProjectMetadata) -> Result<()> {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_package = false;
        let mut in_dependencies = false;

        for line in lines {
            let line = line.trim();

            if line.starts_with("[package]") {
                in_package = true;
                in_dependencies = false;
                continue;
            }

            if line.starts_with("[dependencies") {
                in_package = false;
                in_dependencies = true;
                continue;
            }

            if line.starts_with("[") {
                in_package = false;
                in_dependencies = false;
                continue;
            }

            if in_package
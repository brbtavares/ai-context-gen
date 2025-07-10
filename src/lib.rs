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

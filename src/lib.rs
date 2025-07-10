//! # AI Context Generator
//!
//! A Rust library for generating structured context from code repositories,
//! especially useful for LLMs and AI agents.
//!
//! ## Features
//!
//! - 🔍 **Complete Scanning**: Analyzes all `.rs` and `.md` files in the repository
//! - 🌳 **Abstract Syntax Tree**: Extracts and documents structures, functions, enums and implementations
//! - 📊 **Token Control**: Respects token limits and prioritizes important content
//! - 📁 **Project Structure**: Generates file tree visualization
//! - 📖 **Documentation**: Includes markdown files like README, documentation, etc.
//! - ⚡ **Performance**: Asynchronous and optimized processing
//!
//! ## Usage Example
//!
//! ```rust
//! use ai_context_gen::{Config, ContextGenerator, RepositoryScanner};
//! use std::path::PathBuf;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config {
//!     repo_path: PathBuf::from("."),
//!     max_tokens: 50000,
//!     output_file: "repo_context.md".to_string(),
//!     include_hidden: false,
//!     include_deps: false,
//! };
//!
//! let scanner = RepositoryScanner::new(config.clone());
//! let scan_result = scanner.scan().await?;
//!
//! let generator = ContextGenerator::new(config);
//! generator.generate_context(scan_result).await?;
//! # Ok(())
//! # }
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
pub use scanner::{RepositoryScanner, ScanResult, FileInfo, FileType};
pub use parser::{RustParser, RustAnalysis, FunctionInfo, StructInfo, EnumInfo, ImplInfo};
pub use token_counter::{TokenCounter, ContentPrioritizer, ContentSection};

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

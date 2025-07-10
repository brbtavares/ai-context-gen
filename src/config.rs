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

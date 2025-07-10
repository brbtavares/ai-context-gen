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

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
        println!("  - {} ({:?})", file.relative_path.display(), file.file_type);
    }

    // Step 2: Context generation
    println!("\n🏗️  Step 2: Generating context...");
    let generator = ContextGenerator::new(config);
    generator.generate_context(scan_result).await?;

    println!("✅ Advanced context generated successfully!");
    println!("📖 Check the 'advanced_context.md' file");

    Ok(())
}

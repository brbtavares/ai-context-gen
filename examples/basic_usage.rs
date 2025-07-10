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

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use colored::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContextEntry {
    timestamp: DateTime<Utc>,
    title: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ContextWindow {
    entries: Vec<ContextEntry>,
    current_project: Option<String>,
}

impl ContextWindow {
    fn new() -> Self {
        Self::default()
    }

    fn add_entry(&mut self, title: String, content: String, tags: Vec<String>) {
        let entry = ContextEntry {
            timestamp: Utc::now(),
            title,
            content,
            tags,
        };
        self.entries.push(entry);
    }

    fn get_context_file_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".ia-context-gen");
        path.push("context.json");
        path
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_context_file_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_context_file_path();
        
        if !path.exists() {
            return Ok(Self::new());
        }
        
        let json = fs::read_to_string(path)?;
        let context = serde_json::from_str(&json)?;
        Ok(context)
    }

    fn display_entries(&self) {
        if self.entries.is_empty() {
            println!("{}", "📝 No context entries found.".yellow());
            return;
        }

        println!("\n{}", "🔍 Context Window:".green().bold());
        println!("{}", "=".repeat(60).blue());
        
        for (i, entry) in self.entries.iter().enumerate() {
            println!("\n{} [{:02}] {}", "📌".cyan(), i + 1, entry.title.bold());
            println!("{} {}", "📅".cyan(), entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
            
            if !entry.tags.is_empty() {
                println!("{} Tags: {}", "🏷️".cyan(), entry.tags.join(", ").purple());
            }
            
            // Display only the first 3 lines of content
            let content_lines: Vec<&str> = entry.content.lines().collect();
            for (line_idx, line) in content_lines.iter().enumerate() {
                if line_idx < 3 {
                    println!("   {}", line);
                } else if line_idx == 3 && content_lines.len() > 3 {
                    println!("   {}", format!("... ({} more lines)", content_lines.len() - 3).dimmed());
                    break;
                }
            }
            
            println!("{}", "-".repeat(40).blue());
        }
    }

    fn search_entries(&self, query: &str) -> Vec<&ContextEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.title.to_lowercase().contains(&query.to_lowercase())
                    || entry.content.to_lowercase().contains(&query.to_lowercase())
                    || entry.tags.iter().any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect()
    }

    fn clear_all(&mut self) {
        self.entries.clear();
    }
}

fn get_input(prompt: &str) -> String {
    print!("{} ", prompt.cyan());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(_) => {
            println!("{}", "❌ Error reading input. Exiting application.".red());
            std::process::exit(1);
        }
    }
}

fn get_multiline_input(prompt: &str) -> String {
    println!("{}", prompt.cyan());
    println!("{}", "Type 'END' on a separate line to finish:".dimmed());
    
    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "END" {
            break;
        }
        content.push_str(&line);
    }
    content
}

fn display_menu() {
    println!("\n{}", "Select an option:".green().bold());
    println!("{}", "1. 📝 Add new context entry".white());
    println!("{}", "2. 👀 View all entries".white());
    println!("{}", "3. 🔍 Search entries".white());
    println!("{}", "4. 🗑️  Clear all entries".white());
    println!("{}", "5. 📊 Generate context summary".white());
    println!("{}", "6. 🚪 Exit".white());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure Ctrl+C handler
    ctrlc::set_handler(|| {
        println!("\n{}", "👋 Application interrupted by user. Exiting...".yellow());
        std::process::exit(0);
    })?;

    let mut context_window = ContextWindow::load().unwrap_or_else(|_| ContextWindow::new());

    println!("{}", "🚀 AI Context Generator - Context Window".green().bold());
    println!("{}", "=".repeat(50).blue());

    loop {
        display_menu();
        
        let choice = get_input("Enter your choice (1-6):");
        
        match choice.as_str() {
            "1" => {
                // Add new entry
                let title = get_input("Entry title:");
                let content = get_multiline_input("📝 Enter the entry content:");
                let tags_input = get_input("Tags (separated by comma):");
                
                let tags: Vec<String> = tags_input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                context_window.add_entry(title, content, tags);
                context_window.save()?;
                println!("{}", "✅ Entry added successfully!".green());
            }
            "2" => {
                // View all entries
                context_window.display_entries();
            }
            "3" => {
                // Search entries
                let query = get_input("Enter search term:");
                let results = context_window.search_entries(&query);
                
                if results.is_empty() {
                    println!("{}", format!("❌ No entries found for '{}'", query).red());
                } else {
                    println!("\n{}", format!("🔍 Search results for '{}':", query).green().bold());
                    println!("{}", "=".repeat(60).blue());
                    for entry in results {
                        println!("\n{} {}", "📌".cyan(), entry.title.bold());
                        println!("{} {}", "📅".cyan(), entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
                        if !entry.tags.is_empty() {
                            println!("{} Tags: {}", "🏷️".cyan(), entry.tags.join(", ").purple());
                        }
                        println!("{}", entry.content);
                        println!("{}", "-".repeat(40).blue());
                    }
                }
            }
            "4" => {
                // Clear all entries
                let confirm = get_input("Are you sure you want to clear all entries? (y/N):");
                if confirm.to_lowercase() == "y" || confirm.to_lowercase() == "yes" {
                    context_window.clear_all();
                    context_window.save()?;
                    println!("{}", "🗑️  All entries have been removed!".yellow());
                } else {
                    println!("{}", "❌ Operation cancelled.".red());
                }
            }
            "5" => {
                // Generate context summary
                if context_window.entries.is_empty() {
                    println!("{}", "📝 No entries to generate summary.".yellow());
                } else {
                    println!("\n{}", "📊 Context Summary:".green().bold());
                    println!("{}", "=".repeat(40).blue());
                    println!("{} Total entries: {}", "📈".cyan(), context_window.entries.len().to_string().bold());
                    
                    // Count tags
                    let mut tag_counts = std::collections::HashMap::new();
                    for entry in &context_window.entries {
                        for tag in &entry.tags {
                            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                        }
                    }
                    
                    if !tag_counts.is_empty() {
                        println!("{} Most used tags:", "🏷️".cyan());
                        let mut sorted_tags: Vec<_> = tag_counts.iter().collect();
                        sorted_tags.sort_by(|a, b| b.1.cmp(a.1));
                        for (tag, count) in sorted_tags.iter().take(5) {
                            println!("   - {}: {} times", tag.purple(), count.to_string().bold());
                        }
                    }
                    
                    // Most recent entry
                    if let Some(latest) = context_window.entries.last() {
                        println!("{} Most recent entry: {}", "📅".cyan(), latest.title.bold());
                        println!("   Date: {}", latest.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
                    }
                }
            }
            "6" => {
                // Exit
                println!("{}", "👋 Thank you for using AI Context Generator!".green().bold());
                println!("{}", "💾 Context saved automatically.".dimmed());
                return Ok(());
            }
            _ => {
                println!("{}", "❌ Invalid option. Please try again.".red());
            }
        }

        println!("\n{}", "=".repeat(50).blue());
    }
}

use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
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
        self.save_to_path(&path)
    }

    fn save_to_path(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

    fn scan_project(&mut self, project_path: &Path) -> Result<(usize, PathBuf), Box<dyn std::error::Error>> {
        let mut files_processed = 0;
        
        // Clear existing entries for fresh scan
        self.entries.clear();
        
        // Set current project
        if let Some(project_name) = project_path.file_name() {
            self.current_project = Some(project_name.to_string_lossy().to_string());
        }
        
        println!("{}", "🔍 Scanning project files...".cyan());
        
        // Scan different types of files
        self.scan_directory(project_path, &mut files_processed)?;
        
        // Add project overview entry
        self.add_project_overview(project_path)?;
        
        // Save to the project directory instead of home
        let context_file = project_path.join("ai-context.json");
        self.save_to_path(&context_file)?;
        
        Ok((files_processed, context_file))
    }
    
    fn scan_directory(&mut self, dir: &Path, files_processed: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
        if dir.is_dir() {
            // Skip common directories that shouldn't be scanned
            if let Some(dir_name) = dir.file_name() {
                let dir_str = dir_name.to_string_lossy().to_lowercase();
                if ["target", "node_modules", ".git", "dist", "build", ".next", 
                    "__pycache__", ".vscode", ".idea", "coverage"].contains(&dir_str.as_str()) {
                    return Ok(());
                }
            }
            
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.scan_directory(&path, files_processed)?;
                } else {
                    self.process_file(&path, files_processed)?;
                }
            }
        }
        Ok(())
    }
    
    fn process_file(&mut self, file_path: &Path, files_processed: &mut usize) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(extension) = file_path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            
            // Only process relevant file types
            if self.should_process_file(&ext) {
                if let Ok(content) = fs::read_to_string(file_path) {
                    // Skip empty files or very large files
                    if content.trim().is_empty() || content.len() > 50_000 {
                        return Ok(());
                    }
                    
                    let relative_path = file_path.to_string_lossy();
                    let title = format!("📄 {}", relative_path);
                    
                    // Truncate content if too long for context
                    let processed_content = if content.len() > 2000 {
                        let lines: Vec<&str> = content.lines().collect();
                        let preview_lines = lines.iter().take(30).cloned().collect::<Vec<_>>().join("\n");
                        format!("{}\n\n... (file truncated, {} total lines)", preview_lines, lines.len())
                    } else {
                        content
                    };
                    
                    let tags = self.generate_tags_for_file(file_path, &ext);
                    
                    self.add_entry(title, processed_content, tags);
                    *files_processed += 1;
                    
                    if *files_processed % 10 == 0 {
                        print!(".");
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }
        Ok(())
    }
    
    fn should_process_file(&self, extension: &str) -> bool {
        matches!(extension, 
            "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "java" | "cpp" | "c" | "h" |
            "go" | "php" | "rb" | "swift" | "kt" | "dart" | "scala" | "clj" |
            "html" | "css" | "scss" | "sass" | "vue" | "svelte" |
            "json" | "yaml" | "yml" | "toml" | "xml" | "ini" | "cfg" |
            "md" | "txt" | "rst" | "adoc" |
            "sql" | "sh" | "bat" | "ps1" | "dockerfile" | "makefile"
        )
    }
    
    fn generate_tags_for_file(&self, file_path: &Path, extension: &str) -> Vec<String> {
        let mut tags = vec![extension.to_string()];
        
        // Add tags based on file path
        let path_str = file_path.to_string_lossy().to_lowercase();
        
        if path_str.contains("test") || path_str.contains("spec") {
            tags.push("test".to_string());
        }
        if path_str.contains("config") || path_str.contains("settings") {
            tags.push("config".to_string());
        }
        if path_str.contains("src") || path_str.contains("lib") {
            tags.push("source".to_string());
        }
        if path_str.contains("doc") || path_str.contains("readme") {
            tags.push("documentation".to_string());
        }
        if path_str.contains("component") {
            tags.push("component".to_string());
        }
        if path_str.contains("util") || path_str.contains("helper") {
            tags.push("utility".to_string());
        }
        if path_str.contains("api") || path_str.contains("service") {
            tags.push("api".to_string());
        }
        
        // Add language-specific tags
        match extension {
            "rs" => tags.push("rust".to_string()),
            "py" => tags.push("python".to_string()),
            "js" | "jsx" => tags.push("javascript".to_string()),
            "ts" | "tsx" => tags.push("typescript".to_string()),
            "java" => tags.push("java".to_string()),
            "go" => tags.push("golang".to_string()),
            "php" => tags.push("php".to_string()),
            "rb" => tags.push("ruby".to_string()),
            "swift" => tags.push("swift".to_string()),
            "kt" => tags.push("kotlin".to_string()),
            "dart" => tags.push("dart".to_string()),
            "html" => tags.push("html".to_string()),
            "css" | "scss" | "sass" => tags.push("css".to_string()),
            "vue" => tags.push("vue".to_string()),
            "svelte" => tags.push("svelte".to_string()),
            "json" | "yaml" | "yml" | "toml" => tags.push("config".to_string()),
            "md" => tags.push("markdown".to_string()),
            "sql" => tags.push("database".to_string()),
            "dockerfile" => tags.push("docker".to_string()),
            _ => {}
        }
        
        tags.push("auto-scanned".to_string());
        tags
    }
    
    fn add_project_overview(&mut self, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut overview = String::new();
        overview.push_str("🚀 PROJECT OVERVIEW\n");
        overview.push_str("==================\n\n");
        
        if let Some(project_name) = project_path.file_name() {
            overview.push_str(&format!("Project: {}\n", project_name.to_string_lossy()));
        }
        overview.push_str(&format!("Path: {}\n", project_path.display()));
        overview.push_str(&format!("Scanned at: {}\n\n", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        // Detect project type
        if project_path.join("Cargo.toml").exists() {
            overview.push_str("📦 Project Type: Rust (Cargo)\n");
            if let Ok(cargo_content) = fs::read_to_string(project_path.join("Cargo.toml")) {
                overview.push_str("\nCargo.toml dependencies:\n");
                for line in cargo_content.lines().take(20) {
                    if line.contains("=") && !line.starts_with('#') {
                        overview.push_str(&format!("  {}\n", line.trim()));
                    }
                }
            }
        } else if project_path.join("package.json").exists() {
            overview.push_str("📦 Project Type: Node.js/JavaScript\n");
            if let Ok(pkg_content) = fs::read_to_string(project_path.join("package.json")) {
                overview.push_str("\npackage.json info:\n");
                overview.push_str(&pkg_content.lines().take(10).collect::<Vec<_>>().join("\n"));
            }
        } else if project_path.join("requirements.txt").exists() || project_path.join("pyproject.toml").exists() {
            overview.push_str("📦 Project Type: Python\n");
        } else if project_path.join("pom.xml").exists() {
            overview.push_str("📦 Project Type: Java (Maven)\n");
        } else if project_path.join("build.gradle").exists() {
            overview.push_str("📦 Project Type: Java/Kotlin (Gradle)\n");
        } else if project_path.join("go.mod").exists() {
            overview.push_str("📦 Project Type: Go\n");
        }
        
        // Count files by type
        overview.push_str("\n📊 File Statistics:\n");
        let mut file_counts = std::collections::HashMap::new();
        self.count_files_by_type(project_path, &mut file_counts)?;
        
        for (ext, count) in file_counts.iter() {
            if *count > 0 {
                overview.push_str(&format!("  .{}: {} files\n", ext, count));
            }
        }
        
        self.add_entry("🏠 Project Overview".to_string(), overview, vec!["overview".to_string(), "project".to_string(), "summary".to_string()]);
        Ok(())
    }
    
    fn count_files_by_type(&self, dir: &Path, counts: &mut std::collections::HashMap<String, usize>) -> Result<(), Box<dyn std::error::Error>> {
        if dir.is_dir() {
            // Skip directories we don't want to count
            if let Some(dir_name) = dir.file_name() {
                let dir_str = dir_name.to_string_lossy().to_lowercase();
                if ["target", "node_modules", ".git", "dist", "build"].contains(&dir_str.as_str()) {
                    return Ok(());
                }
            }
            
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    self.count_files_by_type(&path, counts)?;
                } else if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    *counts.entry(ext).or_insert(0) += 1;
                }
            }
        }
        Ok(())
    }

    // ...existing code...
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
    println!("{}", "6. 🤖 Auto-scan project (create context from scratch)".white());
    println!("{}", "7. 🚪 Exit".white());
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
        
        let choice = get_input("Enter your choice (1-7):");
        
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
                // Auto-scan project
                let current_dir = std::env::current_dir()?;
                let project_path = get_input(&format!("Enter project path (default: {}):", current_dir.display()));
                
                let scan_path = if project_path.is_empty() {
                    current_dir
                } else {
                    PathBuf::from(project_path)
                };
                
                if !scan_path.exists() {
                    println!("{}", "❌ Path does not exist!".red());
                    continue;
                }
                
                println!("{}", format!("🤖 Starting auto-scan of: {}", scan_path.display()).cyan());
                println!("{}", "This will replace all existing context entries. Continue? (y/N):".yellow());
                
                let confirm = get_input("");
                if confirm.to_lowercase() != "y" && confirm.to_lowercase() != "yes" {
                    println!("{}", "❌ Scan cancelled.".red());
                    continue;
                }
                
                match context_window.scan_project(&scan_path) {
                    Ok((files_processed, context_file)) => {
                        println!("\n{}", "✅ Auto-scan completed successfully!".green());
                        println!("{}", format!("📊 Processed {} files", files_processed).cyan());
                        println!("{}", format!("📝 Created {} context entries", context_window.entries.len()).cyan());
                        println!("{}", format!("💾 Context saved to: {}", context_file.display()).green().bold());
                    }
                    Err(e) => {
                        println!("{}", format!("❌ Error during scan: {}", e).red());
                    }
                }
            }
            "7" => {
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

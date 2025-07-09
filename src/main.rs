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
            println!("{}", "📝 Nenhuma entrada de contexto encontrada.".yellow());
            return;
        }

        println!("\n{}", "🔍 Janela de Contexto:".green().bold());
        println!("{}", "=".repeat(60).blue());
        
        for (i, entry) in self.entries.iter().enumerate() {
            println!("\n{} [{:02}] {}", "📌".cyan(), i + 1, entry.title.bold());
            println!("{} {}", "📅".cyan(), entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
            
            if !entry.tags.is_empty() {
                println!("{} Tags: {}", "🏷️".cyan(), entry.tags.join(", ").purple());
            }
            
            // Exibir apenas as primeiras 3 linhas do conteúdo
            let content_lines: Vec<&str> = entry.content.lines().collect();
            for (line_idx, line) in content_lines.iter().enumerate() {
                if line_idx < 3 {
                    println!("   {}", line);
                } else if line_idx == 3 && content_lines.len() > 3 {
                    println!("   {}", format!("... (mais {} linhas)", content_lines.len() - 3).dimmed());
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
            println!("{}", "❌ Erro ao ler entrada. Encerrando aplicação.".red());
            std::process::exit(1);
        }
    }
}

fn get_multiline_input(prompt: &str) -> String {
    println!("{}", prompt.cyan());
    println!("{}", "Digite 'FIM' em uma linha separada para finalizar:".dimmed());
    
    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "FIM" {
            break;
        }
        content.push_str(&line);
    }
    content
}

fn display_menu() {
    println!("\n{}", "Selecione uma opção:".green().bold());
    println!("{}", "1. 📝 Adicionar nova entrada de contexto".white());
    println!("{}", "2. 👀 Visualizar todas as entradas".white());
    println!("{}", "3. 🔍 Buscar entradas".white());
    println!("{}", "4. 🗑️  Limpar todas as entradas".white());
    println!("{}", "5. 📊 Gerar resumo do contexto".white());
    println!("{}", "6. 🚪 Sair".white());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar handler para Ctrl+C
    ctrlc::set_handler(|| {
        println!("\n{}", "👋 Aplicação interrompida pelo usuário. Saindo...".yellow());
        std::process::exit(0);
    })?;

    let mut context_window = ContextWindow::load().unwrap_or_else(|_| ContextWindow::new());

    println!("{}", "🚀 IA Context Generator - Janela de Contexto".green().bold());
    println!("{}", "=".repeat(50).blue());

    loop {
        display_menu();
        
        let choice = get_input("Digite sua escolha (1-6):");
        
        match choice.as_str() {
            "1" => {
                // Adicionar nova entrada
                let title = get_input("Título da entrada:");
                let content = get_multiline_input("📝 Digite o conteúdo da entrada:");
                let tags_input = get_input("Tags (separadas por vírgula):");
                
                let tags: Vec<String> = tags_input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                context_window.add_entry(title, content, tags);
                context_window.save()?;
                println!("{}", "✅ Entrada adicionada com sucesso!".green());
            }
            "2" => {
                // Visualizar todas as entradas
                context_window.display_entries();
            }
            "3" => {
                // Buscar entradas
                let query = get_input("Digite o termo de busca:");
                let results = context_window.search_entries(&query);
                
                if results.is_empty() {
                    println!("{}", format!("❌ Nenhuma entrada encontrada para '{}'", query).red());
                } else {
                    println!("\n{}", format!("🔍 Resultados da busca para '{}':", query).green().bold());
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
                // Limpar todas as entradas
                let confirm = get_input("Tem certeza que deseja limpar todas as entradas? (s/N):");
                if confirm.to_lowercase() == "s" || confirm.to_lowercase() == "sim" {
                    context_window.clear_all();
                    context_window.save()?;
                    println!("{}", "🗑️  Todas as entradas foram removidas!".yellow());
                } else {
                    println!("{}", "❌ Operação cancelada.".red());
                }
            }
            "5" => {
                // Gerar resumo do contexto
                if context_window.entries.is_empty() {
                    println!("{}", "📝 Nenhuma entrada para gerar resumo.".yellow());
                } else {
                    println!("\n{}", "📊 Resumo do Contexto:".green().bold());
                    println!("{}", "=".repeat(40).blue());
                    println!("{} Total de entradas: {}", "📈".cyan(), context_window.entries.len().to_string().bold());
                    
                    // Contar tags
                    let mut tag_counts = std::collections::HashMap::new();
                    for entry in &context_window.entries {
                        for tag in &entry.tags {
                            *tag_counts.entry(tag.clone()).or_insert(0) += 1;
                        }
                    }
                    
                    if !tag_counts.is_empty() {
                        println!("{} Tags mais usadas:", "🏷️".cyan());
                        let mut sorted_tags: Vec<_> = tag_counts.iter().collect();
                        sorted_tags.sort_by(|a, b| b.1.cmp(a.1));
                        for (tag, count) in sorted_tags.iter().take(5) {
                            println!("   - {}: {} vezes", tag.purple(), count.to_string().bold());
                        }
                    }
                    
                    // Entrada mais recente
                    if let Some(latest) = context_window.entries.last() {
                        println!("{} Entrada mais recente: {}", "📅".cyan(), latest.title.bold());
                        println!("   Data: {}", latest.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string().dimmed());
                    }
                }
            }
            "6" => {
                // Sair
                println!("{}", "👋 Obrigado por usar o IA Context Generator!".green().bold());
                println!("{}", "💾 Contexto salvo automaticamente.".dimmed());
                return Ok(());
            }
            _ => {
                println!("{}", "❌ Opção inválida. Tente novamente.".red());
            }
        }

        println!("\n{}", "=".repeat(50).blue());
    }
}

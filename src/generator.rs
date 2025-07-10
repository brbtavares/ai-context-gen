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
            content.push_str(&format!("**Description:** {description}\n"));
        }

        if !scan_result.metadata.dependencies.is_empty() {
            content.push_str("**Dependencies:**\n");
            for dep in &scan_result.metadata.dependencies {
                content.push_str(&format!("- {dep}\n"));
            }
        }

        if let Some(rust_version) = &scan_result.metadata.rust_version {
            content.push_str(&format!("**Version:** {rust_version}\n"));
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

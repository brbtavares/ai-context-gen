//! Repository scanning module for the AI Context Generator.
//!
//! This module provides functionality to scan and analyze repository structure,
//! extracting metadata, file information, and project organization.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{Config, IGNORED_DIRS, IGNORED_FILES, SUPPORTED_EXTENSIONS};

/// Information about a single file in the repository.
///
/// Contains both metadata and content for files that are included in the analysis.
/// This structure is used to pass file information between scanning and generation phases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// Absolute path to the file on the filesystem.
    pub path: PathBuf,
    
    /// Path relative to the repository root.
    /// 
    /// This is used for display purposes in the generated context.
    pub relative_path: PathBuf,
    
    /// Complete content of the file as a string.
    ///
    /// For text files, this contains the entire file content.
    /// Binary files are not processed and won't appear in scan results.
    pub content: String,
    
    /// Type classification of the file based on its extension.
    pub file_type: FileType,
    
    /// Size of the file in bytes.
    pub size: u64,
}

/// Classification of file types supported by the generator.
///
/// Different file types receive different processing and priority levels
/// during context generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    /// Rust source files (`.rs` extension).
    ///
    /// These files receive full AST analysis to extract structural information
    /// about modules, functions, structs, enums, and implementations.
    Rust,
    
    /// Markdown documentation files (`.md` extension).
    ///
    /// These files are included as high-priority documentation content.
    Markdown,
}

/// Complete result of repository scanning operation.
///
/// Contains all information gathered during the scanning phase, including
/// individual files, project structure, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// List of all files that were processed during scanning.
    ///
    /// Only files with supported extensions that passed filtering are included.
    pub files: Vec<FileInfo>,
    
    /// Structural information about the project organization.
    pub project_structure: ProjectStructure,
    
    /// Metadata extracted from project configuration files.
    pub metadata: ProjectMetadata,
}

/// Information about the overall structure and organization of the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStructure {
    /// String representation of the project's file tree.
    ///
    /// This is formatted as a text-based tree structure suitable for
    /// inclusion in markdown documentation.
    pub tree: String,
    
    /// Total number of files that were processed.
    pub total_files: usize,
    
    /// Combined size of all processed files in bytes.
    pub total_size: u64,
}

/// Project metadata extracted from configuration files and repository structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Name of the project.
    ///
    /// Extracted from `Cargo.toml` if available, otherwise derived from
    /// the repository directory name.
    pub name: String,
    
    /// Project description, if available.
    ///
    /// Extracted from `Cargo.toml` description field or README.md content.
    pub description: Option<String>,
    
    /// List of main dependencies.
    ///
    /// Extracted from the `[dependencies]` section of `Cargo.toml`.
    pub dependencies: Vec<String>,
    
    /// Rust version or project version.
    ///
    /// Extracted from `Cargo.toml` version field.
    pub rust_version: Option<String>,
}

/// Repository scanner that processes project files and structure.
///
/// The scanner walks through the repository directory, identifies relevant files,
/// extracts their content, and gathers project metadata.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::{Config, RepositoryScanner};
/// use std::path::PathBuf;
///
/// # async fn example() -> anyhow::Result<()> {
/// let config = Config {
///     repo_path: PathBuf::from("."),
///     max_tokens: 50000,
///     output_file: "context.md".to_string(),
///     include_hidden: false,
///     include_deps: true,
/// };
///
/// let scanner = RepositoryScanner::new(config);
/// let scan_result = scanner.scan().await?;
///
/// println!("Found {} files", scan_result.files.len());
/// # Ok(())
/// # }
/// ```
pub struct RepositoryScanner {
    config: Config,
}

impl RepositoryScanner {
    /// Creates a new repository scanner with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration specifying scanning behavior and output options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Performs a complete scan of the repository.
    ///
    /// This method walks through the repository directory structure, processes
    /// all supported files, extracts project metadata, and builds a comprehensive
    /// scan result.
    ///
    /// # Returns
    ///
    /// A `ScanResult` containing all discovered files, project structure, and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The repository path doesn't exist or isn't accessible
    /// - File system errors occur during scanning
    /// - Files can't be read or parsed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::{Config, RepositoryScanner};
    ///
    /// # async fn example() -> anyhow::Result<()> {
    /// let config = Config::default();
    /// let scanner = RepositoryScanner::new(config);
    /// let result = scanner.scan().await?;
    ///
    /// println!("Scanned {} files", result.files.len());
    /// println!("Total size: {} bytes", result.project_structure.total_size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn scan(&self) -> Result<ScanResult> {
        let mut files = Vec::new();
        let mut total_size = 0u64;

        for entry in WalkDir::new(&self.config.repo_path)
            .into_iter()
            .filter_entry(|e| self.should_include_path(e.path()))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_info) = self.process_file(path).await? {
                    total_size += file_info.size;
                    files.push(file_info);
                }
            }
        }

        let project_structure = self.build_project_structure(&files, total_size)?;
        let metadata = self.extract_project_metadata().await?;

        Ok(ScanResult {
            files,
            project_structure,
            metadata,
        })
    }

    /// Determines whether a path should be included in the scan.
    ///
    /// This method applies filtering rules based on the configuration and
    /// predefined ignore lists to determine if a file or directory should
    /// be processed.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to evaluate for inclusion
    ///
    /// # Returns
    ///
    /// `true` if the path should be included, `false` otherwise
    fn should_include_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Ignore hidden directories if not configured to include them
        if !self.config.include_hidden && path_str.contains("/.") {
            return false;
        }

        // Ignore specific directories
        for ignored_dir in IGNORED_DIRS {
            if path_str.contains(ignored_dir) {
                return false;
            }
        }

        // If it's a file, check if it's supported
        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            // Ignore specific files
            if IGNORED_FILES.contains(&filename.as_ref()) {
                return false;
            }

            // Check extension
            if let Some(ext) = path.extension() {
                let ext_str = format!(".{}", ext.to_string_lossy());
                return SUPPORTED_EXTENSIONS.contains(&ext_str.as_str());
            }

            return false;
        }

        true
    }

    /// Processes a single file and extracts its information.
    ///
    /// Reads the file content, determines its type based on extension,
    /// and creates a `FileInfo` structure with all relevant metadata.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to process
    ///
    /// # Returns
    ///
    /// `Some(FileInfo)` if the file was successfully processed and should be included,
    /// `None` if the file should be skipped
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or metadata cannot be accessed
    async fn process_file(&self, path: &Path) -> Result<Option<FileInfo>> {
        let content = fs::read_to_string(path)?;
        let metadata = fs::metadata(path)?;

        let file_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => FileType::Rust,
            Some("md") => FileType::Markdown,
            _ => return Ok(None),
        };

        let relative_path = path
            .strip_prefix(&self.config.repo_path)
            .unwrap_or(path)
            .to_path_buf();

        Ok(Some(FileInfo {
            path: path.to_path_buf(),
            relative_path,
            content,
            file_type,
            size: metadata.len(),
        }))
    }

    fn build_project_structure(
        &self,
        files: &[FileInfo],
        total_size: u64,
    ) -> Result<ProjectStructure> {
        let mut tree = String::new();
        let mut paths: Vec<_> = files.iter().map(|f| &f.relative_path).collect();
        paths.sort();

        tree.push_str("```\n");
        for (i, path) in paths.iter().enumerate() {
            let depth = path.components().count() - 1;
            let indent = "│   ".repeat(depth);
            let connector = if i == paths.len() - 1 {
                "└── "
            } else {
                "├── "
            };

            tree.push_str(&format!("{}{}{}\n", indent, connector, path.display()));
        }
        tree.push_str("```\n");

        Ok(ProjectStructure {
            tree,
            total_files: files.len(),
            total_size,
        })
    }

    async fn extract_project_metadata(&self) -> Result<ProjectMetadata> {
        let cargo_toml_path = self.config.repo_path.join("Cargo.toml");
        let readme_path = self.config.repo_path.join("README.md");

        let mut metadata = ProjectMetadata {
            name: self
                .config
                .repo_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            description: None,
            dependencies: Vec::new(),
            rust_version: None,
        };

        // Extract information from Cargo.toml
        if cargo_toml_path.exists() {
            let cargo_content = fs::read_to_string(&cargo_toml_path)?;
            self.parse_cargo_toml(&cargo_content, &mut metadata)?;
        }

        // Extract description from README.md
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            metadata.description = self.extract_description_from_readme(&readme_content);
        }

        Ok(metadata)
    }

    fn parse_cargo_toml(&self, content: &str, metadata: &mut ProjectMetadata) -> Result<()> {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_package = false;
        let mut in_dependencies = false;

        for line in lines {
            let line = line.trim();

            if line.starts_with("[package]") {
                in_package = true;
                in_dependencies = false;
                continue;
            }

            if line.starts_with("[dependencies") {
                in_package = false;
                in_dependencies = true;
                continue;
            }

            if line.starts_with("[") {
                in_package = false;
                in_dependencies = false;
                continue;
            }

            if in_package {
                if line.starts_with("name") {
                    if let Some(name) = line.split('=').nth(1) {
                        metadata.name = name.trim().trim_matches('"').to_string();
                    }
                } else if line.starts_with("version") {
                    if let Some(version) = line.split('=').nth(1) {
                        metadata.rust_version = Some(version.trim().trim_matches('"').to_string());
                    }
                }
            }

            if in_dependencies && !line.is_empty() {
                if let Some(dep_name) = line.split('=').next() {
                    metadata.dependencies.push(dep_name.trim().to_string());
                }
            }
        }

        Ok(())
    }

    fn extract_description_from_readme(&self, content: &str) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut description = String::new();

        for line in lines.iter().take(10) {
            if line.starts_with('#') {
                continue;
            }

            if !line.trim().is_empty() {
                description.push_str(line);
                description.push('\n');

                if description.len() > 200 {
                    break;
                }
            }
        }

        if description.trim().is_empty() {
            None
        } else {
            Some(description.trim().to_string())
        }
    }
}

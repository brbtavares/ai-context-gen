//! Token counting and content prioritization module.
//!
//! This module provides functionality for accurate token counting using the GPT-4
//! tokenizer and intelligent content prioritization to fit within token limits.

use anyhow::Result;
use tiktoken_rs::{get_bpe_from_model, CoreBPE};

/// Token counter using the GPT-4 tokenizer for accurate token counting.
///
/// Provides methods for counting tokens in text and truncating content to fit
/// within specified token limits while maintaining text coherence.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::TokenCounter;
///
/// let counter = TokenCounter::new().unwrap();
/// let text = "Hello, world!";
/// let token_count = counter.count_tokens(text);
/// println!("Text has {} tokens", token_count);
/// ```
pub struct TokenCounter {
    bpe: CoreBPE,
}

impl TokenCounter {
    /// Creates a new token counter using the GPT-4 tokenizer.
    ///
    /// # Returns
    ///
    /// A new `TokenCounter` instance configured with the GPT-4 BPE tokenizer.
    ///
    /// # Errors
    ///
    /// Returns an error if the GPT-4 tokenizer model cannot be loaded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        let bpe = get_bpe_from_model("gpt-4")?;
        Ok(Self { bpe })
    }

    /// Counts the number of tokens in the given text.
    ///
    /// Uses the GPT-4 tokenizer to provide accurate token counts that match
    /// what would be used by OpenAI's models and similar systems.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to count tokens for
    ///
    /// # Returns
    ///
    /// The number of tokens in the text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// let count = counter.count_tokens("Hello, world!");
    /// assert!(count > 0);
    /// ```
    pub fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }

    /// Truncates text to fit within a specified token limit.
    ///
    /// Attempts to preserve text coherence by truncating at token boundaries
    /// rather than character boundaries. Falls back to character truncation
    /// if token decoding fails.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to truncate
    /// * `max_tokens` - Maximum number of tokens to include
    ///
    /// # Returns
    ///
    /// The truncated text that fits within the token limit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::TokenCounter;
    ///
    /// let counter = TokenCounter::new().unwrap();
    /// let long_text = "This is a very long text that exceeds the token limit...";
    /// let truncated = counter.truncate_to_token_limit(long_text, 10);
    /// assert!(counter.count_tokens(&truncated) <= 10);
    /// ```
    pub fn truncate_to_token_limit(&self, text: &str, max_tokens: usize) -> String {
        let tokens = self.bpe.encode_with_special_tokens(text);

        if tokens.len() <= max_tokens {
            return text.to_string();
        }

        let truncated_tokens = &tokens[..max_tokens];
        match self.bpe.decode(truncated_tokens.to_vec()) {
            Ok(truncated_text) => truncated_text,
            Err(_) => {
                // Fallback: truncate by characters
                let char_limit = (text.len() * max_tokens) / tokens.len();
                text.chars().take(char_limit).collect()
            }
        }
    }
}

/// Content prioritizer that manages sections based on priority and token limits.
///
/// The prioritizer sorts content sections by priority and ensures the total
/// content fits within specified token limits by truncating lower priority
/// sections when necessary.
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::{ContentPrioritizer, ContentSection};
///
/// let prioritizer = ContentPrioritizer::new().unwrap();
/// let sections = vec![
///     ContentSection::new("High Priority".to_string(), "Content...".to_string(), 10),
///     ContentSection::new("Low Priority".to_string(), "More content...".to_string(), 1),
/// ];
/// let prioritized = prioritizer.prioritize_content(sections, 1000);
/// ```
pub struct ContentPrioritizer {
    token_counter: TokenCounter,
}

impl ContentPrioritizer {
    /// Creates a new content prioritizer.
    ///
    /// # Returns
    ///
    /// A new `ContentPrioritizer` instance with an initialized token counter.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying token counter cannot be initialized.
    pub fn new() -> Result<Self> {
        Ok(Self {
            token_counter: TokenCounter::new()?,
        })
    }

    /// Prioritizes and truncates content sections to fit within token limits.
    ///
    /// Sorts sections by priority (highest first) and includes as many complete
    /// sections as possible. When a section would exceed the token limit, it
    /// attempts to truncate it if there are sufficient remaining tokens.
    ///
    /// # Arguments
    ///
    /// * `sections` - List of content sections to prioritize
    /// * `max_tokens` - Maximum total tokens allowed
    ///
    /// # Returns
    ///
    /// A vector of sections that fit within the token limit, sorted by priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::{ContentPrioritizer, ContentSection};
    ///
    /// let prioritizer = ContentPrioritizer::new().unwrap();
    /// let sections = vec![
    ///     ContentSection::new("Important".to_string(), "Critical info".to_string(), 10),
    ///     ContentSection::new("Less Important".to_string(), "Extra details".to_string(), 5),
    /// ];
    /// let result = prioritizer.prioritize_content(sections, 100);
    /// // Higher priority sections appear first
    /// ```
    pub fn prioritize_content(
        &self,
        sections: Vec<ContentSection>,
        max_tokens: usize,
    ) -> Vec<ContentSection> {
        let mut prioritized = sections;

        // Sort by priority (highest priority first)
        prioritized.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut total_tokens = 0;
        let mut result = Vec::new();

        for mut section in prioritized {
            let section_tokens = self.token_counter.count_tokens(&section.content);

            if total_tokens + section_tokens <= max_tokens {
                total_tokens += section_tokens;
                result.push(section);
            } else {
                // Try to truncate content to fit within the limit
                let remaining_tokens = max_tokens - total_tokens;
                if remaining_tokens > 100 {
                    // Only include if at least 100 tokens remain
                    section.content = self
                        .token_counter
                        .truncate_to_token_limit(&section.content, remaining_tokens);
                    section.truncated = true;
                    result.push(section);
                    break;
                }
            }
        }

        result
    }
}

/// A content section with associated metadata for prioritization.
///
/// Represents a section of content (like project metadata, source code, or
/// documentation) with a title, content, priority level, and truncation status.
///
/// # Priority Levels
///
/// - `9-10`: High priority (metadata, documentation)
/// - `5-6`: Medium priority (AST analysis, structure)
/// - `1-2`: Low priority (source code)
///
/// # Examples
///
/// ```rust
/// use ai_context_gen::token_counter::ContentSection;
///
/// // Create a high-priority section
/// let section = ContentSection::high_priority(
///     "Project Metadata".to_string(),
///     "Important project information...".to_string()
/// );
///
/// // Create a custom priority section
/// let custom = ContentSection::new(
///     "Custom Section".to_string(),
///     "Content here...".to_string(),
///     7
/// );
/// ```
#[derive(Debug, Clone)]
pub struct ContentSection {
    /// Title of the content section.
    pub title: String,

    /// The actual content of the section.
    pub content: String,

    /// Priority level (higher numbers = higher priority).
    pub priority: u8,

    /// Whether this section was truncated to fit token limits.
    pub truncated: bool,
}

impl ContentSection {
    /// Creates a new content section with the specified priority.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    /// * `priority` - Priority level (0-255, higher is more important)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::token_counter::ContentSection;
    ///
    /// let section = ContentSection::new(
    ///     "My Section".to_string(),
    ///     "Section content...".to_string(),
    ///     8
    /// );
    /// ```
    pub fn new(title: String, content: String, priority: u8) -> Self {
        Self {
            title,
            content,
            priority,
            truncated: false,
        }
    }

    /// Creates a high-priority content section (priority 9).
    ///
    /// Use for critical content like project metadata and documentation
    /// that should always be included.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn high_priority(title: String, content: String) -> Self {
        Self::new(title, content, 9)
    }

    /// Creates a medium-priority content section (priority 5).
    ///
    /// Use for structural information like AST analysis and project organization.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn medium_priority(title: String, content: String) -> Self {
        Self::new(title, content, 5)
    }

    /// Creates a low-priority content section (priority 1).
    ///
    /// Use for detailed content like complete source code that can be
    /// truncated if necessary.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the section
    /// * `content` - The content text
    pub fn low_priority(title: String, content: String) -> Self {
        Self::new(title, content, 1)
    }
}

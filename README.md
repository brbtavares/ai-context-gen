# AI Context Generator

A command-line tool in Rust for creating and managing a context window that facilitates AI interactions during project development.

## 🚀 Features

- **📝 Add Entries**: Create new context entries with title, content, and tags
- **👀 View Entries**: Display all saved entries with colored formatting
- **🔍 Search Entries**: Search by title, content, or tags
- **🗑️ Clear Entries**: Remove all entries from the context
- **📊 Context Summary**: Statistics about entries and most used tags
- **🤖 Auto-scan Project**: Automatically scan and create context from project files
- **💾 Persistence**: Data automatically saved to `~/.ia-context-gen/context.json`

## 📋 Prerequisites

- Rust 1.70 or higher
- Cargo

## 🔧 Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd ia-context-gen
```

2. Install using the installation script:
```bash
./install.sh
```

Or compile manually:
```bash
cargo build --release
```

3. Run the application:
```bash
# If installed with the script
ia-context-gen

# Or run directly
cargo run
```

## 🛠️ Development

### Useful commands (via Makefile)
```bash
make dev        # Run in development mode
make release    # Compile for release
make test       # Run tests
make clean      # Clean build files
make fmt        # Format code
make lint       # Check linting
make install    # Install locally
make demo       # Run demonstration
```

### Initial context example
The `example_context.json` file contains example entries that you can use as reference or import to test the application.

## 📖 How to Use

### Run the application
```bash
cargo run
```

### Main Menu
The application presents an interactive menu with the following options:

1. **📝 Add new context entry**
   - Enter the entry title
   - Enter the content (finish with "END" on a separate line)
   - Add tags separated by comma

2. **👀 View all entries**
   - Shows all saved entries
   - Displays only the first 3 lines of content

3. **🔍 Search entries**
   - Search for any term
   - Searches in titles, content, and tags

4. **🗑️ Clear all entries**
   - Removes all entries (requires confirmation)

5. **📊 Generate context summary**
   - Shows context statistics
   - Most used tags
   - Most recent entry

6. **🤖 Auto-scan project (create context from scratch)**
   - Automatically scan project files and create context entries
   - Detects project type and generates comprehensive overview
   - Processes source code, configuration, and documentation files
   - Creates intelligent tags based on file types and locations

7. **🚪 Exit**
   - Closes the application

## 📁 Data Structure

Entries are stored with the following structure:

```rust
struct ContextEntry {
    timestamp: DateTime<Utc>,
    title: String,
    content: String,
    tags: Vec<String>,
}
```

## 🎯 Use Cases

### For Developers
- Save important code snippets
- Document architecture decisions
- Keep record of bugs and solutions
- Create AI prompt templates

### For AI Interactions
- Maintain context from previous conversations
- Save prompts that worked well
- Document experiment results
- Create project knowledge base

## 🤖 Auto-scan Feature

The auto-scan feature (option 6) automatically analyzes your project and creates comprehensive context entries:

### Supported File Types
- **Source Code**: `.rs`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`, `.java`, `.cpp`, `.go`, `.php`, `.rb`, etc.
- **Web Files**: `.html`, `.css`, `.scss`, `.vue`, `.svelte`
- **Configuration**: `.json`, `.yaml`, `.toml`, `.xml`, `.ini`
- **Documentation**: `.md`, `.txt`, `.rst`
- **Scripts**: `.sh`, `.bat`, `Dockerfile`, `Makefile`

### Automatic Project Detection
- **Rust**: Detects `Cargo.toml`, analyzes dependencies
- **JavaScript/Node.js**: Detects `package.json`, analyzes scripts and dependencies
- **Python**: Detects `requirements.txt`, `setup.py`, `pyproject.toml`
- **Java**: Detects `pom.xml` (Maven) or `build.gradle` (Gradle)
- **Go**: Detects `go.mod`
- **PHP**: Detects `composer.json`
- **Ruby**: Detects `Gemfile`

### Intelligent Tagging
Files are automatically tagged based on:
- File extension (language)
- Directory structure (`src/`, `test/`, `config/`, etc.)
- File purpose (components, utilities, services, etc.)
- Project type and framework detection

## 🔮 Usage Example

```
🚀 AI Context Generator - Context Window
==================================================

Select an option:
1. 📝 Add new context entry
2. 👀 View all entries
3. 🔍 Search entries
4. 🗑️  Clear all entries
5. 📊 Generate context summary
6. 🤖 Auto-scan project (create context from scratch)
7. 🚪 Exit

Enter your choice (1-7): 6
Enter project path (default: current directory): 
⚠️ This will replace all existing context entries.
Continue? (y/N): y
🚀 Starting project scan...
🔍 Scanning project files...
📄 Processed 10 files...
📄 Processed 20 files...

✅ Project scan completed successfully!
📊 25 files processed
💾 Context saved to: /home/user/.ia-context-gen/context.json

📋 Context entries created:
  • Project overview
  • 25 source files

💡 Use option 2 to view all entries or option 3 to search!
```

## 🛠️ Dependencies

- `serde` - JSON serialization/deserialization
- `chrono` - Date and time manipulation
- `dirs` - System directory retrieval
- `colored` - Terminal output coloring
- `crossterm` - Terminal manipulation

## 📝 Development

This project was developed specifically to assist in project development, providing a quick and efficient way to maintain context during AI interactions.

### Compile for release
```bash
cargo build --release
```

### Run tests
```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the project
2. Create a branch for your feature
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

This project is under the MIT license - see the LICENSE file for details.

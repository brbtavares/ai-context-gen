# AI Context Generator

A command-line tool in Rust for creating and managing a context window that facilitates AI interactions during project development.

## 🚀 Features

- **📝 Add Entries**: Create new context entries with title, content, and tags
- **👀 View Entries**: Display all saved entries with colored formatting
- **🔍 Search Entries**: Search by title, content, or tags
- **🗑️ Clear Entries**: Remove all entries from the context
- **📊 Context Summary**: Statistics about entries and most used tags
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

6. **🚪 Exit**
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
6. 🚪 Exit

Enter your choice (1-6): 1
Entry title: Rust Configuration
📝 Enter the entry content:
Type 'END' on a separate line to finish:
To configure a new Rust project:
1. cargo new project
2. cd project
3. cargo run
END
Tags (separated by comma): rust, setup, cargo
✅ Entry added successfully!
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
